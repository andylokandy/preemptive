use crate::switch_context::{svc_handler, systick_handler};
use crate::debug::hard_fault_handler;

extern "C" {
    static mut _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

#[link_section = ".isr_vector"]
#[used]
pub static ISR_VECTOR: [unsafe extern "C" fn(); 15] = [
    reset_handler,
    unhandled_interrupt, // NMI
    hard_fault_handler, // Hard Fault
    unhandled_interrupt2, // MemManage
    unhandled_interrupt2, // BusFault
    unhandled_interrupt2, // UsageFault
    unhandled_interrupt2,
    unhandled_interrupt2,
    unhandled_interrupt2,
    unhandled_interrupt2,
    svc_handler,          // SVC
    unhandled_interrupt2, // DebugMon
    unhandled_interrupt2,
    unhandled_interrupt2, // PendSV
    systick_handler,     // SysTick
];

unsafe extern "C" fn unhandled_interrupt() {
    loop {}
}

unsafe extern "C" fn unhandled_interrupt2() {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn reset_handler() {
    init_data(&mut _sidata, &mut _sdata, &mut _edata);
    zero_bss(&mut _sbss, &mut _ebss);

    crate::main();
}

unsafe fn init_data(mut sidata: *const u32, mut sdata: *mut u32, edata: *mut u32) {
    while sdata < edata {
        sdata.write(sidata.read());
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }
}

unsafe fn zero_bss(mut sbss: *mut u32, ebss: *mut u32) {
    while sbss < ebss {
        sbss.write_volatile(0);
        sbss = sbss.offset(1);
    }
}
