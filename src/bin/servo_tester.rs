// #![deny(warnings)]
#![no_main]
#![no_std]

use btoi::btoi;
use common::cli::cli;
use core::fmt::Write;
use cortex_m;
use cortex_m_rt as rt;
use hal::prelude::*;
use hal::stm32;
use panic_rtt_target as _;
use rt::{entry, exception, ExceptionFrame};
use stm32g0xx_hal as hal;

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let mut io = common::init();

    // let cp = hal::pac::CorePeripherals::take().expect("cannot take core peripherals");
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");

    let mut rcc = dp.RCC.constrain();

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut delay = dp.TIM15.delay(&mut rcc);
    let mut pwm = dp.TIM1.pwm(10.kHz(), &mut rcc);
    let blue_button = gpioc.pc13.into_floating_input();
    // let mut green_led = gpioa.pa5.into_push_pull_output();

    let mut pwm_ch1 = pwm.bind_pin(gpioa.pa8);
    // let mut pwm_ch2 = pwm.bind_pin(gpioa.pa9);

    // let max = pwm_ch1.get_max_duty();
    // writeln!(io.log, "max_duty = {}", max).ok(); // wrong? 1599, but actually higher
    pwm_ch1.set_duty(5_500);
    pwm.set_freq(100.Hz());
    writeln!(io.log, "actual freq = {}", pwm.freq()).ok();
    pwm_ch1.enable();

    loop {
        cli(&mut io, |cmd, args, out| match cmd {
            "pwm" => {
                let Some(duty) = args.next() else {
                    writeln!(out, "Expected duty in decimal").ok();
                    return;
                };
                let duty: Result<u32, _> = btoi(duty.as_bytes());
                let Ok(duty) = duty else {
                    writeln!(out, "Number parse error").ok();
                    return;
                };
                // 5_500 .. 11_000
                let duty = 5_500 + duty * 5_500 / 100;
                writeln!(out, "Setting duty={duty}").ok();
                pwm_ch1.set_duty(duty as u16);
            }
            _ => {
                writeln!(out, "Unsupported command '{cmd}'").ok();
            }
        });

        if blue_button.is_low().unwrap_or(false) {
            if pwm_ch1.get_duty() == 5_500 {
                pwm_ch1.set_duty(6_800);
            } else {
                pwm_ch1.set_duty(5_500);
            }
            delay.delay_ms(100u32);
        }
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
