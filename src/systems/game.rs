use std::{sync::{mpsc, Arc, Mutex}, thread, time};
use crossterm::event::KeyCode;
use crate::models::{Direction, GameSpeed, GameState};

/// Создать поток логики игры.
pub fn spawn_game_thread(
    key_event_receiver: mpsc::Receiver<KeyCode>,
    state: Arc<Mutex<GameState>>,) -> thread::JoinHandle<()> {

    thread::spawn(move || {
        let mut game_speed = {
            let state = state.lock().unwrap();
            state.game_speed()
        };

        loop {
            let event = key_event_receiver.recv_timeout(game_speed.into());

            let mut state = state.lock().unwrap();
            if state.is_game_over() {
                break;
            }

            match event {
                Ok(KeyCode::Esc) => {
                    state.game_over();
                    continue;
                }
                Ok(key) => {
                    if let Ok(new_direction) = Direction::try_from(key) {
                        state.snake.change_direction(new_direction);
                    }
                }
                _ => {}
            }

            handle_game(&mut state);

            if game_speed != state.game_speed() {
                game_speed = state.game_speed();
            }
        }
    })
}

/// Логика игры.
fn handle_game(state:&mut GameState) {
    let mut is_eating = false;
    if let Some(food_position) = state.food_position() {
        is_eating = state.snake.head_position() == food_position;
    }
    else {
        state.generate_food();
    }

    if is_eating {
        state.increment_score(10);
        if state.score_increment_counter() >= 5 {
            state.up_game_speed();
            state.reset_score_increment_counter();
        }
        state.generate_food();
    }

    state.snake.move_forward(is_eating);

    if state.is_snake_head_in_border() || state.is_snake_head_in_body() {
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
            GameSpeed::VerySlow => time::Duration::from_millis(550),
            GameSpeed::Slow => time::Duration::from_millis(400),
            GameSpeed::Normal => time::Duration::from_millis(250),
            GameSpeed::Fast => time::Duration::from_millis(100),
            GameSpeed::VeryFast => time::Duration::from_millis(50),
        }
    }
}
