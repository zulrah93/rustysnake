extern crate piston_window;
extern crate rand;
extern crate find_folder;
use piston_window::*;
mod food;
mod snake;
mod text;
use food::{Food, FOOD_WIDTH, FOOD_HEIGHT};
use rand::{thread_rng, Rng};
use snake::{Snake, SnakeDirection, GAME_WIDTH, GAME_HEIGHT};
use std::process::exit;
use text::TextRenderer;
pub const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 255.0]; // Red like an 🍎

fn main() {
    let snake = Snake::new();
    let mut rng = thread_rng();
    let start_food_x = rng.gen::<u16>() % ((GAME_WIDTH / (FOOD_WIDTH as u32)) as u16);
    let start_food_y = rng.gen::<u16>() % ((GAME_HEIGHT / (FOOD_HEIGHT as u32)) as u16);
    let mut food = Food::new(start_food_x, start_food_y);
    let mut score : u32 = 0;
    let mut window: PistonWindow = WindowSettings::new("Rusty Snake", (GAME_WIDTH, GAME_HEIGHT))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build window: {}", e));
    window.set_max_fps(15);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("fonts").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
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
        window.draw_2d(&e, |c, g, d| {
            if snake.has_collided_with_any_wall() || snake.ate_itself() {
                println!("Game Over!");
                exit(0);
            }
            if snake.can_eat(&food) {
                food.set_x(rng.gen::<u16>() % ((GAME_WIDTH / (FOOD_WIDTH as u32)) as u16));
                food.set_y(rng.gen::<u16>() % ((GAME_HEIGHT / (FOOD_HEIGHT as u32)) as u16));
                score += 100;
                snake.walk(true);
            }
            else {
                snake.walk(false);
            }            
            snake.render(&c.transform, g);
            food.render(&c.transform, g, false);
            TextRenderer::render(format!("Score: {}", score), &WHITE,  &c.transform.trans(490.0, 20.0), g, &mut glyphs, &c.draw_state);
            glyphs.factory.encoder.flush(d);
        });
    }
}
