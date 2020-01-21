use crate::algebra_basics::{Coordinates, Size};
use crate::game_data::gameboard::{Gameboard, GameboardObjectOperation};
use crate::game_data::game_object::GameObject;
use crate::game_data::gameboard;
use crate::game_controller::movement_manager::MovementHandler;

pub enum UserInput {
    NoInputCursorPos(Coordinates),
    LeftMouse(Coordinates),
    RightMouse(Coordinates)
}

pub struct BasicState {
    pub current_selected_id: u32,
    pub external_event: UserInput,
    pub gameboard: Gameboard,
    pub movements: Vec<MovementHandler>
}

pub trait BasicStateContainer {
    fn get_basic_state(&mut self) -> &mut BasicState;
}

fn process_selection<S>(mut state: S) -> S
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

fn process_movement<S>(mut state: S) -> S
where S: BasicStateContainer {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::setup;
    use crate::game_data::game_object::{GameObject, GameObjectType};
    use crate::algebra_basics::{Coordinates, Size};

    #[test]
    fn process_movement_right_click_movement_added() {
        let mut state = setup::setup_game_state_with_one_object();
        state.basic_state.external_event = UserInput::RightMouse(Coordinates::new(50.0, 50.0));

        let state = process_movement(state);
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