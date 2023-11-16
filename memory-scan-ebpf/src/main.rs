#![no_std]
#![no_main]

use aya_bpf::{macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;

#[tracepoint]
pub fn memory_scan(ctx: TracePointContext) -> u32 {
    match try_memory_scan(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_memory_scan(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "tracepoint page_fault_user called");
    //     name: page_fault_kernel
    // ID: 118
    // format:
    //         field:unsigned short common_type;       offset:0;       size:2; signed:0;
    //         field:unsigned char common_flags;       offset:2;       size:1; signed:0;
    //         field:unsigned char common_preempt_count;       offset:3;       size:1; signed:0;
    //         field:int common_pid;   offset:4;       size:4; signed:1;

    //         field:unsigned long address;    offset:8;       size:8; signed:0;
    //         field:unsigned long ip; offset:16;      size:8; signed:0;
    //         field:unsigned long error_code; offset:24;      size:8; signed:0;
    let address = unsafe { ctx.read_at::<i64>(8) };
    let pid = unsafe { ctx.read_at::<i64>(16) };
    if address.is_ok() && pid.is_ok() {
        info!(
            &ctx,
            "pid ={} gain an page fault at {}",
            pid.unwrap(),
            address.unwrap()
        );
    }
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
