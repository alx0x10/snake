#![no_std]
#![no_main]

mod display;
mod controls;
mod game;

use defmt::unwrap;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;

use crate::{controls::handle_controls, display::{display_task, image::{body_to_image, IMG_SIG}, images::HAPPY}, game::game_task};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    
    // Display setup
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

    unwrap!(spawner.spawn(display_task(rows, cols)));

    unwrap!(spawner.spawn(handle_controls(p.P0_14.into(), p.P0_23.into(), p.P1_04.into())));

    unwrap!(spawner.spawn(game_task()));
}
