use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use heapless::Deque;

use crate::game::Coordinates;

pub const ROWS: usize = 5;
pub const COLS: usize = 5;

pub type Image = [[u8; COLS]; ROWS];

pub static IMG_SIG: Signal<CriticalSectionRawMutex, Image> = Signal::new();    

/// Convert snake body into a pixel image
pub fn body_to_image(
    body: &Deque<Coordinates, { ROWS * COLS }>
) -> Image {
    let mut img = [[0u8; COLS]; ROWS];
    for coord in body.iter() {
        if let (Some(x), Some(y)) = (usize::try_from(coord.x).ok(), usize::try_from(coord.y).ok()) {
            if x < COLS && y < ROWS {
                img[y][x] = 1;
            }
        }
    }
    img
}

/// Convert image back into a snake body
pub fn image_to_body(
    image: &Image
) -> Deque<Coordinates, { ROWS * COLS }> {
    let mut body = Deque::new();
    for (y, row) in image.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == 1 {
                let _ = body.push_back(Coordinates {
                    x: x as u8,
                    y: y as u8,
                });
            }
        }
    }
    body
}
