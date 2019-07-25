use core::ptr::write_volatile;

pub struct Process {
    stack_ptr: *mut usize,
    // R4 - R11
    states: [usize; 8],
}

impl Process {
    /// Initialize stack frame of task
    pub unsafe fn new(stack_ptr: *mut usize, callback: fn() -> !) -> Self {
        Self {
            stack_ptr: push_function_call(stack_ptr, callback),
            states: [0; 8],
        }
    }

    /// Switch context from kernel to task
    pub fn switch_to_task(&mut self) {
        unsafe { self.stack_ptr = switch_to_task(self.stack_ptr, &mut self.states) }
    }
}

/// Set initial register of the context of task
///
/// The processor will automaticllay load the top 8 words(u32)
/// from the stakc frame of task into register when switching to context.
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

/// Toggle context between kernel and task
///
/// SVC interrupt can only be fired by instruction `svc`.
///
/// SVC handler is an interrupt handler, which means it will
/// be executed in handler mode, and because of that, it could
/// choose the execution context when it returns by loading special
/// EXC_RETURN value into pc register.
///
/// EXC_RETURN varients:
/// - 0xfffffff9 : return to msp (thread mode) - switch to kernel
/// - 0xfffffffd : return to psp (thread mode) - switch to task
/// - 0xfffffff1 : return to msp (handler mode) - return to another interrupt handler
///
/// `msp` means the Main Stack Pointer and
/// `psp` means the Process Stack Pointer.
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

/// Setup task context and switch to it
///
/// This function is doing these few steps:
/// 1. Saves registers {r4-r12, lr} into msp (by complier ABI).
/// 2. Load task stack address into psp.
/// 3. Restore the register states of task from `process_regs` into {r4-r11}.
/// 4. Invoke SVC execption in order to jump into svc_handler,
///    therefore we switched to task context.
/// 5. Saves registers states {r4-r11} into `process_regs`
///    when switched back to kernel (by systick_handler or svc_handler),
/// 6. Restore new psp into `user_stack`.
/// 7. Restore kernel registers states {r4-r12, lr->pc} from msp (by complier ABI).
///
/// The first step and last step is performed by function call ABI convention,
/// so we have to ensure this function is never inlined.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn switch_to_task(
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
