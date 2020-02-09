use crate::algebra_basics::{Coordinates, Size, Vector, LineEquation};
use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::{GameObject, GameObjectType};

pub struct MovementHandler {
    path: Vec<Coordinates>,
    current_position: Coordinates,
    current_vector: Vector
}

impl MovementHandler {
    pub fn start(current_position: Coordinates, destination: Coordinates, game_objects: &Vec<&GameObject>) -> MovementHandler {
        let vector = Vector::get_vector(&current_position, &destination);
        let vector = Vector::to_unit_vector(&vector);
        let mut path = Vec::new();
        path = pathfinding::find_path(&current_position, &destination, game_objects, path);
        path.push(destination);

        MovementHandler {
            path,
            current_position,
            current_vector: vector
        }
    }

    pub fn poll_movement(&mut self) -> Option<Coordinates> {
        if self.is_reached() {
            return None;
        }

        self.current_position.x += self.current_vector.x;
        self.current_position.y += self.current_vector.y;
        
        Some(self.current_position.clone())
    }

    fn is_reached(&self) -> bool {
        if self.path.is_empty() {
            return true;
        }

        if (self.current_position.x as u32) == (self.path[0].x as u32) 
            && (self.current_position.y as u32) == (self.path[0].y as u32) {
            return true;
        }

        return false;
    }
}



pub mod pathfinding {
use crate::algebra_basics::{Coordinates, LineEquation, Vector, Size, RectangleLineEquations};
use crate::algebra_basics;
use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::{GameObject, GameObjectType};

    enum Node {
        Blocked,
        NonBlocked
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn movement_handler_poll_movement_position_changes_correctly() {
        // let a = Coordinates::new(0.0, 0.0);
        // let b = Coordinates::new(50.0, 50.0);
        // let mut movement_handler = MovementHandler::start(a.clone(), b.clone());

        // let vector = Vector::get_vector(&a, &b);
        // let unit_vector = Vector::to_unit_vector(&vector);

        // let expected = Coordinates::new(a.x + unit_vector.x, a.y + unit_vector.y);
        // let result = movement_handler.poll_movement();

        // assert_eq!(result.unwrap(), expected);
        unimplemented!();
    }

    #[test]
    fn movement_handler_poll_movement_returns_none_when_destination_is_reached() {
        // let a = Coordinates::new(0.0, 0.0);
        // let b = Coordinates::new(10.0, 10.0);
        // let mut movement_handler = MovementHandler::start(a.clone(), b.clone());
        
        // for _ in 0..14 {
        //     movement_handler.poll_movement();
        // }

        // assert_eq!(movement_handler.poll_movement().is_some(), true);
        // assert_eq!(movement_handler.poll_movement().is_none(), true);
        unimplemented!();
    }
}