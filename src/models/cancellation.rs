use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

/// Управление токенами отмены.
pub struct CancellationTokenSource {
    is_cancelled: Arc<AtomicBool>
}

impl CancellationTokenSource {
    pub fn new() -> Self {
        CancellationTokenSource {
            is_cancelled: Arc::new(AtomicBool::new(false))
        }
    }

    /// Получить токен.
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            is_cancelled: Arc::clone(&self.is_cancelled)
        }
    }

    /// Отозвать токен.
    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }
}

/// Токен отмены.
pub struct CancellationToken {
    is_cancelled: Arc<AtomicBool>
}

impl CancellationToken {
    /// Проверить состояние токена.
    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }
}
