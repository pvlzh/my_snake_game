use std::{sync::{mpsc, Arc, Mutex}, thread, time};
use crossterm::event::KeyCode;
use crate::models::{Direction, GameSpeed, GameState};

/// Создать поток логики игры.
pub fn spawn_game_thread(
    key_event_receiver: mpsc::Receiver<KeyCode>,
    state: Arc<Mutex<GameState>>,) -> thread::JoinHandle<()> {

    thread::spawn(move || {
        let mut last_event = time::Instant::now();
        let logic_duration: time::Duration = {
            let state = state.lock().unwrap();
            state.game_speed().into()
        };

        loop {
            let elapsed = last_event.elapsed();
            let remaining = logic_duration
                .checked_sub(elapsed)
                .unwrap_or(time::Duration::ZERO);

            match key_event_receiver.recv_timeout(remaining) {
                Ok(key) => {
                    last_event = time::Instant::now();
                    let mut state = state.lock().unwrap();
                    if let Ok(new_direction) = Direction::try_from(key) {
                        state.snake.change_direction(new_direction);
                    }
                    handle_game(state);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    last_event = time::Instant::now();
                    handle_game(state.lock().unwrap());
                }
                _ => {
                    panic!("Key reciver connection lost");
                }
            }
        }
    })
}

/// Поведение игры.
fn handle_game(mut state: std::sync::MutexGuard<'_, GameState>) {
    let snake_head = state.snake.head_position();

    let mut is_eating = false;
    if let Some(food_position) = state.food_position() {
        is_eating = snake_head == food_position;
    }
    else {
        state.generate_food();
    }

    if is_eating {
        state.increment_score(10);
        state.generate_food();
    }

    state.snake.move_forward(is_eating);

    if state.snake_head_in_border() || state.snake_head_in_body() {
        state.game_over();
    }
}


/// 'KeyCode' to 'Direction'.
impl TryFrom<KeyCode> for Direction {
    type Error = ();

    /// Приведение `KeyCode` к `Direction`.
    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Up => Ok(Direction::Up),
            KeyCode::Down => Ok(Direction::Down),
            KeyCode::Left => Ok(Direction::Left),
            KeyCode::Right => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

/// 'GameSpeed' to 'Duration'.
impl Into<time::Duration> for GameSpeed {
    fn into(self) -> time::Duration {
        match self {
            GameSpeed::VerySlow => time::Duration::from_millis(1000),
            GameSpeed::Slow => time::Duration::from_millis(750),
            GameSpeed::Normal => time::Duration::from_millis(500),
            GameSpeed::Fast => time::Duration::from_millis(350),
            GameSpeed::VeryFast => time::Duration::from_millis(100),
        }
    }
}
