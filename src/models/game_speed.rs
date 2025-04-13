/// Скорость игры.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameSpeed {
    VerySlow = 0,
    Slow = 1,
    Normal = 2,
    Fast = 3,
    VeryFast = 4
}

impl GameSpeed {
    pub fn upgrade(&mut self) {
        *self = match self {
            GameSpeed::VerySlow => GameSpeed::Slow,
            GameSpeed::Slow => GameSpeed::Normal,
            GameSpeed::Normal => GameSpeed::Fast,
            GameSpeed::Fast => GameSpeed::VeryFast,
            GameSpeed::VeryFast => GameSpeed::VeryFast,
        }
    }

    pub fn downgrade(&mut self) {
        *self = match self {
            GameSpeed::VerySlow => GameSpeed::VerySlow,
            GameSpeed::Slow => GameSpeed::VerySlow,
            GameSpeed::Normal => GameSpeed::Slow,
            GameSpeed::Fast => GameSpeed::Normal,
            GameSpeed::VeryFast => GameSpeed::Fast,
        }
    }
}

impl std::fmt::Display for GameSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
