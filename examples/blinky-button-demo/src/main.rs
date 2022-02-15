#![no_main]
#![no_std]

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use nrf5340_app_hal as hal;
use nrf5340_app_hal::gpio::Level;
use rtt_target::{rprintln, rtt_init_print};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0_NS);
    let button = port0.p0_23.into_pullup_input();
    let mut led = port0.p0_28.into_push_pull_output(Level::Low);

    rprintln!("Blinky button demo starting");
    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
