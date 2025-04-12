use std::collections::VecDeque;
use super::{Direction, Position};

/// Змейка.
pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction
}

impl Snake {
    /// Создать новую змейку.
    pub fn new(start_pos: Position, direction: Direction) -> Self {
        let body = VecDeque::from([start_pos]);
        return Snake { body, direction };
    }

    /// Получить координаты головы змейки.
    pub fn head_position(&self) -> Position {
        *self.body.back().unwrap()
    }

    /// Получить координаты каждой части тела змейки.
    pub fn body_position(&self) -> impl Iterator<Item = &Position> + '_ {
        self.body.iter()
    }

    /// Длина тела змейки.
    pub fn body_len(&self) -> usize {
        self.body.len()
    }

    /// Изменить направление змейки.
    pub fn change_direction(&mut self, new_direction: Direction) {
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) |
            (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) |
            (Direction::Right, Direction::Left) => return,
            _ => self.direction = new_direction,
        }
    }

    /// Передвинуть змейку вперед.
    pub fn move_forward(&mut self, is_eating: bool) {
        let head = *self.body.back().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        self.body.push_back(new_head);
        
        if !is_eating {
            self.body.pop_front();
        }
    }

}