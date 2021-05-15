use piston_window::math::Matrix2d;
use piston_window::{rectangle, G2d};

pub const FOOD_WIDTH: u16 = 25;
//Note: snake is built off food
pub const FOOD_HEIGHT: u16 = 25;
pub const FOOD_COLOR: [f32; 4] = [255.0, 0.0, 0.0, 255.0]; // Red like an 🍎
pub const SNAKE_COLOR: [f32; 4] = [0.0, 255.0, 0.0, 255.0]; // Green

#[derive(Clone, Debug)]
pub struct Food {
    x: Option<u16>,
    y: Option<u16>,
}

impl Food {
    pub fn new(x: u16, y: u16) -> Self {
        Food {
            x: Some(x),
            y: Some(y),
        }
    }

    #[allow(dead_code)]
    pub fn blank() -> Self {
        Food { x: None, y: None } // Snake food object exists but can't be drawn yet as the coordinates don't exist
    }

    pub fn get_x(&self) -> Option<u16> {
        self.x
    }

    pub fn get_y(&self) -> Option<u16> {
        self.y
    }

    pub fn set_x(&mut self, x: u16) {
        self.x.replace(x);
    }

    pub fn set_y(&mut self, y: u16) {
        self.y.replace(y);
    }

    pub fn equal(&self, x1: u16, y1: u16) -> bool {
        if let Some(x2) = self.get_x() {
            if let Some(y2) = self.get_y() {
                return x1 == x2 && y1 == y2;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    pub fn render(&self, transform: &Matrix2d, graphics: &mut G2d, part_of_snake: bool) {
        if let Some(x) = self.x {
            if let Some(y) = self.y {
                rectangle(
                    if part_of_snake {
                        SNAKE_COLOR
                    } else {
                        FOOD_COLOR
                    },
                    [
                        (x * FOOD_WIDTH) as f64,
                        (y * FOOD_HEIGHT) as f64,
                        FOOD_WIDTH as f64,
                        FOOD_HEIGHT as f64,
                    ],
                    *transform,
                    graphics,
                );
            }
        }
    }
}
