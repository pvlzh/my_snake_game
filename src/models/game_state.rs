use super::{screen::Rotation, Direction, GameSpeed, Position, Screen, Snake};
use rand::Rng;

/// Состояние игры.
pub struct GameState {
    pub snake: Snake,
    food: Option<Position>,
    score: u32,
    score_increment_counter: u32,
    is_game_over: bool,
    game_speed: GameSpeed,
    screen: Screen,
}

impl GameState {
    /// Создать состояние.
    pub fn new(screen: Screen, game_speed: GameSpeed) -> Self {
        let center = Position {
            x: screen.get_size(Rotation::X) / 2,
            y: screen.get_size(Rotation::Y) / 2,
        };

        let mut state = GameState {
            snake: Snake::new(center, Direction::Right),
            food: None,
            score: 0,
            score_increment_counter: 0,
            is_game_over: false,
            game_speed,
            screen,
        };

        state.generate_food();
        return state;
    }

    /// Разместить на поле еду.
    pub fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        self.food = Some(Position { // todo posible generation into snake body
            x: rng.gen_range(1..self.screen_size(Rotation::X) - 1),
            y: rng.gen_range(1..self.screen_size(Rotation::Y) - 1),
        })
    }

    /// Получить координаты еды.
    pub fn food_position(&self) -> Option<Position> {
        self.food
    }

    /// Увеличить игровой счет.
    pub fn increment_score(&mut self, count: u32) {
        self.score += count;
        self.score_increment_counter += 1;
    }

    pub fn reset_score_increment_counter(&mut self) {
        self.score_increment_counter = 0;
    }

    /// Получить игровой счет.
    pub fn game_score(&self) -> u32 {
        self.score
    }

    pub fn score_increment_counter(&self) -> u32 {
        self.score_increment_counter
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
    pub fn is_snake_head_in_border(&self) -> bool {
        let snake_head = self.snake.head_position();
        return snake_head.x == 0
            || snake_head.x >= self.screen_size(Rotation::X) - 1
            || snake_head.y == 0
            || snake_head.y >= self.screen_size(Rotation::Y) - 1;
    }

    /// Проверить расположение головы змеи по отношению к телу.
    pub fn is_snake_head_in_body(&self) -> bool {
        let snake_head = self.snake.head_position();
        let snake_len = self.snake.body_len();
        return self
            .snake
            .body_position()
            .take(snake_len - 1)
            .any(|p| *p == snake_head);
    }

    /// Получить размер окна.
    pub fn screen_size(&self, rotation: Rotation) -> u16 {
        self.screen.get_size(rotation)
    }

    /// Получить скорость игры
    pub fn game_speed(&self) -> GameSpeed {
        self.game_speed
    }

    /// Повысить скорость игры.
    pub fn up_game_speed(&mut self) {
        self.game_speed.upgrade();
    }

    /// Понизить скорость игры.
    pub fn down_game_speed(&mut self) {
        self.game_speed.downgrade();
    }
}
