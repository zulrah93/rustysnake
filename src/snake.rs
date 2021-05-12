use crate::food::*;
use piston_window::math::Matrix2d;
use piston_window::{clear, rectangle, G2d};
use std::cell::{Cell, RefCell};
use std::vec::Vec;

pub const SNAKE_SPEED: u16 = 25; // How many pixels to move the snake each frame
const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 255.0];

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

pub struct Snake {
    head_x: Cell<u16>,
    head_y: Cell<u16>,
    direction: Cell<SnakeDirection>,
    food_eaten: RefCell<Vec<Food>>
}

impl Snake {
    fn get_head_x(&self) -> u16 {
        // Returns the x position of the snake's head
        self.head_x.get()
    }

    fn get_head_y(&self) -> u16 {
        // Returns the y position of the snake's head
        self.head_y.get()
    }

    fn get_tail_x(&self) -> Option<u16> {
        // Returns an Optional x position of the snake's head
        if let Some(tail) = self.food_eaten.borrow().last() {
            tail.get_x()
        } else {
            None
        }
    }

    fn get_tail_y(&self) -> Option<u16> {
        // Returns an Optional y position of the snake's head
        if let Some(tail) = self.food_eaten.borrow().last() {
            tail.get_y()
        } else {
            None
        }
    }

    pub fn eat(&self, mut food: Food) {
        if let Some(x) = self.get_tail_x() {
            if let Some(y) = self.get_tail_y() {
                match self.direction.get() {
                    SnakeDirection::Left => {
                        food.set_x(x - FOOD_WIDTH);
                        food.set_y(y);
                    },
                    SnakeDirection::Right => {
                        food.set_x(x + FOOD_WIDTH);
                        food.set_y(y);
                    },
                    SnakeDirection::Up => {
                        food.set_x(x);
                        food.set_y(y + FOOD_HEIGHT);
                    },
                    SnakeDirection::Down => {
                        food.set_x(x );
                        food.set_y(y - FOOD_HEIGHT);
                    },
                    SnakeDirection::Intial => {
                        //Do nothing this is so the snake starts stationary
                        
                    }
                }
                let mut food_eaten = self.food_eaten.borrow_mut();
                food_eaten.push(food.clone());
            }
        } else {
            match self.direction.get() {
                SnakeDirection::Left => {
                    food.set_x(self.head_x.get() - FOOD_WIDTH);
                    food.set_y(self.head_y.get());
                },
                SnakeDirection::Right => {
                    food.set_x(self.head_x.get() + FOOD_WIDTH);
                    food.set_y(self.head_y.get());
                },
                SnakeDirection::Up => {
                    food.set_x(self.head_x.get());
                    food.set_y(self.head_y.get() + FOOD_HEIGHT);
                },
                SnakeDirection::Down => {
                    food.set_x(self.head_x.get());
                    food.set_y(self.head_y.get() - FOOD_HEIGHT);
                },
                SnakeDirection::Intial => {
                    //Do nothing this is so the snake starts stationary
                }
            }
            let mut food_eaten = self.food_eaten.borrow_mut();
            food_eaten.push(food.clone());
        }
    }

    pub fn can_eat(&self, food: &Food) -> bool {
        // True if snake is in range to eat the food
        if let Some(x) = food.get_x() {
            if let Some(y) = food.get_y() {
                collided_with(
                    Point::new(self.head_x.get(), self.head_y.get()),
                    Point::new(
                        self.head_x.get() + FOOD_WIDTH,
                        self.head_y.get() - FOOD_HEIGHT,
                    ),
                    Point::new(x, y),
                    Point::new(x + FOOD_WIDTH, y - FOOD_HEIGHT),
                )
            } else {
                false //y is None
            }
        } else {
            false // x is None
        }
    }

    pub fn has_collided_with_any_wall(&self) -> bool {
        let x = self.head_x.get() as i16;
        let y = self.head_y.get() as i16;
        if (x - (SNAKE_SPEED as i16)) < 0 || (y - (SNAKE_SPEED as i16)) < 0 {
            true
        } else if (x + (SNAKE_SPEED as i16)) > (GAME_WIDTH as i16) || (y + (SNAKE_SPEED as i16)) > (GAME_HEIGHT as i16) {
            true
        } else {
            false
        }
    }

    pub fn render(&self, transform: &Matrix2d, graphics: &mut G2d) {
        //Render snake but clear the screen first
        clear(CLEAR_COLOR, graphics);
        //Render head
        rectangle(
            SNAKE_COLOR,
            [
                self.get_head_x() as f64,
                self.get_head_y() as f64,
                FOOD_WIDTH as f64,
                FOOD_HEIGHT as f64,
            ],
            *transform,
            graphics,
        );
        let foods = self.food_eaten.borrow();
        for food in foods.iter() {
            food.render(transform, graphics, true);
        }
    }

    pub fn set_direction(&self, direction: SnakeDirection) {
        self.direction.set(direction);
    }

    pub fn walk(&self) {
        // I want to use move for this method name but its a reserved word. Thanks Rust! 😂
        let mut foods = self.food_eaten.borrow_mut();
        match self.direction.get() {
            SnakeDirection::Left => {
                self.head_x.set(self.head_x.get() - SNAKE_SPEED);
                let mut previous = (self.head_x.get(), self.head_y.get());
                for food in foods.iter_mut().rev() {
                    food.set_x(previous.0 -  FOOD_WIDTH - SNAKE_SPEED);
                    food.set_y(previous.1);
                    previous = (food.get_x().unwrap(), food.get_y().unwrap());
                }
            }
            SnakeDirection::Right => {
                self.head_x.set(self.head_x.get() + SNAKE_SPEED);
                let mut previous = (self.head_x.get(), self.head_y.get());
                for food in foods.iter_mut().rev() {
                    food.set_x(previous.0 +  FOOD_WIDTH + SNAKE_SPEED);
                    food.set_y(previous.1);
                    previous = (food.get_x().unwrap(), food.get_y().unwrap());
                }
            }
            SnakeDirection::Down => {
                self.head_y.set(self.head_y.get() + SNAKE_SPEED);
                let mut previous = (self.head_x.get(), self.head_y.get());
                for food in foods.iter_mut().rev() {
                    food.set_y(previous.1 +  FOOD_HEIGHT + SNAKE_SPEED);
                    food.set_x(previous.0);
                    previous = (food.get_x().unwrap(), food.get_y().unwrap());
                }
            }
            SnakeDirection::Up => {
                self.head_y.set(self.head_y.get() - SNAKE_SPEED);
                let mut previous = (self.head_x.get(), self.head_y.get());
                for food in foods.iter_mut().rev() {
                    food.set_y(previous.1 - FOOD_HEIGHT - SNAKE_SPEED);
                    food.set_x(previous.0);
                    previous = (food.get_x().unwrap(), food.get_y().unwrap());
                }
            },
            SnakeDirection::Intial => {
                
            }
        }
    }

    pub fn new() -> Self {
        Snake {
            head_x: Cell::new(((GAME_WIDTH / 2) as u16) - FOOD_WIDTH),
            head_y: Cell::new(((GAME_HEIGHT / 2) as u16) - FOOD_HEIGHT),
            direction: Cell::new(SnakeDirection::Intial),
            food_eaten: RefCell::new(Vec::new())
        }
    }
}

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

fn collided_with(l1: Point, r1: Point, l2: Point, r2: Point) -> bool {
    if l1.x > r2.x || l2.x > r1.x {
        return false;
    }

    if l1.y < r2.y || l2.y < r1.y {
        return false;
    }

    return true;
}

#[test]
fn test_bounding_box() {
    let l1 = Point::new(0, 10);
    let l2 = Point::new(5, 5);
    let r1 = Point::new(10, 0);
    let r2 = Point::new(15, 0);
    assert_eq!(true, collided_with(l1, r1, l2, r2));
}

#[test]
fn test_collision_wall() {
    let snake = Snake::new();
    snake.head_x.set(SNAKE_SPEED+2);
    snake.head_y.set(SNAKE_SPEED+2);
    assert_eq!(false, snake.has_collided_with_any_wall());
    snake.set_direction(SnakeDirection::Left);
    snake.walk();
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
#[test]
fn test_snake_walking() {
    let snake = Snake::new();
    snake.head_x.set(0);
    snake.head_y.set(0);
    snake.set_direction(SnakeDirection::Right);
    snake.walk();
    assert_eq!(SNAKE_SPEED, snake.head_x.get());
    snake.head_x.set(SNAKE_SPEED);
    snake.head_y.set(0);
    snake.set_direction(SnakeDirection::Left);
    snake.walk();
    assert_eq!(0, snake.head_x.get());
    snake.head_x.set(0);
    snake.head_y.set(SNAKE_SPEED);
    snake.set_direction(SnakeDirection::Up);
    snake.walk();
    assert_eq!(0, snake.head_y.get());
    snake.head_x.set(0);
    snake.head_y.set(0);
    snake.set_direction(SnakeDirection::Down);
    snake.walk();
    assert_eq!(SNAKE_SPEED, snake.head_y.get());
}
