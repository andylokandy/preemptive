#![no_std]
#![no_main]
#![feature(
    asm,
    const_fn,
    naked_functions,
    core_intrinsics,
    panic_info_message
)]

mod debug;
mod led;
mod rcc;
mod startup;
mod switch_context;
mod systick;
mod usart;

use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m;
use stm32f103xx;
use switch_context::Process;
use usart::USART;

const TASK_STACK_SIZE: usize = 100;
static mut TASK_STACK: [usize; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

#[no_mangle]
#[inline(never)]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f103xx::Peripherals::take().unwrap();

    // initialize task stack
    let stack_pointer = unsafe { TASK_STACK.last_mut().unwrap() as *mut usize };
    let mut process_task = unsafe { Process::new(stack_pointer, task) };

    // initialize resourses
    rcc::rcc_clock_init(&mut dp.RCC, &mut dp.FLASH);
    usart::usart_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART2);
    led::led_init(&mut dp.RCC, &mut dp.GPIOB);
    systick::systick_start(&mut cp.SYST);

    // light up
    led::set(true);

    writeln!(USART, "Kernel started!").unwrap();

    // main dispatch loop
    loop {
        // switch to task
        process_task.switch_to_task();

        // switched back now (when Systick ticks)
        writeln!(USART, "Entering kernel").unwrap();
    }
}

#[no_mangle]
fn task() -> ! {
    let mut n = 0;
    loop {
        writeln!(USART, "Starting task n=({})", n).unwrap();
        n += 1;

        writeln!(USART, "Working").unwrap();
        delay();
        writeln!(USART, "Work is done").unwrap();
    }
}

pub fn delay() {
    for _ in 0..20000000 {
        cortex_m::asm::nop();
    }
}

#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    write!(USART, "{}", info.message().unwrap()).unwrap();

    loop {}
}
