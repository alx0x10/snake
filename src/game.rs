use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Timer;
use heapless::Deque;

use crate::{controls::BTN_CHAN, display::{image::{body_to_image, Image, COLS, IMG_SIG, ROWS}, images::{FOOD_INIT, SNAKE_INIT}}};

pub static GAME_STATE: Mutex<CriticalSectionRawMutex, Option<GameState>> = Mutex::new(None);

pub struct GameState {
    snake: Snake,
    //food_position: Coordinates,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            snake: Snake::new_at( Coordinates { x: 0, y: 2 }, 3, Direction::Right),
            //food_position: FOOD_INIT,
        }
    }
    
    pub fn next_step(&mut self) {
        self.snake.next_step();
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

pub struct Coordinates {
    pub x: u8,
    pub y: u8
}

pub struct Snake {
    body: Deque<Coordinates, {(ROWS as usize) * (COLS as usize)}>,
    moving_direction: Direction,
    length: u8
}

impl Snake {
pub fn new_at(start: Coordinates, length: u8, moving_direction: Direction) -> Snake {
    let mut body: Deque<Coordinates, { (ROWS as usize) * (COLS as usize) }> = Deque::new();

    let target_len = usize::from(length);
    
    for i in 0..target_len {
        let x = start.x.wrapping_add(i as u8);
        let _ = body.push_back(Coordinates { x, y: start.y });
    }
    
    Snake { body, moving_direction, length }
}

    
    pub fn turn_right(&mut self) {
        self.moving_direction = match self.moving_direction {
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Up    => Direction::Right,
        };
    }

    pub fn turn_left(&mut self) {
        self.moving_direction = match self.moving_direction {
            Direction::Right => Direction::Up,
            Direction::Up    => Direction::Left,
            Direction::Left  => Direction::Down,
            Direction::Down  => Direction::Right,
        };
    }

    
    pub fn new_head(&self) -> Option<Coordinates> {
        let max_x = (COLS - 1) as u8;
        let max_y = (ROWS - 1) as u8;
        
        let head = self.body.back()?;
        let next = match self.moving_direction {
            Direction::Right => Coordinates { x: if head.x == max_x { 0 } else { head.x + 1 }, y: head.y },
            Direction::Left  => Coordinates { x: if head.x == 0     { max_x } else { head.x - 1 }, y: head.y },
            Direction::Down  => Coordinates { x: head.x, y: if head.y == max_y { 0 } else { head.y + 1 } },
            Direction::Up    => Coordinates { x: head.x, y: if head.y == 0     { max_y } else { head.y - 1 } },
        };
        Some(next)
    }

    pub fn next_step(&mut self) {
        if let Some(new_head) = self.new_head() {
            let _ = self.body.pop_front();
            let _ = self.body.push_back(new_head);
        }
    }
}

pub enum Action {
    TurnRight,
    TurnLeft
}

#[embassy_executor::task]
pub async fn game_task() {
    let actions_receiver = BTN_CHAN.receiver();

    loop {
        let action = actions_receiver.try_receive().ok();

        {
            let mut game_state_opt = GAME_STATE.lock().await;
            let game_state = game_state_opt.get_or_insert_with(GameState::new);

            if let Some(Action::TurnRight) = action { game_state.snake.turn_right(); }
            if let Some(Action::TurnLeft)  = action { game_state.snake.turn_left();  }

            game_state.next_step();
         
            IMG_SIG.signal(body_to_image(&game_state.snake.body));
        }

        embassy_time::Timer::after_millis(300).await;
    }
}
