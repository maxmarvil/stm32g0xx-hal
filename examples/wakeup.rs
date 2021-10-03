#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::{
    prelude::*, timer::Timer,delay::Delay, interrupt,
    exti::Event, stm32, time::*,
    gpio::{Output, PushPull, SignalEdge, gpioc::PC6},
    power::{LowPowerMode, PowerMode},
    rtc::{self, Rtc},
    rcc::{RTCSrc, Rcc, Config},
    hal::blocking::delay::DelayMs,
};
use embedded_hdc1080_rs::Hdc1080;
use cortex_m::asm::delay;
use nb::block;
use rt::entry;
use cortex_m::{asm, interrupt::Mutex, peripheral::NVIC};
use hal::stm32::Interrupt::RTC_TAMP;
use hal::stm32g0::stm32g031::exti::exticr1::EXTI16_23_R;

const PERIOD: u32 = 8_000_000;


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let mut exti = dp.EXTI;
    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc6.into_push_pull_output();
    let mut rtc = dp.RTC.constrain(&mut rcc);
    hprintln!("set rtc");
    rtc.wakeup_default(15);
    hprintln!("set WU");
    unsafe { NVIC::unmask(RTC_TAMP) };
    loop {
        asm::wfi();

    }
}

#[interrupt]
fn RTC_TAMP() {

    hprintln!("W! up");
}