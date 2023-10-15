// #![deny(warnings)]
#![no_main]
#![no_std]

use common::cli::cli;
use core::fmt::Write;
use cortex_m;
use cortex_m::asm;
use cortex_m_rt as rt;
use hal::prelude::*;
use hal::stm32;
use panic_rtt_target as _;
use rt::{entry, exception, ExceptionFrame};
use stm32g0xx_hal as hal;

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let mut io = common::init(file!());

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");

    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut timer = dp.TIM17.timer(&mut rcc);
    timer.start(1.millis());
    let mut pwm = dp.TIM1.pwm(10.kHz(), &mut rcc);

    let blue_button = gpioc.pc13.into_floating_input();
    let mut green_led = gpioa.pa5.into_push_pull_output();

    let mut pwm_ch1 = pwm.bind_pin(gpioa.pa8);
    let mut pwm_ch2 = pwm.bind_pin(gpioa.pa9);

    let max = pwm_ch1.get_max_duty();
    pwm_ch1.set_duty(max / 2);
    pwm_ch2.set_duty(max / 4);

    pwm_ch1.enable();
    pwm_ch2.enable();

    pwm_ch1.set_duty(max / 4);
    pwm_ch2.set_duty(max / 8);

    pwm_ch1.set_duty(max / 8);
    pwm_ch2.set_duty(max / 16);

    pwm.set_freq(20.kHz());

    let mut now: u16 = timer.get_current() as u16;
    let mut counter = 0;
    loop {
        if (timer.get_current() as u16).wrapping_sub(now) >= 1_000 {
            writeln!(io.log, "alive {counter} {}", timer.get_current()).ok();
            now = timer.get_current() as u16;
            counter += 1;
        }
        cli(&mut io, |cmd, args, out| match cmd {
            "pwm" => {}
            _ => {
                writeln!(out, "Unsupported command '{cmd}'").ok();
            }
        });
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault {:#?}", ef);
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
