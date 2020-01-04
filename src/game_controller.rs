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
    SomethingSelected(u32, (u32, u32)),
    NothingSelected
}

pub struct GameboardController {
    gameboard: Gameboard,
    id_to_position: HashMap<(u32, u32), u32>,
    selection_status: SelectionStatus,
    current_cursor_pos: (u32, u32)
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
            selection_status: SelectionStatus::NothingSelected,
            id_to_position: HashMap::new(),
            current_cursor_pos: (0, 0)
        }
    }
    
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(cursor_pos) = e.mouse_cursor_args() {
            println!("cursor args is some");
            self.current_cursor_pos = (cursor_pos[0] as u32, cursor_pos[1] as u32);
        }

        // println!("checking rest");
        if let Some(button) = e.press_args() {
            println!("event is button");
            match button {
                Button::Mouse(button) => self.onClick(self.current_cursor_pos, button),
                _ => ()
            }
        }
    }

    fn onClick(&mut self, coordinates: (u32, u32), button: MouseButton) {
        match button {
            MouseButton::Left => self.leftClick(coordinates),
            MouseButton::Right => self.rightClick(coordinates),
            _ => {
                println!("not a mouse button");
            }
        }
    }

    fn leftClick(&mut self, coordinates: (u32, u32)) {
        println!("LeftClick on ({},{})", coordinates.0, coordinates.1);
        if let Some(id) = self.id_to_position.get(&coordinates) {
            self.selection_status = SelectionStatus::SomethingSelected(*id, coordinates);
        }
        else {
            self.selection_status = SelectionStatus::NothingSelected;
        }
    }

    fn rightClick(&mut self, coordinates: (u32, u32)) {
        println!("RightClick on ({},{})", coordinates.0, coordinates.1);
        match self.selection_status {
            SelectionStatus::SomethingSelected(id, current_coordinates) => {
                let operation = GameboardObjectOperation::Move(Coordinates::new(coordinates.0 as f32, coordinates.1 as f32));
                self.gameboard.execute_operation(id, operation, GameObjectType::Selectable).unwrap();
                self.id_to_position.remove(&coordinates);
                self.selection_status = SelectionStatus::SomethingSelected(id, coordinates);
                self.id_to_position.insert(coordinates, id);
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
        let gameboard = Gameboard::new();
        let mut game_controller = GameboardController::new(gameboard);
        game_controller.id_to_position.insert((0, 0), 1);
        game_controller
    }

    #[test]
    fn nothing_selected_on_left_click_coordinates_with_something_selection_status_changes() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick((0, 0), MouseButton::Left);
        
        let expected_status = SelectionStatus::SomethingSelected(1, (0,0));
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn nothing_selected_on_left_click_coordinates_with_nothing_selection_doesent_change() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick((1, 1), MouseButton::Left);
        
        let expected_status = SelectionStatus::NothingSelected;
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn something_selected_on_left_click_coordinates_with_something_selection_status_changes() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.id_to_position.insert((1, 1), 2);
        game_controller.onClick((1, 1), MouseButton::Left);
        
        let expected_status = SelectionStatus::SomethingSelected(2, (1,1));
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn something_selected_on_left_click_coordinates_with_nothing_status_changes_to_nothing() {
        let mut game_controller = setup_gameboard_controller_with_one_id();
        game_controller.onClick((1, 1), MouseButton::Left);
        
        let expected_status = SelectionStatus::NothingSelected;
        assert_eq!(game_controller.selection_status, expected_status);
    }

    #[test]
    fn something_selected_on_right_click_coordinates_change() {
        use gameboard::{CharacterObject, Size};
        
        let mut game_controller = setup_gameboard_controller_with_one_id();
        let character_object = CharacterObject::new(Coordinates::new(0.0, 0.0), Size::new(0.0, 0.0));
        game_controller.gameboard.add_object(GameObjectType::Selectable, character_object);

        game_controller.leftClick((0,0));
        game_controller.onClick((1, 1), MouseButton::Right);

        let new_coordinates = game_controller.id_to_position.get(&(1,1));
        let expected_status = Some(&1);
        assert_eq!(new_coordinates, expected_status);
    }
}
