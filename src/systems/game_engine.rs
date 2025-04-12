use crossterm as ct;
use std::sync::{mpsc, Arc, Mutex};

use super::*;
use crate::models::{GameSpeed, GameState, Screen};

/// Запустить игровой процесс.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let screen_size = Screen::from(ct::terminal::size()?);
    let game_speed = GameSpeed::Normal; // todo select after start game

    let game_state = Arc::new(Mutex::new(GameState::new(screen_size, game_speed)));

    let (key_event_sender, key_event_receiver) = mpsc::channel();

    let input_thread = input::spawn_input_thread(key_event_sender, game_state.clone());
    let game_thread = game::spawn_game_thread(key_event_receiver, game_state.clone());
    let render_thread = render::spawn_render_thread(game_state.clone());

    game_thread.join().unwrap();
    render_thread.join().unwrap();
    input_thread.join().unwrap();

    Ok(())
}
