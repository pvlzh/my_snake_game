/// Экран.
pub struct Screen {
    x: u16,
    y: u16
}

impl Screen {
    /// Создать новый экран.
    pub fn new(x: u16, y: u16) -> Self {
        Screen { x, y }
    }

    /// Получить ширину экрана.
    pub fn size_x(&self) -> u16 {
        self.x
    }
    
    /// Получить высоту экрана. 
    pub fn size_y(&self) -> u16 {
        self.y
    }
}

impl From<(u16, u16)> for Screen {
    /// Приведение `(u16, u16)` к `Screen`.
    fn from(value: (u16, u16)) -> Self {
        Screen::new(value.0, value.1)
    }
}