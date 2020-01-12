use std::collections::HashMap;
use std::cmp::Eq;
use std::boxed::Box;

use crate::game_data::game_object;
use crate::game_data::game_object::GameObject;
use crate::game_data::game_object::data::*;

pub enum GameboardObjectOperation {
    Move(Coordinates)
}

pub struct Gameboard {
    game_objects: HashMap<u32, GameObject>,
    next_id: u32
}

impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            game_objects: HashMap::new(),
            next_id: 0
        }
    }

    pub fn add_object(&mut self, object: GameObject) -> u32 {
        self.game_objects.insert(self.next_id, object);

        let current_id = self.next_id;
        self.next_id +=1;

        return current_id;
    }

    pub fn remove_object(&mut self, id: u32) {
        self.game_objects.remove(&id);
    }

    pub fn get_object_by_id(&self, id: u32) -> Option<&GameObject> {
        self.game_objects.get(&id)
    }

    pub fn querry_object<Q>(&self, querry: Q) -> Option<(&u32, &GameObject)> 
    where for<'r> Q: FnMut(&'r (&u32, &GameObject)) -> bool {
        self.game_objects.iter().find(querry)
    }

    pub fn execute_operation(&mut self, id: u32, operation: GameboardObjectOperation) {
        if let Some(object) = self.game_objects.get_mut(&id) {
            match operation {
                GameboardObjectOperation::Move(new_position) => object.position = new_position
            }
        }
    }
}

fn check_if_object_area_contains_coordinates(object: &GameObject, coordinates: &Coordinates) -> bool {
    let object_size = &object.size;
    let object_position = &object.position;
    
    let object_bottom_position = Coordinates::new(object_position.x + object_size.width
        , object_position.y + object_size.height);

    if (coordinates.x >= object_position.x && coordinates.x <= object_bottom_position.x) &&
        (coordinates.y >= object_position.y && coordinates.y <= object_bottom_position.y) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::setup;

    #[test]
    fn gameboard_add_object_next_id_changes() {
        let mut gameboard = Gameboard::new();

        let id_pre_add = gameboard.next_id;
        gameboard.add_object(setup::setup_selectable_object());
        let id_after_add = gameboard.next_id;

        assert_ne!(id_pre_add, id_after_add);
    }

    #[test]
    fn gameboard_remove_object_correct_id_object_is_removed() {
        let mut gameboard = setup::setup_gameboard_with_selectable_object();
        gameboard.remove_object(0);

        let get_result = gameboard.game_objects.get(&0);

        assert_eq!(get_result.is_none(), true);
    }

    #[test]
    fn gameboard_get_object_by_id_correct_id_returns_some() {
        let gameboard = setup::setup_gameboard_with_selectable_object();
        let result = gameboard.get_object_by_id(0);

        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn gameboard_get_object_by_id_incorrect_id_returns_none() {
        let gameboard = setup::setup_gameboard_with_selectable_object();
        let result = gameboard.get_object_by_id(1);

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn gameboard_execute_operation_move_objects_position_changes() {
        let mut gameboard = setup::setup_gameboard_with_selectable_object();
        let new_position = Coordinates::new(50.0, 50.0);
        let operation = GameboardObjectOperation::Move(new_position);

        gameboard.execute_operation(0, operation);
        let object = gameboard.game_objects.get(&0).unwrap();
        let expected_position = Coordinates::new(50.0, 50.0);

        assert_eq!(object.position, expected_position);
    }

    #[test]
    fn check_if_object_area_contains_coordinates_correct_coordinates_returns_true() {
        let coordinates = Coordinates::new(25.0, 25.0);
        let object = setup::setup_selectable_object();

        let result = check_if_object_area_contains_coordinates(&object, &coordinates);

        assert_eq!(result, true);
    }

    #[test]
    fn check_if_object_area_contains_coordinates_incorrect_coordinates_returns_false() {
        let coordinates = Coordinates::new(60.0, 60.0);
        let object = setup::setup_selectable_object();

        let result = check_if_object_area_contains_coordinates(&object, &coordinates);

        assert_eq!(result, false);
    }
}