use crate::food::Food;
use crate::snake::Snake;
use crate::text::TextRenderer;
use piston_window::math::Matrix2d;
use piston_window::*;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

const GREEN: [f32; 4] = [0.0, 255.0, 0.0, 255.0]; // Green

pub type MenuOptionCallback = fn(snake: &Snake, food: &mut Food);
pub type RenderCallback = fn(
    transform: &Matrix2d,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    draw_state: &DrawState,
    snake: &Snake,
    food: &Food,
);

pub struct Menu {
    currently_selected_state: Cell<Option<MenuOption>>,
    states_reference: RefCell<HashMap<MenuOption, MenuOptionCallback>>,
    render_states_reference: RefCell<HashMap<MenuOption, RenderCallback>>,
    option_index: Cell<u8>,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            currently_selected_state: Cell::new(None),
            states_reference: RefCell::new(HashMap::new()),
            render_states_reference: RefCell::new(HashMap::new()),
            option_index: Cell::new(0),
        }
    }

    pub fn increment_menu(&self) {
        self.option_index.set((self.option_index.get() + 1) % 4);
    }

    pub fn decrement_menu(&self) {
        let new_value = if self.option_index.get() == 0 {
            3
        } else {
            self.option_index.get() - 1
        };
        self.option_index.set(new_value);
    }

    pub fn enter(&self) {
        self.currently_selected_state
            .set(convert_index_to_menuoption(self.option_index.get()));
    }

    pub fn is_in_game(&self) -> bool {
        if let Some(state) = self.currently_selected_state.get() {
            state != MenuOption::ExitGame
        } else {
            false
        }
    }

    pub fn map_option_to_state(&self, option: MenuOption, state: MenuOptionCallback) {
        let mut states = self.states_reference.borrow_mut();
        states.insert(option, state);
    }

    pub fn map_option_to_render_state(&self, option: MenuOption, state: RenderCallback) {
        let mut states = self.render_states_reference.borrow_mut();
        states.insert(option, state);
    }

    pub fn execute_state(&self, snake: &Snake, food: &mut Food) {
        if let Some(option) = self.currently_selected_state.get() {
            let states = self.states_reference.borrow();
            states[&option](snake, food);
        }
    }

    pub fn execute_render_state(
        &self,
        transform: &Matrix2d,
        graphics: &mut G2d,
        glyphs: &mut Glyphs,
        draw_state: &DrawState,
        snake: &Snake,
        food: &Food,
    ) {
        if let Some(option) = self.currently_selected_state.get() {
            let states = self.render_states_reference.borrow();
            states[&option](transform, graphics, glyphs, draw_state, snake, food);
        // Call the callback
        } else {
            // Render Menu
            TextRenderer::render(
                format!(
                    "{}Single Player",
                    if self.option_index.get() == 0 {
                        "• "
                    } else {
                        ""
                    }
                ),
                30,
                &GREEN,
                &transform.trans(350.0, 300.0),
                graphics,
                glyphs,
                draw_state,
            );
            TextRenderer::render(
                format!(
                    "{}MultiPlayer Random Join",
                    if self.option_index.get() == 1 {
                        "• "
                    } else {
                        ""
                    }
                ),
                30,
                &GREEN,
                &transform.trans(350.0, 400.0),
                graphics,
                glyphs,
                draw_state,
            );
            TextRenderer::render(
                format!(
                    "{}MultiPlayer Existing Room",
                    if self.option_index.get() == 2 {
                        "• "
                    } else {
                        ""
                    }
                ),
                30,
                &GREEN,
                &transform.trans(350.0, 500.0),
                graphics,
                glyphs,
                draw_state,
            );
            TextRenderer::render(
                format!(
                    "{}Exit",
                    if self.option_index.get() == 3 {
                        "• "
                    } else {
                        ""
                    }
                ),
                30,
                &GREEN,
                &transform.trans(350.0, 600.0),
                graphics,
                glyphs,
                draw_state,
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MenuOption {
    SinglePlayer = 0,
    MultiplayerRandomRoom = 1,
    MultiPlayerExistingRoom = 2,
    ExitGame = 3,
}

fn convert_index_to_menuoption(index: u8) -> Option<MenuOption> {
    match index {
        0 => Some(MenuOption::SinglePlayer),
        1 => Some(MenuOption::MultiplayerRandomRoom),
        2 => Some(MenuOption::MultiPlayerExistingRoom),
        3 => Some(MenuOption::ExitGame),
        _ => None,
    }
}
