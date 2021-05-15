extern crate find_folder;
extern crate piston_window;
extern crate rand;
use piston_window::*;
mod food;
mod menu;
mod snake;
mod text;
use food::{Food, FOOD_HEIGHT, FOOD_WIDTH};
use menu::{Menu, MenuOption};
use rand::{thread_rng, Rng};
use snake::{Snake, SnakeDirection, GAME_HEIGHT, GAME_WIDTH};
use std::process::exit;

pub const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 255.0];
const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 255.0];
static mut RANDOM: Option<rand::prelude::ThreadRng> = None;
static mut SCORE: u32 = 0;

fn main() {
    let snake = Snake::new();
    let mut rng = unsafe {
        RANDOM.replace(thread_rng());
        RANDOM.unwrap()
    };
    let start_food_x = rng.gen::<u16>() % ((GAME_WIDTH / (FOOD_WIDTH as u32)) as u16);
    let start_food_y = rng.gen::<u16>() % ((GAME_HEIGHT / (FOOD_HEIGHT as u32)) as u16);
    let mut food = Food::new(start_food_x, start_food_y);
    let mut window: PistonWindow = WindowSettings::new("Rusty Snake", (GAME_WIDTH, GAME_HEIGHT))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build window: {}", e));
    window.set_max_fps(15);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("fonts")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    let menu = Menu::new();
    menu.map_option_to_state(MenuOption::ExitGame, |_s, _f| exit(0));
    menu.map_option_to_state(MenuOption::SinglePlayer, single_player_game);
    menu.map_option_to_state(
        MenuOption::MultiPlayerExistingRoom,
        multipler_player_game_existing_match,
    );
    menu.map_option_to_state(
        MenuOption::MultiplayerRandomRoom,
        multipler_player_game_random_match,
    );
    menu.map_option_to_render_state(MenuOption::SinglePlayer, render_single_player_game);
    menu.map_option_to_render_state(
        MenuOption::MultiPlayerExistingRoom,
        render_multipler_player_game_existing_match,
    );
    menu.map_option_to_render_state(
        MenuOption::MultiplayerRandomRoom,
        render_multipler_player_game_random_match,
    );
    while let Some(e) = window.next() {
        // Event loop
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;
            if menu.is_in_game() {
                if let Some(button) = e.press_args() {
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
                        Keyboard(Key::Space) => {
                            snake.toggle_acceleration();
                        }
                        _ => {}
                    }
                }
            } else {
                match button {
                    Keyboard(Key::Up) => {
                        menu.decrement_menu();
                    }
                    Keyboard(Key::Down) => {
                        menu.increment_menu();
                    }
                    Keyboard(Key::Space) => {
                        menu.enter();
                    }
                    _ => {}
                }
            }
        }
        window.draw_2d(&e, |c, g, d| {
            // Game loop
            menu.execute_state(&snake, &mut food);
            clear(CLEAR_COLOR, g);
            menu.execute_render_state(&c.transform, g, &mut glyphs, &c.draw_state, &snake, &food);
            glyphs.factory.encoder.flush(d);
        });
    }
}

fn single_player_game(snake: &Snake, food: &mut Food) {
    let mut rng = unsafe { RANDOM.unwrap() };
    if snake.has_collided_with_any_wall() || snake.ate_itself() {
        println!("Game Over!");
        exit(0);
    }
    if snake.can_eat(&food) {
        food.set_x(rng.gen::<u16>() % ((GAME_WIDTH / (FOOD_WIDTH as u32)) as u16));
        food.set_y(rng.gen::<u16>() % ((GAME_HEIGHT / (FOOD_HEIGHT as u32)) as u16));
        unsafe {
            SCORE += 100;
        }
        snake.walk(true);
    } else {
        snake.walk(false);
    }
}

#[allow(dead_code)]
fn multipler_player_game_existing_match(_snake: &Snake, _food: &mut Food) {}

#[allow(dead_code)]
fn multipler_player_game_random_match(_snake: &Snake, _food: &mut Food) {}

use piston_window::math::Matrix2d;
fn render_single_player_game(
    transform: &Matrix2d,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    draw_state: &DrawState,
    snake: &Snake,
    food: &Food,
) {
    snake.render(transform, graphics);
    food.render(transform, graphics, false);
    text::TextRenderer::render(
        format!("Score: {}", unsafe { SCORE }),
        16,
        &WHITE,
        &transform.trans(490.0, 20.0),
        graphics,
        glyphs,
        draw_state,
    );
}

#[allow(dead_code)]
fn render_multipler_player_game_existing_match(
    _transform: &Matrix2d,
    _graphics: &mut G2d,
    _glyphs: &mut Glyphs,
    _draw_state: &DrawState,
    _snake: &Snake,
    _food: &Food,
) {
}

#[allow(dead_code)]
fn render_multipler_player_game_random_match(
    _transform: &Matrix2d,
    _graphics: &mut G2d,
    _glyphs: &mut Glyphs,
    _draw_state: &DrawState,
    _snake: &Snake,
    _food: &Food,
) {
}
