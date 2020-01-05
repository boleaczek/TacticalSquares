use piston::input::GenericEvent;
use piston::input::{Button, MouseButton};

use crate::gameboard;
use crate::gameboard::Gameboard;
use crate::gameboard::Coordinates;
use crate::gameboard::GameboardObjectOperation;
use crate::gameboard::GameObjectType;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum SelectionStatus {
    SomethingSelected(u32),
    NothingSelected
}

pub struct GameboardController {
    pub gameboard: Gameboard,
    selection_status: SelectionStatus,
    current_cursor_pos: Coordinates
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
            selection_status: SelectionStatus::NothingSelected,
            current_cursor_pos: Coordinates::new(0.0, 0.0)
        }
    }
    
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(cursor_pos) = e.mouse_cursor_args() {
            println!("cursor args is some");
            self.current_cursor_pos = Coordinates::new(cursor_pos[0], cursor_pos[1]);
        }

        // println!("checking rest");
        if let Some(button) = e.press_args() {
            println!("event is button");
            match button {
                Button::Mouse(button) => {
                    let pos = Coordinates::new(self.current_cursor_pos.x, self.current_cursor_pos.y);
                    self.onClick(pos, button)
                },
                _ => ()
            }
        }
    }

    fn onClick(&mut self, coordinates: Coordinates, button: MouseButton) {
        match button {
            MouseButton::Left => self.leftClick(coordinates),
            MouseButton::Right => self.rightClick(coordinates),
            _ => {
                println!("not a mouse button");
            }
        }
    }

    fn leftClick(&mut self, cursor_pos: Coordinates) {
        println!("LeftClick on ({},{})", cursor_pos.x, cursor_pos.y);
        if let Some(id) = self.gameboard.is_selected(cursor_pos) {
            self.selection_status = SelectionStatus::SomethingSelected(id);
        }
        else {
            self.selection_status = SelectionStatus::NothingSelected;
        }
    }

    fn rightClick(&mut self, cursor_pos: Coordinates) {
        println!("RightClick on ({},{})", cursor_pos.x, cursor_pos.y);
        match self.selection_status {
            SelectionStatus::SomethingSelected(id) => {
                let operation = GameboardObjectOperation::Move(cursor_pos);
                self.gameboard.execute_operation(id, operation, GameObjectType::Selectable).unwrap();
            },
            SelectionStatus::NothingSelected => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    fn setup_gameboard_controller_with_one_id() -> GameboardController{
        let gameboard = mocks::setup_board_with_one_selectable_object();
        let mut game_controller = GameboardController::new(gameboard);

        game_controller
    }

    #[test]
    fn nothing_selected_on_left_click_coordinates_with_something_selection_status_changes() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick(Coordinates::new(0.0, 0.0), MouseButton::Left);
        
        let expected_status = SelectionStatus::SomethingSelected(1);
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn nothing_selected_on_left_click_coordinates_with_nothing_selection_doesent_change() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick(Coordinates::new(60.0, 60.0), MouseButton::Left);
        
        let expected_status = SelectionStatus::NothingSelected;
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn something_selected_on_left_click_coordinates_with_something_selection_status_changes() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick(Coordinates::new(1.0, 1.0), MouseButton::Left);
        
        let expected_status = SelectionStatus::SomethingSelected(1);
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn something_selected_on_left_click_coordinates_with_nothing_status_changes_to_nothing() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick(Coordinates::new(60.0, 60.0), MouseButton::Left);
        
        let expected_status = SelectionStatus::NothingSelected;
        assert_eq!(game_controller.selection_status, expected_status);
    }
}
