use core::ptr::write_volatile;

pub struct Process {
    stack_ptr: *mut usize,
    states: [usize; 8],
}

impl Process {
    pub unsafe fn new(stack_ptr: *mut usize, callback: fn() -> !) -> Self {
        Self {
            stack_ptr: unsafe { push_function_call(stack_ptr, callback) },
            states: [0; 8],
        }
    }

    pub fn switch_to_user(&mut self) {
        unsafe { self.stack_ptr = switch_to_user(self.stack_ptr, &mut self.states) }
    }
}

pub unsafe fn push_function_call(user_stack: *mut usize, callback: fn() -> !) -> *mut usize {
    let stack_bottom = user_stack.offset(-8);
    write_volatile(stack_bottom.offset(7), 0x01000000); // xPSR
    write_volatile(stack_bottom.offset(6), callback as usize | 1); // PC
    write_volatile(stack_bottom.offset(5), 0 | 0x1); // LR
    write_volatile(stack_bottom.offset(3), 0); // R3
    write_volatile(stack_bottom.offset(2), 0); // R2
    write_volatile(stack_bottom.offset(1), 0); // R1
    write_volatile(stack_bottom.offset(0), 0); // R0

    stack_bottom
}

#[no_mangle]
pub extern "C" fn syscall() {
    unsafe {
        asm!("svc 0x01" :::: "volatile");
    }
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn svc_handler() {
    asm!("
    cmp lr, #0xfffffff9
    bne to_kernel

    movw lr, #0xfffd
    movt lr, #0xffff
    bx lr

    to_kernel:
    movw lr, #0xfff9
    movt lr, #0xffff
    bx lr"
    :::: "volatile" );
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn systick_handler() {
    asm!("
    movw lr, #0xfff9
    movt lr, #0xffff
    bx lr
    "
    :::: "volatile"  );
}

#[no_mangle]
pub unsafe extern "C" fn switch_to_user(
    mut user_stack: *mut usize,
    process_regs: &mut [usize; 8],
) -> *mut usize {
    asm!("
    /* Load bottom of stack into Process Stack Pointer */
    msr psp, $0

    /* Load non-hardware-stacked registers from Process stack */
    /* Ensure that $2 is stored in a callee saved register */
    ldmia $2, {r4-r11}

    /* SWITCH */
    svc 0xff /* It doesn't matter which SVC number we use here */

    /* Push non-hardware-stacked registers into Process struct's */
    /* regs field */
    stmia $2, {r4-r11}

    mrs $0, PSP /* PSP into r0 */
    "
    : "={r0}"(user_stack)
    : "{r0}"(user_stack), "{r1}"(process_regs as *mut _ as *mut _)
    : "r4","r5","r6","r7","r8","r9","r10","r11" : "volatile" );

    user_stack
}
