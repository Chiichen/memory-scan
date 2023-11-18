mod analyzer;

use std::time::Duration;

use aya::maps::HashMap;
use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Bpf};
use aya_log::BpfLogger;
use log::{debug, info, warn};
use tokio::{signal, time};

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

    let mut address_map: HashMap<_, i64, i64> = HashMap::try_from(bpf.map_mut("MAP").unwrap())?;

    let memory_hot_map: MemoryHotMap<i64, i64> = MemoryHotMap::new();
    memory_hot_map.take_from_bpfmap(address_map);
    // address_map.pin("/sys/fs/bpf/memory-scan");
    info!("Waiting for Ctrl-C...");
    // Periodically execute memory_hot_map.take_from_bpfmap(address_map)
    loop {
        memory_hot_map.take_from_bpfmap(address_map);

        // Adjust the sleep duration as needed
        time::sleep(Duration::from_secs(10)).await;
    }

    info!("Exiting...");

    Ok(())
}
