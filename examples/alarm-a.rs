#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate rtic;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::{
    prelude::*, timer::Timer,delay::Delay,
    exti::Event, stm32, time::*,
    gpio::{Output, PushPull, SignalEdge, gpioc::PC6},
    power::{LowPowerMode, PowerMode},
    rtc::{self, Rtc},
    rcc::{RTCSrc, Rcc, Config},
    hal::blocking::delay::DelayMs,
};
use rtic::app;
use embedded_hdc1080_rs::Hdc1080;
use rtic::{ Monotonic};
use cortex_m::asm::delay;

const PERIOD: u32 = 8_000_000;
#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        exti: stm32::EXTI,
        timer: Timer<stm32::TIM17>,
        led: PC6<Output<PushPull>>,
        rtc: Rtc
    }

    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let mut rcc = ctx.device.RCC.constrain();
        let gpioc = ctx.device.GPIOC.split(&mut rcc);

        let mut timer = ctx.device.TIM17.timer(&mut rcc);
        let mut led = gpioc.pc6.into_push_pull_output();
        let mut rtc = ctx.device.RTC.constrain(&mut rcc);

        rtc.set_date(&Date::new(2021.year(), 1.month(), 1.day()));
        rtc.set_time(&Time::new(0.hours(), 0.minutes(), 0.seconds(), false));
        hprintln!("set rtc");
        rtc.set_alarma_date(&Date::new(2021.year(), 1.month(), 1.day()));
        rtc.set_alarma_time(&Time::new(0.hours(), 0.minutes(), 30.seconds(), false));
        rtc.init_alarm_a();
        //hprintln!("flag init {:?}", rtc.get_alarm_init());
        //hprintln!("alarm {:?} {:?}", rtc.get_alarma_date(),  rtc.get_alarma_time());
        hprintln!("set alarm");

        // hprintln!("prescallers {:?}", rtc.get_prescaller());
        // hprintln!("time {:?}", rtc.get_time());


        init::LateResources {
            timer,
            exti: ctx.device.EXTI,
            led,
            rtc
        }
    }

    /*    #[idle]
        fn idle(c:idle::Context) -> !{
            loop {
                cortex_m::asm::nop();
            }
        }*/

    #[task( binds = RTC_TAMP, resources = [led, rtc])]
    fn blink(ctx: blink::Context) {

        ctx.resources.led.toggle().unwrap();//toggle().unwrap();
        delay(3_000_000);

        hprintln!("W! up");
    }
    extern "C" {
        fn TIM16();
    }
};