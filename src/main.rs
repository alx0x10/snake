#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut row1 = Output::new(p.P0_21, Level::Low, OutputDrive::Standard);
    let mut _col1 = Output::new(p.P0_28, Level::Low, OutputDrive::Standard);

    loop {
        info!("Hello, World!");
        row1.set_high();
        Timer::after_millis(500).await;
        row1.set_low();
        Timer::after_millis(500).await;
    }
}
