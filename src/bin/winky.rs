//! This example runs the RP Pico on board LED and an external LED connected to Pin 20.


#![no_std]  // We do not want the std library since we do not have an OS with things like a filesystem
#![no_main] // We do not want a main function

use defmt::*; // Logging package
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _}; // Probe/logging package

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default()); // Initialise peripherals
    let mut led = Output::new(p.PIN_15, Level::Low); // Init external LED
    let mut onb_led = Output::new(p.PIN_25, Level::Low); // Init onboard LED
    let delay = Duration::from_secs(1);

    loop {
        info!("onboard_led on!"); // This will appear in the debugging probe
        onb_led.set_level(Level::High);
        Timer::after(delay).await;

        info!("onboard_led off!");
        onb_led.set_level(Level::Low);
        Timer::after(delay).await;

        info!("led on!");
        led.set_high();
        Timer::after(delay).await;

        info!("led off!");
        led.set_low();
        Timer::after(delay).await;


    }
}
