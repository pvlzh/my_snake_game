use super::{Direction, Position, Screen, GameSpeed, Snake};
use rand::Rng;

/// Состояние игры.
pub struct GameState {
    pub snake: Snake,
    food: Option<Position>,
    score: u32,
    is_game_over: bool,
    game_speed: GameSpeed,
    screen_size: Screen,
}

impl GameState {
    /// Создать состояние.
    pub fn new(screen_size: Screen, game_speed: GameSpeed) -> Self {
        let center = Position {
            x: screen_size.size_x() / 2,
            y: screen_size.size_y() / 2,
        };

        let mut state = GameState {
            snake: Snake::new(center, Direction::Right),
            food: None,
            score: 0,
            is_game_over: false,
            game_speed,
            screen_size,
        };

        state.generate_food();
        return state;
    }

    /// Разместить на поле еду.
    pub fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        self.food = Some(Position {
            x: rng.gen_range(1..self.screen_size_x() - 1),
            y: rng.gen_range(1..self.screen_size_y() - 1),
        })
    }

    /// Получить координаты еды.
    pub fn food_position(&self) -> Option<Position> {
        self.food
    }

    /// Увеличить игровой счет.
    pub fn increment_score(&mut self, count: u32) {
        self.score += count;
    }

    /// Получить игровой счет.
    pub fn game_score(&self) -> u32 {
        self.score
    }

    /// Закончить игру.
    pub fn game_over(&mut self) {
        self.is_game_over = true;
    }

    /// Проверить состояние игры.
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    /// Проверить расположение головы змеи по отношению к границам поля.
    pub fn snake_head_in_border(&self) -> bool {
        let snake_head = self.snake.head_position();
        return snake_head.x == 0
            || snake_head.x >= self.screen_size_x() - 1
            || snake_head.y == 0
            || snake_head.y >= self.screen_size_y() - 1;
    }

    /// Проверить расположение головы змеи по отношению к телу.
    pub fn snake_head_in_body(&self) -> bool {
        let snake_head = self.snake.head_position();
        let snake_len = self.snake.body_len();
        return self
            .snake
            .body_position()
            .take(snake_len - 1)
            .any(|p| *p == snake_head);
    }

    /// Получить размер окна по горизонтали.
    pub fn screen_size_x(&self) -> u16 {
        self.screen_size.size_x()
    }

    /// Получить размер окна по вертикали.
    pub fn screen_size_y(&self) -> u16 {
        self.screen_size.size_y()
    }

    /// Получить скорость игры
    pub fn game_speed(&self) -> GameSpeed {
        self.game_speed
    }
}
