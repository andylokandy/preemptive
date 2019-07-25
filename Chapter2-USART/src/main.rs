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
mod usart;

use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m;
use stm32f103xx;
use usart::USART;

#[no_mangle]
#[inline(never)]
fn main() -> ! {
    let mut dp = stm32f103xx::Peripherals::take().unwrap();

    // initialize resourses
    rcc::rcc_clock_init(&mut dp.RCC, &mut dp.FLASH);
    usart::usart_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART2);
    led::led_init(&mut dp.RCC, &mut dp.GPIOB);

    // light up
    led::set(true);

    writeln!(USART, "Kernel started!").unwrap();

    for n in 0.. {
        writeln!(USART, "Counting: {}", n).unwrap();
        delay();
    }

    unreachable!();
}

pub fn delay() {
    for _ in 0..20000000 {
        cortex_m::asm::nop();
    }
}

/// Prints panic information via serial interface
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    write!(USART, "{}", info.message().unwrap()).unwrap();

    loop {}
}
