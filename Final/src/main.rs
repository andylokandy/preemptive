#![no_std]
#![no_main]
#![feature(asm, const_fn, naked_functions, core_intrinsics, panic_info_message)]

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
#[inline(never)]
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

    writeln!(USART, "Kernel started!").unwrap();

    // main dispatch loop
    loop {
        writeln!(USART, "\nEntering task1!").unwrap();
        process_task1.switch_to_task();
        writeln!(USART, "\nEntering task2!").unwrap();
        process_task2.switch_to_task();
    }
}

#[no_mangle]
fn task1() -> ! {
    let mut n = 0;
    loop {
        writeln!(USART, "fib({})={}", n, fib(n)).unwrap();
        n += 1;
    }
}

#[no_mangle]
fn task2() -> ! {
    let mut n = 0;
    loop {
        writeln!(USART, "is_prime({})={}", n, is_prime(n)).unwrap();
        n += 1;
    }
}

#[no_mangle]
fn fib(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    let mut result = 0;

    for _ in 3..n + 1 {
        result = a + b;
        a = b;
        b = result;
    }

    result
}

#[no_mangle]
fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        true
    } else if n % 6 != 1 && n % 6 != 5 {
        false
    } else {
        let n_sqrt = (1..).take_while(|x| x * x <= n).last().unwrap();
        (5..n_sqrt + 1)
            .step_by(6)
            .all(|x| n % x != 0 && n % (x + 2) != 0)
    }
}
