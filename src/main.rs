#![no_std]
#![no_main]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f411 as stm32_pac;

#[entry]
fn main() -> ! {
    let cortex_peri = cortex_m::Peripherals::take().unwrap();
    let st_peri = stm32_pac::Peripherals::take().unwrap();

    let mut syst = cortex_peri.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(2_100_000);
    syst.enable_counter();

    let rcc = st_peri.RCC;
    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());

    let gpioc = st_peri.GPIOC;
    gpioc.moder.write(|w| w.moder13().output());
    gpioc.otyper.write(|w| w.ot13().push_pull());
    gpioc.ospeedr.write(|w| w.ospeedr13().low_speed());
    gpioc.pupdr.write(|w| w.pupdr13().floating());

    syst.clear_current();

    loop {
        while !syst.has_wrapped() {}
        gpioc.odr.write(|w| w.odr13().set_bit());
        while !syst.has_wrapped() {}
        gpioc.odr.write(|w| w.odr13().clear_bit());
    }
}
