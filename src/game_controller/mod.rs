pub mod game_managers;
pub mod movement_manager;

pub mod utils {
    use crate::game_controller::game_managers::UserInput;
    use crate::algebra_basics::Coordinates;
    use piston::input::GenericEvent;
    use piston::input::{Button, MouseButton, Key};
    
    pub struct PistonEventTranslator {
        pub current_cursor_pos: Coordinates
    }

    impl PistonEventTranslator {
        pub fn piston_event_to_internal_user_input<E: GenericEvent>(&mut self, e: &E) -> UserInput {
            if let Some(cursor_pos) = e.mouse_cursor_args() {
                self.current_cursor_pos = Coordinates::new(cursor_pos[0], cursor_pos[1]);
            }

            if let Some(button) = e.press_args() {
                match button {
                    Button::Mouse(button) => {
                        match button {
                            MouseButton::Left => return UserInput::LeftMouse(self.current_cursor_pos.clone()),
                            MouseButton::Right => return UserInput::RightMouse(self.current_cursor_pos.clone()),
                            _ => {}
                        }
                        
                    },
                    Button::Keyboard(button) => {
                        match button {
                            Key::D => return UserInput::D,
                            Key::C => return UserInput::C,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }

            return UserInput::NoInputCursorPos(self.current_cursor_pos.clone());
        }
    }
}