#![no_std]
#![no_main]
#![feature(
    asm,
    const_fn,
    try_from,
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
use cortex_m;
use stm32f103xx;
use switch_context::Process;
use usart::USART;

const TASK_NUM: usize = 2;
const TASK_STACK_SIZE: usize = 100;
static mut TASK_STACKS: [[usize; TASK_STACK_SIZE]; TASK_NUM] = [[0; TASK_STACK_SIZE]; TASK_NUM];

#[no_mangle]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f103xx::Peripherals::take().unwrap();

    // initialize task stack
    let mut process_task1 =
        unsafe { Process::new(TASK_STACKS[0].last_mut().unwrap() as *mut usize, task1) };

    let mut process_task2 =
        unsafe { Process::new(TASK_STACKS[1].last_mut().unwrap() as *mut usize, task2) };

    // initialize resourses
    rcc::rcc_clock_init(&mut dp.RCC, &mut dp.FLASH);
    usart::usart_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART2);
    led::led_init(&mut dp.RCC, &mut dp.GPIOB);
    systick::systick_start(&mut cp.SYST);

    // light up
    led::set(true);

    writeln!(USART, "Kernel started!");

    // main dispatcher loop
    loop {
        process_task1.switch_to_task();
        process_task2.switch_to_task();
    }
}

#[no_mangle]
fn task1() -> ! {
    loop {
        writeln!(USART, "Executing task1!");
        delay();
    }
}

#[no_mangle]
fn task2() -> ! {
    loop {
        writeln!(USART, "Executing task2!");
        delay();
    }
}

pub fn delay() {
    for _ in 0..1000000 {
        cortex_m::asm::nop();
    }
}
