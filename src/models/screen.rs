/// Экран.
pub struct Screen {
    x: u16,
    y: u16
}

/// Направление.
pub enum Rotation {
    X,
    Y
}

impl Screen {
    /// Создать новый экран.
    pub fn new(x: u16, y: u16) -> Self {
        Screen { x, y }
    }

    /// Получить размер экрана.
    pub fn get_size(&self, rotation: Rotation) -> u16 {
        match rotation {
            Rotation::X => self.x,
            Rotation::Y => self.y,
        }
    }
}

impl From<(u16, u16)> for Screen {
    /// Приведение `(u16, u16)` к `Screen`.
    fn from(value: (u16, u16)) -> Self {
        Screen::new(value.0, value.1)
    }
}
