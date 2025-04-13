use core::time;
use std::{io::{Stdout, Write}, sync::{Arc, Mutex}, thread};
use crossterm::{self as ct, terminal::EnterAlternateScreen, QueueableCommand};
use crate::models::{GameState, Rotation};

/// Создать поток рендеринга.
pub fn spawn_render_thread(stdout: Arc<Mutex<Stdout>>, state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut render_buff = RenderBuffer::new();

        loop {
            render_buff.clear();

            let state = state.lock().unwrap();
            if state.is_game_over() {
                 break;
            }

            let screen_width = state.screen_size(Rotation::X);
            let screen_height = state.screen_size(Rotation::Y);

            for x in 0..screen_width {
                render_buff.draw(x, 0, "#");
                render_buff.draw(x, screen_height - 1, "#");
            }
            for y in 0..screen_height {
                render_buff.draw(0, y, "#");
                render_buff.draw(screen_width - 1, y, "#");
            }

            for pos in state.snake.body_position() {
                render_buff.draw(pos.x, pos.y, "■");
            }

            if let Some(food_position) = state.food_position() {
                render_buff.draw(food_position.x, food_position.y, "♦");
            }

            let current_score = state.game_score();
            let current_game_speed = state.game_speed();

            render_buff.draw(2, 0, &format!(" Score: {}|Speed: {} ", current_score, current_game_speed));
            render_buff.draw(2, screen_height - 1, &format!(" {}x{} ", screen_width, screen_height));
            render_buff.draw(screen_width - 16, screen_height - 1, &format!(" Esc for Exit "));

            let mut stdout = stdout.lock().unwrap();
            render_buff.present(&mut stdout).unwrap();

            thread::sleep(time::Duration::from_secs(1/30));
        }
    })
}

/// Отрисовать экран конца игры.
pub fn draw_endgame_screen(stdout: Arc<Mutex<Stdout>>, state: Arc<Mutex<GameState>>) {
    let mut render_buff = RenderBuffer::new();

    let game_state = state.lock().unwrap();
    let screen_width = game_state.screen_size(Rotation::X);
    let screen_height = game_state.screen_size(Rotation::Y);

    for x in 0..screen_width {
        render_buff.draw(x, 0, "#");
        render_buff.draw(x, screen_height - 1, "#");
    }
    for y in 0..screen_height {
        render_buff.draw(0, y, "#");
        render_buff.draw(screen_width - 1, y, "#");
    }

    let mut center_screen = screen_height / 2;

    fn center_x(text: &str, width: u16) -> u16 {
        let text_len = text.len() as u16;
        (width.saturating_sub(text_len)) / 2
    }

    let game_over_text = "Game Over!";
    let x = center_x(game_over_text, screen_width);
    render_buff.draw(x, center_screen, game_over_text);
    center_screen += 2;

    let score_text = format!("Score: {}", game_state.game_score());
    let speed_text = format!("Speed: {}", game_state.game_speed());
    let screen_size_text = format!("Screen Size: {}x{}", screen_width, screen_height);

    for text in &[score_text, speed_text, screen_size_text] {
        let x = center_x(text, screen_width);
        render_buff.draw(x, center_screen, text);
        center_screen += 1;
    }
    center_screen += 1;
    
    let exit_text = "Press any key to exit...";
    let x = center_x(exit_text, screen_width);
    render_buff.draw(x, center_screen, exit_text);

    let mut stdout = stdout.lock().unwrap();
    render_buff.present(&mut stdout).unwrap();
}

pub fn terminal_size() -> std::io::Result<(u16, u16)> {
    ct::terminal::size()
}

/// Настроить вывод терминала.
pub fn configure_terminal(stdout: Arc<Mutex<std::io::Stdout>>) {
    let mut stdout = stdout.lock().unwrap();
    ct::terminal::enable_raw_mode().unwrap();
    ct::execute!(stdout, EnterAlternateScreen, ct::cursor::Hide).unwrap();
}

/// Настроить сбросить настройки вывода терминала.
pub fn reset_terminal(stdout: Arc<Mutex<std::io::Stdout>>) {
    let mut stdout = stdout.lock().unwrap();
    ct::execute!(stdout, EnterAlternateScreen, ct::cursor::Show).unwrap();
    ct::terminal::disable_raw_mode().unwrap();
}


pub struct RenderBuffer {
    buffer: Vec<u8>,
}

impl RenderBuffer {
    pub fn new() -> Self {
        RenderBuffer {
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
