use crossterm::event::{Event, KeyCode};
use core::time;
use std::{sync::mpsc::Sender, thread, time::{Duration, Instant}};

use crate::models::CancellationToken;

/// Создать поток ввода.
pub fn spawn_input_thread(key_event_sender: Sender<KeyCode>, ct: CancellationToken) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let key_repeat_cooldown = Duration::from_millis(100);
        let input_idle_duration = time::Duration::from_millis(500);

        let mut input_limiter = InputFrequencyLimiter::new(key_repeat_cooldown);

        while !ct.is_cancelled() {
            if let Ok(true) = crossterm::event::poll(input_idle_duration) {
                if let Ok(Event::Key(key_event)) = crossterm::event::read() {
                    if input_limiter.should_process(key_event.code) {
                        _ = key_event_sender.send(key_event.code);
                    }
                }
            }
        }
    })
}

/// Ожидать нажатия любой клавиши.
pub fn wait_for_any_key() {
    loop {
        match crossterm::event::read() {
            Ok(Event::Key(_)) => break,
            _ => continue
        }
    }
}

/// Ограничитель частоты ввода.
struct InputFrequencyLimiter {
    last_processed: Instant,
    last_processed_key: Option<KeyCode>,
    processing_cooldown: Duration,
}

impl InputFrequencyLimiter {
    /// Создать ограничитель.
    fn new(cooldown: Duration) -> Self {
        InputFrequencyLimiter {
            last_processed: Instant::now(),
            last_processed_key: None,
            processing_cooldown: cooldown,
        }
    }

    /// Принять решение по обработке нажатия.
    fn should_process(&mut self, key: KeyCode) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_processed) >= self.processing_cooldown || self.last_processed_key != Some(key) {
            self.last_processed = now;
            self.last_processed_key = Some(key);
            true
        } else {
            false
        }
    }
}
