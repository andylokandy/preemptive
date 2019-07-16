pub fn led_init(rcc: &mut stm32f103xx::RCC, gpiob: &mut stm32f103xx::GPIOB) {
    // enable gpiob
    rcc.apb2enr.write(|w| w.iopben().enabled());

    // configurate PB12 as push-pull output
    gpiob.crh.write(|w| w.mode12().output50().cnf12().push());
}

pub fn set(on: bool) {
    let gpiob = unsafe { &*stm32f103xx::GPIOB::ptr() };
    if on {
        gpiob.bsrr.write(|w| w.br12().reset());
    } else {
        gpiob.bsrr.write(|w| w.bs12().set());
    }
}
