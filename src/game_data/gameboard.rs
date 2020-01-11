use std::collections::HashMap;
use std::cmp::Eq;
use std::boxed::Box;

use crate::game_data::game_object;
use crate::game_data::game_object::GameObject;
use crate::game_data::game_object::data::*;

pub enum GameboardObjectOperation {
    None, // for test purposes only
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
}

// fn check_if_gameboard_object_is_selected(object: &Box<dyn GameboardObject>, coordinates: &Coordinates) -> bool {
//     let object_size = object.get_size();
    
//     let object_coordinates = object.get_position();
//     let object_bottom_coordinates = Coordinates::new(object_coordinates.x + object_size.width
//         , object_coordinates.y + object_size.height);

//     if (coordinates.x >= object_coordinates.x && coordinates.x <= object_bottom_coordinates.x) &&
//         (coordinates.y >= object_coordinates.y && coordinates.y <= object_bottom_coordinates.y) {
//         return true;
//     }
//     return false;
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_selectable_object() -> GameObject {
        GameObject::new(GameObjectType::Selectable, Coordinates::new(0.0, 0.0), Size::new(50.0, 50.0))
    }

    fn setup_gameboard_with_selectable_object() -> Gameboard {
        let mut gameboard = Gameboard::new();
        gameboard.add_object(setup_selectable_object());
        gameboard
    }

    #[test]
    fn gameboard_add_object_next_id_changes() {
        let mut gameboard = Gameboard::new();

        let id_pre_add = gameboard.next_id;
        gameboard.add_object(setup_selectable_object());
        let id_after_add = gameboard.next_id;

        assert_ne!(id_pre_add, id_after_add);
    }

    #[test]
    fn gameboard_remove_object_correct_id_object_is_removed() {
        let mut gameboard = setup_gameboard_with_selectable_object();
        gameboard.remove_object(0);

        let get_result = gameboard.game_objects.get(&0);

        assert_eq!(get_result.is_none(), true);
    }

    #[test]
    fn gameboard_get_object_by_id_correct_id_returns_some() {
        let gameboard = setup_gameboard_with_selectable_object();
        let result = gameboard.get_object_by_id(0);

        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn gameboard_get_object_by_id_incorrect_id_returns_none() {
        let gameboard = setup_gameboard_with_selectable_object();
        let result = gameboard.get_object_by_id(1);

        assert_eq!(result.is_none(), true);
    }
}