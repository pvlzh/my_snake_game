use core::time;
use std::{io::Write, sync::{Arc, Mutex}, thread};
use crossterm::{self as ct, QueueableCommand, terminal::EnterAlternateScreen};
use crate::models::{GameState, Rotation};

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

            let screen_width = state.screen_size(Rotation::X);
            let screen_height = state.screen_size(Rotation::Y);

            for x in 0..screen_width {
                renderer.draw(x, 0, "#");
                renderer.draw(x, screen_height - 1, "#");
            }
            for y in 0..screen_height {
                renderer.draw(0, y, "#");
                renderer.draw(screen_width - 1, y, "#");
            }

            for pos in state.snake.body_position() {
                renderer.draw(pos.x, pos.y, "■");
            }

            if let Some(food_position) = state.food_position() {
                renderer.draw(food_position.x, food_position.y, "♦");
            }

            let current_score = state.game_score();
            let current_game_speed = state.game_speed();

            renderer.draw(2, 0, &format!(" Score: {}|Speed: {} ", current_score, current_game_speed));
            renderer.draw(2, screen_height - 1, &format!(" {}x{} ", screen_width, screen_height));
            renderer.draw(screen_width - 16, screen_height - 1, &format!(" Esc for Exit "));

            renderer.present(&mut stdout).unwrap();

            thread::sleep(time::Duration::from_secs(1/30));
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
