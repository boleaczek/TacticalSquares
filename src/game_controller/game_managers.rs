use std::collections::HashMap;

use crate::algebra_basics::{Coordinates, Size};
use crate::game_data::gameboard::{Gameboard, GameboardObjectOperation};
use crate::game_data::game_object::GameObject;
use crate::game_data::gameboard;
use crate::game_controller::movement_manager::MovementHandler;

pub enum UserInput {
    NoInputCursorPos(Coordinates),
    LeftMouse(Coordinates),
    RightMouse(Coordinates),
    D
}

impl UserInput {
    pub fn get_coordinates_if_mouse_input(&self) -> Option<&Coordinates> {
        match self {
            UserInput::NoInputCursorPos(coordinates) => return Some(coordinates),
            UserInput::LeftMouse(coordinates) => return Some(coordinates),
            UserInput::RightMouse(coordinates) => return Some(coordinates),
            _ => return None
        }
    }
}

pub struct BasicState {
    pub current_selected_id: u32,
    pub external_event: UserInput,
    pub gameboard: Gameboard,
    pub movements: HashMap<u32, MovementHandler>
}

pub trait BasicStateContainer {
    fn get_basic_state(&mut self) -> &mut BasicState;
}

pub fn process_selection<S>(mut state: S) -> S
where S: BasicStateContainer {
    let basic_state = state.get_basic_state();

    let position;
    if let UserInput::LeftMouse(pos) = &basic_state.external_event {
        position = pos
    }
    else {
        return state;
    }

    let gameboard = &basic_state.gameboard;

    let querry = |object_data: &(&u32, &GameObject)| {
        gameboard::check_if_object_area_contains_coordinates(object_data.1, position)
    };

    let object = gameboard.querry_object(querry);
    
    if let Some(object) = object {
        basic_state.current_selected_id = *object.0;
    }

    return state;
}

pub fn process_player_movement<S>(mut state: S) -> S
where S: BasicStateContainer {
    let mut basic_state = state.get_basic_state();

    if let UserInput::RightMouse(destination) = &basic_state.external_event {
        let selected = basic_state.gameboard.get_object_by_id(basic_state.current_selected_id).unwrap();
        let start = selected.position.clone();
        let movement_manager = MovementHandler::start(start, destination.clone());
        basic_state.movements.insert(basic_state.current_selected_id, movement_manager);
    }

    return state;
}

pub fn proces_movement<S>(mut state: S) -> S
where S: BasicStateContainer {
    let basic_state = state.get_basic_state();
    let ids_to_remove = proces_movement_apply(basic_state);

    for id in ids_to_remove {
        basic_state.movements.remove(&id);
    }

    return state;
}

fn proces_movement_apply(state: &mut BasicState) -> Vec<u32> {
    let mut ids_to_remove = Vec::new();

    let movements = &mut state.movements;
    for (id, movement_handler) in movements {
        if let Some(position) = movement_handler.poll_movement() {
            state.gameboard.execute_operation(*id, GameboardObjectOperation::Move(position));
        }
        else {
            ids_to_remove.push(*id);
        }
    }

    return ids_to_remove;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::setup;
    use crate::game_data::game_object::{GameObject, GameObjectType};
    use crate::algebra_basics::{Coordinates, Size};

    #[test]
    fn pprocess_player_movement_right_click_movement_added() {
        let mut state = setup::setup_game_state_with_one_object();
        state.basic_state.external_event = UserInput::RightMouse(Coordinates::new(50.0, 50.0));

        let state = process_player_movement(state);
        let movement_handlers = state.basic_state.movements;
        assert_eq!(movement_handlers.len(), 1);
    }

    #[test]
    fn process_selection_left_click_on_selectable_selected_id_changes() {
        let mut state = setup::setup_game_state_with_one_object();
        let new_object = GameObject::new(GameObjectType::Selectable, Coordinates::new(100.0, 100.0), Size::new(50.0, 50.0));
        
        state.basic_state.gameboard.add_object(new_object);
        state.basic_state.external_event = UserInput::LeftMouse(Coordinates::new(125.0, 125.0));

        state = process_selection(state);
        let selected_id = state.basic_state.current_selected_id;

        assert_eq!(selected_id, 1);
    }

    #[test]
    fn process_selection_manager_left_click_on_non_selectable_selected_id_doesent_change() {
        let mut state = setup::setup_game_state_with_one_object();
        
        state.basic_state.external_event = UserInput::LeftMouse(Coordinates::new(125.0, 125.0));

        state = process_selection(state);
        let selected_id = state.basic_state.current_selected_id;

        assert_eq!(selected_id, 0);
    }
}