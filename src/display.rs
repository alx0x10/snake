pub mod image;
pub mod images;

use embassy_nrf::{gpio::{AnyPin, Level, Output, OutputDrive}, Peri};
use embassy_time::{Duration, Ticker};

use self::image::IMG_SIG;

#[embassy_executor::task]
pub async fn display_task(r_pins: [Peri<'static, AnyPin>; image::ROWS], c_pins: [Peri<'static, AnyPin>; image::COLS]) {
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