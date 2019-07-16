#![no_std]
#![no_main]
#![feature(
    asm,
    const_fn,
    naked_functions,
    core_intrinsics,
    panic_info_message
)]

mod led;
mod startup;

use core::panic::PanicInfo;
use cortex_m;
use stm32f103xx;

#[no_mangle]
#[inline(never)]
fn main() -> ! {
    let mut dp = stm32f103xx::Peripherals::take().unwrap();

    led::led_init(&mut dp.RCC, &mut dp.GPIOB);

    loop {
        // light on
        led::set(true);

        delay();

        // light off
        led::set(false);

        delay();
    }
}

pub fn delay() {
    for _ in 0..2000 {
        cortex_m::asm::nop();
    }
}

#[panic_handler]
pub unsafe extern "C" fn panic_fmt(_info: &PanicInfo) -> ! {
    loop {}
}
