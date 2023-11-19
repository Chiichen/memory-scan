#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, tracepoint},
    maps::HashMap,
    programs::TracePointContext,
};
use aya_log_ebpf::error;

// 暂时设置为 u32 最大值
#[map]
static MAP: HashMap<i64, i64> = HashMap::<i64, i64>::with_max_entries(10000000, 0);

#[inline(always)]
fn increase(ctx: &TracePointContext, address: i64) {
    let reader = unsafe { MAP.get(&address) }; // 这里必须要用一个 Reader 来保存 get 的结果，不能写在 if 的条件里，否则会引起 eBPF 验证器报错
    if reader.is_some() {
        let pre = reader.unwrap();
        let cur = (*pre) + 1;
        let r = MAP.insert(&address, &cur, 0);
        if r.is_err() {
            error!(
                ctx,
                "failed to reinsert a new element ({},{}) into map with errno :{}",
                address,
                cur,
                r.unwrap_err()
            );
        }
    } else {
        let r = MAP.insert(&address, &1, 0);
        if r.is_err() {
            error!(
                ctx,
                "failed to insert a new element ({},1) into map with errno :{}",
                address,
                r.unwrap_err()
            );
        }
    }
}

#[tracepoint]
pub fn memory_scan(ctx: TracePointContext) -> u32 {
    match try_memory_scan(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

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
fn try_memory_scan(ctx: TracePointContext) -> Result<u32, u32> {
    // info!(&ctx, "tracepoint page_fault_user called");

    let address = unsafe { ctx.read_at::<i64>(8) };
    let pid = unsafe { ctx.read_at::<i64>(16) };
    if address.is_ok() && pid.is_ok() {
        // info!(
        //     &ctx,
        //     "pid ={} gain an page fault at {}",
        //     pid.unwrap(),
        //     address.unwrap()
        // );
        increase(&ctx, address.unwrap());
    }
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
