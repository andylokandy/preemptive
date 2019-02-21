use core::fmt::{Error, Write};
use stm32f103xx;

pub fn usart_init(
    rcc: &mut stm32f103xx::RCC,
    gpioa: &mut stm32f103xx::GPIOA,
    usart2: &mut stm32f103xx::USART2,
) {
    // enable usart and gpioa clocks
    rcc.apb1enr.write(|w| w.usart2en().enabled());
    rcc.apb2enr.write(|w| w.iopaen().enabled());

    // configurate alternative pins : Tx -> PA2, Rx -> PA3
    gpioa.crl.write(|w| {
        w.mode2()
            .output50()
            .cnf2()
            .alt_push()
            .mode3()
            .input()
            .cnf3()
            .alt_push()
    });

    unsafe {
        // configurate usart baudrate
        // USARTDIV = FCLK (PCLK1 for USART2) / baudrate / 16
        //          = 36M / 115200 / 16 = 19.53125
        // DIV_MANTISSA = USARTDIV (integer part)
        //              = 19
        // DIV_FRACTION = USARTDIV (fraction part) * 16
        //              = 0.53125 * 16 = 8.5
        usart2
            .brr
            .write(|w| w.div_mantissa().bits(19).div_fraction().bits(8));

        // enable usart
        usart2
            .cr1
            .write(|w| w.ue().set_bit().te().set_bit().re().set_bit());
    }
}

pub struct USART;

impl Write for USART {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let usart2 = unsafe { &*stm32f103xx::USART2::ptr() };

        for ch in s.bytes() {
            while !usart2.sr.read().txe().bit_is_set() {}
            unsafe {
                usart2.dr.write(|w| w.bits(ch as u32));
            }
        }

        Ok(())
    }
}
