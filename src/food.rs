use piston_window::math::Matrix2d;
use piston_window::{rectangle, G2d};

pub const FOOD_WIDTH: u8 = 5;
//Note: snake is built off food
pub const FOOD_HEIGHT: u8 = 5;
pub const FOOD_COLOR: [f32; 4] = [255.0, 255.0, 255.0, 255.0]; // Brown with no alpha

#[derive(Clone)]
pub struct Food {
    x: Option<u8>,
    y: Option<u8>,
}

impl Food {
    pub fn new(x: u8, y: u8) -> Self {
        Food {
            x: Some(x),
            y: Some(y),
        }
    }

    #[allow(dead_code)]
    pub fn blank() -> Self {
        Food { x: None, y: None } // Snake food object exists but can't be drawn yet as the coordinates don't exist
    }

    pub fn get_x(&self) -> Option<u8> {
        self.x
    }

    pub fn get_y(&self) -> Option<u8> {
        self.y
    }

    pub fn set_x(&mut self, x: u8) {
        self.x.replace(x);
    }

    pub fn set_y(&mut self, y: u8) {
        self.y.replace(y);
    }

    pub fn render(&self, transform: &Matrix2d, graphics: &mut G2d) {
        if let Some(x) = self.x {
            if let Some(y) = self.y {
                rectangle(
                    FOOD_COLOR,
                    [x as f64, y as f64, FOOD_WIDTH as f64, FOOD_HEIGHT as f64],
                    *transform,
                    graphics,
                );
            }
        }
    }
}
