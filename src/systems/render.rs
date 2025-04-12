use core::time;
use std::{io::Write, sync::{Arc, Mutex}, thread};
use crossterm::{self as ct, QueueableCommand, terminal::EnterAlternateScreen};
use crate::models::GameState;

/// Создать поток рендеринга.
pub fn spawn_render_thread(
    state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()> {

    thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let mut renderer = Renderer::new();

        ct::terminal::enable_raw_mode().unwrap();
        ct::execute!(stdout, EnterAlternateScreen, ct::cursor::Hide).unwrap();

        loop {
            renderer.clear();

            let state = state.lock().unwrap();
            if state.is_game_over() {
                 break;
            }

            for x in 0..state.screen_size_x() {
                renderer.draw(x, 0, "#");
                renderer.draw(x, state.screen_size_y() - 1, "#");
            }

            for pos in state.snake.body_position() {
                renderer.draw(pos.x, pos.y, "■");
            }

            if let Some(food_position) = state.food_position() {
                renderer.draw(food_position.x, food_position.y, "♦");
            }

            renderer.draw(2, 0, &format!("Score: {}", state.game_score()));

            renderer.present(&mut stdout).unwrap();

            thread::sleep(time::Duration::from_secs(1/10));
        }

        ct::execute!(stdout, EnterAlternateScreen, ct::cursor::Show).unwrap();
        ct::terminal::disable_raw_mode().unwrap();
    })
}

pub struct Renderer { // todo
    buffer: Vec<u8>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            buffer: Vec::with_capacity(1024),
        }
    }

    pub fn clear(&mut self) {
        self.buffer
            .queue(ct::terminal::Clear(crossterm::terminal::ClearType::All))
            .unwrap();
    }

    pub fn draw(&mut self, x: u16, y: u16, symbol: &str) {
        self.buffer
            .queue(ct::cursor::MoveTo(x, y))
            .unwrap()
            .queue(ct::style::Print(symbol))
            .unwrap();
    }

    pub fn present(&mut self, stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
        stdout.write_all(&self.buffer)?;
        stdout.flush()?;
        self.buffer.clear();
        Ok(())
    }
}
