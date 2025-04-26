use std::sync::{mpsc, Arc, Mutex};

use super::*;
use crate::models::{CancellationTokenSource, GameSpeed, GameState, Screen};

/// Запустить игровой процесс.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout().into_arc_mutex();
    render::configure_terminal(stdout.clone());

    let terminal_size = render::terminal_size()?;
    let screen_size = Screen::from(terminal_size);
    let start_game_speed = GameSpeed::Normal; // todo select after start game

    let game_state = GameState::new(screen_size, start_game_speed).into_arc_mutex();

    let (key_event_sender, key_event_receiver) = mpsc::channel();
    let ct_source = CancellationTokenSource::new();

    let ct = ct_source.token();
    let input_thread = input::spawn_input_thread(
        key_event_sender,
        ct);

    let ct = ct_source.token();
    let render_thread = render::spawn_render_thread(
        stdout.clone(),
        game_state.clone(),
        ct);

    let game_thread = game::spawn_game_thread(
        key_event_receiver,
        game_state.clone());

    game_thread.join().unwrap();
    ct_source.cancel();

    render_thread.join().unwrap();
    input_thread.join().unwrap();

    render::draw_endgame_screen(
        stdout.clone(),
        game_state);

    input::wait_for_any_key();
    render::reset_terminal(stdout);
    Ok(())
}


trait ArcMutexWrapper {
    fn into_arc_mutex(self) -> Arc<Mutex<Self>>;
}

impl<T> ArcMutexWrapper for T {
    fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}
