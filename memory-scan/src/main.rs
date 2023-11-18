mod analyzer;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use aya::maps::HashMap;
use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Bpf};
use aya_log::BpfLogger;
use log::{debug, info, warn};
use tokio::time;

use crate::analyzer::MemoryHotMap;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/memory-scan"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/memory-scan"
    ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut TracePoint = bpf.program_mut("memory_scan").unwrap().try_into()?;
    program.load()?;
    program.attach("exceptions", "page_fault_user")?;

    let temp: HashMap<_, i64, i64> = HashMap::try_from(bpf.map_mut("MAP").unwrap())?;

    let address_map = Arc::new(Mutex::new(temp));

    let memory_hot_map = Arc::new(MemoryHotMap::new());
    // address_map.pin("/sys/fs/bpf/memory-scan");
    info!("Waiting for Ctrl-C...");
    // Spawn a task to listen for the Ctrl+C signal
    let signal = tokio::signal::ctrl_c();
    let memory_hot_map_clone = Arc::clone(&memory_hot_map);
    tokio::spawn(async move {
        signal.await.expect("failed to receive Ctrl+C signal");
        memory_hot_map_clone
            .clone()
            .save_to_file("./map.csv")
            .expect("failed to save hot map to file");
        std::process::exit(0);
    });
    let memory_hot_map_clone = Arc::clone(&memory_hot_map);
    // Periodically execute memory_hot_map.take_from_bpfmap(address_map)
    loop {
        memory_hot_map_clone.take_from_bpfmap(address_map.clone());

        // Adjust the sleep duration as needed
        time::sleep(Duration::from_millis(1000)).await;

        // println!("Memory hotmap :{}", memory_hot_map);
        println!("Memory hotmap size:{}", memory_hot_map.len());
    }
}
