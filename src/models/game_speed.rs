/// Скорость игры.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameSpeed {
    VerySlow = 0,
    Slow = 1,
    Normal = 2,
    Fast = 3,
    VeryFast = 4
}

impl std::fmt::Display for GameSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
