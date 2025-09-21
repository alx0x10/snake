use embassy_futures::select::{select3, Either3};
use embassy_nrf::{gpio::{AnyPin, Input, Pull}, Peri};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal};
use embassy_time::Timer;

use crate::game::Action;

pub static BTN_CHAN: Channel<CriticalSectionRawMutex, Action, 10> = Channel::new();
pub static TOUCH_SIGN: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::task]
pub async fn handle_controls(pin_btn_a: Peri<'static, AnyPin>, pin_btn_b: Peri<'static, AnyPin>, pin_touch: Peri<'static, AnyPin>) {
    let mut btn_a = Input::new(pin_btn_a, Pull::Up);
    let mut btn_b = Input::new(pin_btn_b, Pull::Up);
    let mut touch = Input::new(pin_touch, Pull::None);
    
    loop {
        match select3(
            btn_a.wait_for_falling_edge(),
            btn_b.wait_for_falling_edge(),
            touch.wait_for_falling_edge()
        ).await {
            Either3::First(_) => BTN_CHAN.send(Action::TurnLeft).await,
            Either3::Second(_) => BTN_CHAN.send(Action::TurnRight).await,
            Either3::Third(_) => TOUCH_SIGN.signal(()),
        }

        Timer::after_millis(200).await;
    }
}