#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
use crate::{fmt::unwrap, image::IMG_SIG};

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::{gpio::{AnyPin, Level, Output, OutputDrive}, Peri};
use embassy_time::{Duration, Ticker};
use image::Image;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    
    let rows = [
        p.P0_21.into(),
        p.P0_22.into(),
        p.P0_15.into(),
        p.P0_24.into(),
        p.P0_19.into(),
        ];
        
    let cols = [
        p.P0_28.into(),
        p.P0_11.into(),
        p.P0_31.into(),
        p.P1_05.into(),
        p.P0_30.into(),
    ];
            
    const HAPPY: Image = [
        [0,1,0,1,0],
        [0,1,0,1,0],
        [0,0,0,0,0],
        [1,0,0,0,1],
        [0,1,1,1,0],
    ];

    unwrap!(spawner.spawn(display_task(rows, cols)));
    
    IMG_SIG.signal(HAPPY);
}

#[embassy_executor::task]
async fn display_task(r_pins: [Peri<'static, AnyPin>; image::ROWS], c_pins: [Peri<'static, AnyPin>; image::COLS]) {
    let mut r_pins = r_pins.map(|r| Output::new(r, Level::Low, OutputDrive::Standard));
    let mut c_pins = c_pins.map(|c| Output::new(c, Level::High, OutputDrive::Standard));
    
    let mut img = image::IMG_SIG.wait().await;

    let mut ticker = Ticker::every(Duration::from_hz(60 * img.len() as u64));

    loop {
        for (r_pin, r_img) in r_pins.iter_mut().zip(img) {
            r_pin.set_high();

            c_pins
                .iter_mut()
                .zip(r_img)
                .filter(|(_, c_img)| *c_img != 0)
                .for_each(|(c_pin, _)| c_pin.set_low());

            ticker.next().await;

            r_pin.set_low();
            c_pins.iter_mut().for_each(|c_pin| c_pin.set_high());

            if let Some(new_img) = IMG_SIG.try_take() {
                img = new_img;

                break
            }
        }
    }
}

mod image {
    use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

    pub const ROWS: usize = 5;
    pub const COLS: usize = 5;

    pub type Image = [[u8; COLS]; ROWS];

    pub static IMG_SIG: Signal<CriticalSectionRawMutex, Image> = Signal::new();    
}