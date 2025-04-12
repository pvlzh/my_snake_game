use crossterm::event::{Event, KeyCode};
use std::{sync::{mpsc::Sender, Arc, Mutex}, thread};

use crate::models::GameState;

/// Создать поток ввода.
pub fn spawn_input_thread(key_event_sender: Sender<KeyCode>, state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()>{
    thread::spawn(move || {
        loop {
            match crossterm::event::read() {
                Ok(Event::Key(key_event)) => {
                    if let KeyCode::Esc = key_event.code {
                        state.lock().unwrap().game_over();
                        break;
                    }
                    let _ = key_event_sender.send(key_event.code);
                }
                Err(e) => {
                    eprintln!("Error reading event: {}", e);
                    break;
                },
                _ => {}
            }
        }
    })
}
