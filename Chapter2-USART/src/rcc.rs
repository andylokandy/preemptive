use stm32f103xx;

/// Initialize system clock
pub fn rcc_clock_init(rcc: &mut stm32f103xx::RCC, flash: &mut stm32f103xx::FLASH) {
    // configurate AHB(HCLK) = SYSCLK, APB1(PCLK1) = SYSCLK / 2, APB2(PCK2) = SYSCLK
    rcc.cfgr
        .write(|w| w.hpre().no_div().ppre1().div2().ppre2().no_div());

    // configurate PLL(phase locked loop) clock : 9 x HSI (8MHz)
    rcc.cfgr.write(|w| w.pllxtpre().no_div().pllmul().mul9());

    // configurate flash : two wait states, prefetch enabled
    flash.acr.write(|w| w.latency().two().prftbe().enabled());

    // start PLL
    rcc.cr.write(|w| w.pllon().enabled());

    // wait till PLL is locked
    while !rcc.cr.read().pllrdy().is_locked() {}

    // swtich SYSCLK to PLL
    rcc.cfgr.write(|w| w.sw().pll());

    // wait till SYSCLK is stable
    while !rcc.cfgr.read().sws().is_pll() {}
}
