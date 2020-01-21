use crate::algebra_basics::{Coordinates, Size};
use crate::game_data::gameboard::{Gameboard, GameboardObjectOperation};
use crate::game_data::game_object::GameObject;
use crate::game_data::gameboard;

pub enum UserInput {
    NoInputCursorPos(Coordinates),
    LeftMouse(Coordinates),
    RightMouse(Coordinates)
}

pub struct BasicState {
    pub current_selected_id: u32,
    pub external_event: UserInput,
    pub gameboard: Gameboard
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



mod movement_handler {
    use crate::algebra_basics::{Coordinates, Size};
    use crate::game_data::gameboard::GameboardObjectOperation;

    #[derive(PartialEq, Debug)]
    enum MovementDirection {
        Up,
        Down,
        None
    }

    struct MovementHandler {
        current_position: Coordinates,
        destination: Coordinates,
        direction_x: MovementDirection,
        direction_y: MovementDirection
    }

    impl MovementHandler {
        pub fn start_move(current_position: Coordinates, destination: Coordinates) -> MovementHandler {
            let direction_x = MovementHandler::determine_direction(current_position.x, destination.x);
            let direction_y = MovementHandler::determine_direction(current_position.y, destination.y);

            MovementHandler {
                current_position,
                destination,
                direction_x,
                direction_y
            }
        }

        pub fn pool_move_command(&mut self) -> Option<GameboardObjectOperation> {
            self.check_reached_status();
            
            let x = self.current_position.x;
            let y = self.current_position.y;

            let move_x = MovementHandler::get_direction(&self.direction_x);
            let move_y = MovementHandler::get_direction(&self.direction_y);

            let next_position = Coordinates::new(x + move_x, y + move_y);

            self.current_position = next_position.clone();

            Some(GameboardObjectOperation::Move(next_position))
        }

        fn check_reached_status(&mut self) {
            if MovementHandler::is_reached(&self.direction_x, self.current_position.x, self.destination.x) {
                self.direction_x = MovementDirection::None;
            }

            if MovementHandler::is_reached(&self.direction_y, self.current_position.y, self.destination.y) {
                self.direction_y = MovementDirection::None;
            }
        }

        fn is_reached(direction: &MovementDirection, current_position: f64, destination: f64) -> bool {
            match direction {
                MovementDirection::Down => {
                    if current_position <= destination {
                        return true;
                    }
                },
                MovementDirection::Up => {
                    if current_position >= destination {
                        return true;
                    }
                },
                MovementDirection::None => return true
            };
            return false;
        }

        fn get_direction(direction: &MovementDirection) -> f64 {
            match direction {
                MovementDirection::Up => return 1.0,
                MovementDirection::Down => return -1.0,
                MovementDirection::None => return 0.0
            }
        }

        fn determine_direction(current_position: f64, destination: f64) -> MovementDirection {
            let movemement_direction;
            
            if current_position > destination {
                movemement_direction = MovementDirection::Down;
            }
            else if current_position < destination{
                movemement_direction = MovementDirection::Up;
            }
            else {
                movemement_direction = MovementDirection::None;
            }

            return movemement_direction;
        }
    }

    fn get_middle_point(position: &Coordinates, size: &Size) -> Coordinates {
        let x_move = size.width / 2.0;
        let y_move = size.height / 2.0;
        
        Coordinates::new(position.x - x_move, position.y - y_move)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::setup;
    use crate::game_data::game_object::{GameObject, GameObjectType};
    use crate::algebra_basics::{Coordinates, Size};

    #[test]
    fn selectable_movement_manager_right_click_coordinates_change() {
        let mut state = setup::setup_game_state_with_one_object();
        state.basic_state.external_event = UserInput::RightMouse(Coordinates::new(50.0, 50.0));

        let state = process_selection(state);
        unimplemented!();
    }

    #[test]
    fn selectable_movement_manager_left_click_on_selectable_selected_id_changes() {
        let mut state = setup::setup_game_state_with_one_object();
        let mut new_object = GameObject::new(GameObjectType::Selectable, Coordinates::new(100.0, 100.0), Size::new(50.0, 50.0));
        
        state.basic_state.gameboard.add_object(new_object);
        state.basic_state.external_event = UserInput::LeftMouse(Coordinates::new(125.0, 125.0));

        state = process_selection(state);
        let mut selected_id = state.basic_state.current_selected_id;

        assert_eq!(selected_id, 1);
    }

    #[test]
    fn selectable_movement_manager_left_click_on_non_selectable_selected_id_doesent_change() {
        let mut state = setup::setup_game_state_with_one_object();
        
        state.basic_state.external_event = UserInput::LeftMouse(Coordinates::new(125.0, 125.0));

        state = process_selection(state);
        let mut selected_id = state.basic_state.current_selected_id;

        assert_eq!(selected_id, 0);
    }
}