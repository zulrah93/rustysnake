use crate::food::*;
use piston_window::math::Matrix2d;
use piston_window::{clear, rectangle, G2d};
use std::cell::{Cell, RefCell};
use std::vec::Vec;

pub const SNAKE_SPEED: u8 = 2; // How many pixels to move the snake each frame
const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 255.0];

#[derive(Copy, Clone)]
pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    head_x: Cell<u8>,
    head_y: Cell<u8>,
    direction: Cell<SnakeDirection>,
    food_eaten: RefCell<Vec<Food>>,
}

impl Snake {
    fn get_head_x(&self) -> u8 {
        // Returns the x position of the snake's head
        self.head_x.get()
    }

    fn get_head_y(&self) -> u8 {
        // Returns the y position of the snake's head
        self.head_y.get()
    }

    fn get_tail_x(&self) -> Option<u8> {
        // Returns an Optional x position of the snake's head
        if let Some(tail) = self.food_eaten.borrow().last() {
            tail.get_x()
        } else {
            None
        }
    }

    fn get_tail_y(&self) -> Option<u8> {
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
                food.set_x(x + FOOD_WIDTH);
                food.set_y(y + FOOD_HEIGHT);
                let mut food_eaten = self.food_eaten.borrow_mut();
                food_eaten.push(food.clone());
            }
        }
        else {
            food.set_x(self.head_x.get() + FOOD_WIDTH);
            food.set_y(self.head_y.get() + FOOD_HEIGHT);
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

    pub fn has_collided(&self) -> bool {
        // True if snake has collided with wall or itself this triggers game over if this function returns true
        false
    }

    pub fn render(&self, transform: &Matrix2d, graphics: &mut G2d) {
        //Render snake but clear the screen first
        clear(CLEAR_COLOR, graphics);
        //Render head
        rectangle(
            FOOD_COLOR,
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
            food.render(transform, graphics);
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
                for food in foods.iter_mut() {
                    if let Some(x) = food.get_x() {
                        food.set_x(x - SNAKE_SPEED);
                    }
                }
            }
            SnakeDirection::Right => {
                self.head_x.set(self.head_x.get() + SNAKE_SPEED);
                for food in foods.iter_mut() {
                    if let Some(x) = food.get_x() {
                        food.set_x(x + SNAKE_SPEED);
                    }
                }
            }
            SnakeDirection::Down => {
                self.head_y.set(self.head_y.get() + SNAKE_SPEED);
                for food in foods.iter_mut() {
                    if let Some(y) = food.get_y() {
                        food.set_y(y + SNAKE_SPEED);
                    }
                }
            }
            SnakeDirection::Up => {
                self.head_y.set(self.head_y.get() - SNAKE_SPEED);
                for food in foods.iter_mut() {
                    if let Some(y) = food.get_y() {
                        food.set_y(y - SNAKE_SPEED);
                    }
                }
            }
        }
    }

    pub fn new() -> Self {
        Snake {
            head_x: Cell::new(128 - FOOD_WIDTH),
            head_y: Cell::new(128 - FOOD_HEIGHT),
            direction: Cell::new(SnakeDirection::Left),
            food_eaten: RefCell::new(Vec::new()),
        }
    }
}

struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
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
