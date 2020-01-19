use piston::input::GenericEvent;
use piston::input::{Button, MouseButton};

use crate::gameboard;
use crate::gameboard::Gameboard;
use crate::gameboard::Coordinates;
use crate::gameboard::Size;
use crate::gameboard::GameboardObjectOperation;
use crate::gameboard::GameObjectType;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum SelectionStatus {
    SomethingSelected(u32),
    NothingSelected
}

#[derive(PartialEq, Debug)]
enum MovementDirection {
    Up,
    Down,
    None
}

#[derive(PartialEq, Debug)]
struct MovementManager {
    current_position: Coordinates,
    destination: Coordinates,
    vector: Coordinates,
    increment: f64,
    m: f64,
    b: f64,
    count: u32,
    direction_x: MovementDirection,
    direction_y: MovementDirection,
    pub id: u32
}

impl MovementManager {
    pub fn start_move(current_position: Coordinates, destination: Coordinates, id: u32) -> MovementManager {
        let direction_x = MovementManager::determine_direction(current_position.x, destination.x);
        let direction_y = MovementManager::determine_direction(current_position.y, destination.y);

        let m = (destination.y - current_position.y) / (destination.x - current_position.x);
        let b = ((current_position.x * m - current_position.y)) * (-1.0);
        let increment = (destination.x - current_position.x) / 100.0;
        let vector_x = destination.x - current_position.x;
        let vector_y = destination.y - current_position.y;
        let vector_c = ((vector_x * vector_x) + (vector_y * vector_y)).sqrt();
        let vector = Coordinates::new(vector_x / vector_c, vector_y / vector_c);
        println!("start: x: {}, y: {}", current_position.x, current_position.y);
        println!("stop: x: {}, y: {}", destination.x, destination.y);
        println!("vector: x: {}, y: {}", vector.x, vector.y);

        MovementManager {
            current_position,
            destination,
            vector,
            increment,
            m,
            b,
            count: 0,
            direction_x,
            direction_y,
            id
        }
    }

    pub fn pool_move_command(&mut self) -> Option<GameboardObjectOperation> {
        // if self.count > 100 {
        //     self.direction_x = MovementDirection::None;
        //     self.direction_y = MovementDirection::None;
        //     return None
        // }
        self.count += 1;
        // let increment = (self.destination.x - self.current_position.x) / 1000.0;
        if self.check_reached_status(self.vector.x) && self.check_reached_status(self.vector.y) {
            return None;
        }

        // println!("Moves: {}, inc {}", self.count, increment);
        // println!("current y: {}", self.current_position.y);
            
        let next_x = self.current_position.x + self.vector.x;
        let next_y = self.current_position.y + self.vector.y;
        // println!("x: {}, y: {}, m: {}, b: {}", next_x, next_y, self.m, self.b);

        let next_position = Coordinates::new(next_x, next_y);
        self.current_position = next_position.clone();
        println!("Ticks: {}", self.count);
        Some(GameboardObjectOperation::Move(next_position))
        
    }

    fn check_reached_status(&mut self, increment: f64) -> bool {
MovementManager::is_reached(&self.direction_y, self.current_position.y, self.destination.y, increment) &&
        MovementManager::is_reached(&self.direction_x, self.current_position.x, self.destination.x, increment)
    }

    fn is_reached(direction: &MovementDirection, current_position: f64, destination: f64, increment: f64) -> bool {
        if direction == &MovementDirection::Down {
            if current_position <= destination{
                return true;
            }
        }
        else {
            if current_position >= destination{
                return true;
            }
        }
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

pub struct GameboardController {
    pub gameboard: Gameboard,
    selection_status: SelectionStatus,
    movement_status: Option<MovementManager>,
    current_cursor_pos: Coordinates
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
            selection_status: SelectionStatus::NothingSelected,
            movement_status: None,
            current_cursor_pos: Coordinates::new(0.0, 0.0)
        }
    }
    
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(cursor_pos) = e.mouse_cursor_args() {
            self.current_cursor_pos = Coordinates::new(cursor_pos[0], cursor_pos[1]);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(button) => {
                    let pos = Coordinates::new(self.current_cursor_pos.x, self.current_cursor_pos.y);
                    self.onClick(pos, button)
                },
                _ => ()
            }
        }

        if let Some(movement_manager) = &mut self.movement_status {
            if let Some(move_command) = movement_manager.pool_move_command() {
                self.gameboard.execute_operation(movement_manager.id, move_command, GameObjectType::Selectable).unwrap();
            }
        }
    }

    fn onClick(&mut self, coordinates: Coordinates, button: MouseButton) {
        match button {
            MouseButton::Left => self.leftClick(coordinates),
            MouseButton::Right => self.rightClick(coordinates),
            _ => {}
        }
    }

    fn leftClick(&mut self, cursor_pos: Coordinates) {
        if let Some(id) = self.gameboard.is_selected(cursor_pos) {
            self.selection_status = SelectionStatus::SomethingSelected(id);
        }
        else {
            self.selection_status = SelectionStatus::NothingSelected;
        }
    }

    fn rightClick(&mut self, cursor_pos: Coordinates) {
        match self.selection_status {
            SelectionStatus::SomethingSelected(id) => {
                let object = self.gameboard.get_object(id).unwrap();
                let current_pos = object.get_position();
                let size = object.get_size();
                
                let destination = get_middle_point(&cursor_pos, size);

                let movement_manager = MovementManager::start_move(current_pos.clone(), destination, id);
                self.movement_status = Some(movement_manager);
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
