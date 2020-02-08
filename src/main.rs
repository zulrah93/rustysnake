extern crate piston_window;
extern crate rand;
use piston_window::*;
mod food;
mod snake;
use food::Food;
use rand::{thread_rng, Rng};
use snake::{Snake, SnakeDirection};
use std::process::exit;

fn main() {
    let snake = Snake::new();
    let mut rng = thread_rng();
    let start_food_x: u8 = rng.gen();
    let start_food_y: u8 = rng.gen();
    let mut food = Food::new(start_food_x, start_food_y);
    let mut window: PistonWindow = WindowSettings::new("Rusty Snake", (256, 256))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;
            match button {
                Keyboard(Key::Up) => {
                    snake.set_direction(SnakeDirection::Up);
                }
                Keyboard(Key::Down) => {
                    snake.set_direction(SnakeDirection::Down);
                }
                Keyboard(Key::Left) => {
                    snake.set_direction(SnakeDirection::Left);
                }
                Keyboard(Key::Right) => {
                    snake.set_direction(SnakeDirection::Right);
                }
                _ => {}
            }
        }
        window.draw_2d(&e, |_c, g, _d| {
            if snake.can_eat(&food) {
                snake.eat(food.clone());
                food.set_x(rng.gen());
                food.set_y(rng.gen());
            }
            if snake.has_collided_with_any_wall() {
                println!("Game Over!");
                exit(0);
            }
            snake.walk();
            snake.render(&_c.transform, g);
            food.render(&_c.transform, g);
        });
    }
}
