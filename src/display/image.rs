use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

pub const ROWS: usize = 5;
pub const COLS: usize = 5;

pub type Image = [[u8; COLS]; ROWS];

pub static IMG_SIG: Signal<CriticalSectionRawMutex, Image> = Signal::new();    
