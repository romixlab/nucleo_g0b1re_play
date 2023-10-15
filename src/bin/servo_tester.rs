// #![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt as rt;
use panic_rtt_target as _;
use stm32g0xx_hal as hal;

use common::print_fw_info;
use cortex_m::asm;
use hal::prelude::*;
use hal::stm32;
use rt::{entry, exception, ExceptionFrame};
use rtt_target::{rprintln, rtt_init_print};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockSkip);
    print_fw_info(file!());

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");

    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
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
    asm::bkpt();

    pwm_ch1.set_duty(max / 4);
    pwm_ch2.set_duty(max / 8);
    asm::bkpt();

    pwm_ch1.set_duty(max / 8);
    pwm_ch2.set_duty(max / 16);
    asm::bkpt();

    pwm.set_freq(20.kHz());

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault {:#?}", ef);
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
