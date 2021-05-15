use crate::food::*;
use piston_window::math::Matrix2d;
use piston_window::G2d;
use std::cell::{Cell, RefCell};
use std::vec::Vec;

pub const SNAKE_SPEED: u16 = 20; // How many pixels to move the snake each frame

pub const GAME_WIDTH: u32 = 1000;
pub const GAME_HEIGHT: u32 = 1000;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SnakeDirection {
    Intial,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Snake {
    direction: Cell<SnakeDirection>,
    body: RefCell<Vec<Food>>,
    accelerate : Cell<bool>
}

impl Snake {
    fn get_head_x(&self) -> u16 {
        // Returns the x position of the snake's head
        let b = self.body.borrow();
        b[0].get_x().unwrap()
    }

    fn get_head_y(&self) -> u16 {
        // Returns the y position of the snake's head
        let b = self.body.borrow();
        b[0].get_y().unwrap()
    }

    pub fn can_eat(&self, food: &Food) -> bool {
        // True if snake is in range to eat the food
        if let Some(x) = food.get_x() {
            if let Some(y) = food.get_y() {
                self.get_head_x() == x && self.get_head_y() == y
            } else {
                false //y is None
            }
        } else {
            false // x is None
        }
    }

    pub fn has_collided_with_any_wall(&self) -> bool {
        let x = self.get_head_x() as i16 * (FOOD_WIDTH as i16);
        let y = self.get_head_y() as i16 * (FOOD_HEIGHT as i16);
        if (x - (SNAKE_SPEED as i16)) < 0 || (y - (SNAKE_SPEED as i16)) < 0 {
            true
        } else if (x + (SNAKE_SPEED as i16)) > (GAME_WIDTH as i16)
            || (y + (SNAKE_SPEED as i16)) > (GAME_HEIGHT as i16)
        {
            true
        } else {
            false
        }
    }

    pub fn render(&self, transform: &Matrix2d, graphics: &mut G2d) {
        let body = self.body.borrow();
        for part in body.iter() {
            part.render(transform, graphics, true);
        }
    }

    pub fn set_direction(&self, direction: SnakeDirection) {
        let current_direction = self.direction.get();
        if (current_direction == SnakeDirection::Left || current_direction == SnakeDirection::Right) // Don't allow moving from the opposite direction to avoid eating one self
            && (direction == SnakeDirection::Left || direction == SnakeDirection::Right)
        {
            return;
        }
        else if (current_direction == SnakeDirection::Up || current_direction == SnakeDirection::Down) 
        && (direction == SnakeDirection::Up || direction == SnakeDirection::Down)
        {
            return;
        }
        self.direction.set(direction);
    }

    pub fn ate_itself(&self) -> bool {
        let x = self.get_head_x();
        let y = self.get_head_y();
        let body = self.body.borrow_mut();
        for i in 1..body.len() {
            let current_part = &body[i];
            if current_part.equal(x, y) {
                return true;
            }
        }
        return false;
    }

    pub fn toggle_acceleration(&self) {
        self.accelerate.set(!self.accelerate.get());
    }

    pub fn walk(&self, eating: bool) {
        // I want to use move for this method name but its a reserved word. Thanks Rust! 😂
        let mut body = self.body.borrow_mut();
        let old_head = body[0].clone();
        let offset: (i16, i16) = match self.direction.get() {
            SnakeDirection::Left => (-1, 0),
            SnakeDirection::Right => (1, 0),
            SnakeDirection::Up => (0, -1),
            SnakeDirection::Down => (0, 1),
            SnakeDirection::Intial => (0, 0),
        };
        let new_x = (old_head.get_x().unwrap() as i16) + offset.0 + if self.accelerate.get() {
            1 * offset.0
        }
        else {
            0
        };
        let new_y = (old_head.get_y().unwrap() as i16) + offset.1 + if self.accelerate.get() {
            1 * offset.1
        }
        else {
            0
        };
        body.insert(0, Food::new(new_x as u16, new_y as u16));
        if !eating {
            body.pop();
        }
    }

    pub fn new() -> Self {
        Snake {
            direction: Cell::new(SnakeDirection::Intial),
            body: RefCell::new(vec![Food::new(20, 20)]),
            accelerate : Cell::new(false)
        }
    }
}

#[test]
fn test_collision_wall() {
    let snake = Snake::new();
    {
        let mut body = snake.body.borrow_mut();
        body[0].set_x(1);
        body[0].set_y(1);
    }
    assert_eq!(false, snake.has_collided_with_any_wall());
    snake.set_direction(SnakeDirection::Left);
    snake.walk(false);
    assert_eq!(true, snake.has_collided_with_any_wall());
}

#[test]
fn test_snake_direction() {
    let snake = Snake::new();
    snake.set_direction(SnakeDirection::Up);
    assert_eq!(SnakeDirection::Up, snake.direction.get());
    snake.set_direction(SnakeDirection::Down);
    assert_eq!(SnakeDirection::Down, snake.direction.get());
}
