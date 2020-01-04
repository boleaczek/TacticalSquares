use std::collections::HashMap;
use std::cmp::Eq;
use std::boxed::Box;

pub struct Gameboard {
    game_obiects: HashMap<GameObjectType, HashMap<u32, Box<dyn GameboardObject>>>,
    id_to_object_type: HashMap<u32, GameObjectType>,
    last_id: u32
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GameObjectType {
    Static,
    Interactable,
    Selectable
}

impl Gameboard {
    pub fn new() -> Gameboard {
        let mut game_obiects = HashMap::new();
        game_obiects.insert(GameObjectType::Static, HashMap::new());
        game_obiects.insert(GameObjectType::Interactable, HashMap::new());
        game_obiects.insert(GameObjectType::Selectable, HashMap::new());

        Gameboard {
            game_obiects: game_obiects,
            id_to_object_type: HashMap::new(),
            last_id: 0
        }
    }

    fn get_object_type(id_to_object_type: &HashMap<u32, GameObjectType>
        , id: u32) -> Option<&GameObjectType> {
        return id_to_object_type.get(&id);
    }

    pub fn add_object(&mut self, object_type: GameObjectType, object: impl GameboardObject + 'static) -> u32 {
        self.last_id += 1;
        
        let obiects_map = self.game_obiects.get_mut(&object_type).unwrap();
        self.id_to_object_type.insert(self.last_id, object_type);
        
        obiects_map.insert(self.last_id, Box::from(object));
        
        self.last_id
    }

    pub fn remove_object(&mut self, id: u32) {
        // let object_type = self.get_object_type(id);
        if let Some(object_type) = Gameboard::get_object_type(&self.id_to_object_type, id) {
            let obiects_map = self.game_obiects.get_mut(object_type).unwrap();
            obiects_map.remove(&id);
        }
    }

    pub fn get_object(&self, id: u32) -> Option<&Box<dyn GameboardObject>> {
        if let Some(object_type) = Gameboard::get_object_type(&self.id_to_object_type, id) {
            let obiects_map = self.game_obiects.get(&object_type).unwrap();
            return obiects_map.get(&id);
        }

        return None
    }

    pub fn execute_operation(&mut self, id: u32, operation: GameboardObjectOperation, object_type: GameObjectType) -> Result<(), String> {
        let desired_objects = self.game_obiects.get_mut(&object_type).unwrap();
        
        return Gameboard::try_to_execute_for_object_type(id, operation, desired_objects);
    }

    fn try_to_execute_for_object_type(id: u32, 
        operation: GameboardObjectOperation, 
        object_collection: &mut HashMap<u32, Box<dyn GameboardObject>>) -> Result<(), String> {
            if let Some(object) = object_collection.get_mut(&id) {
                match object.execute_operation(operation) {
                    Ok(_) => return Ok(()),
                    Err(error_message) => return Err(error_message)
                }
            }

            Err(String::from("No such object"))
    }

    pub fn is_selected(&self, cursor_pos: Coordinates) -> Option<u32> {
        let selectable_objects = self.game_obiects.get(&GameObjectType::Selectable).unwrap();
        
        for object in selectable_objects {
            if check_if_gameboard_object_is_selected(&object.1, &cursor_pos) {
                return Some(*object.0)
            }
        }
        None
    }
}

pub trait GameboardObject {
    fn execute_operation(&mut self, operation: GameboardObjectOperation) -> Result<(), String>;
    fn get_position(&self) -> &Coordinates;
    fn get_size(&self) -> &Size;
}

pub enum GameboardObjectOperation {
    None, // for test purposes only
    Move(Coordinates)
}

pub struct CharacterObject {
    position: Coordinates,
    size: Size
}

fn check_if_gameboard_object_is_selected(object: &Box<dyn GameboardObject>, coordinates: &Coordinates) -> bool {
    let object_size = object.get_size();
    
    let object_coordinates = object.get_position();
    let object_bottom_coordinates = Coordinates::new(object_coordinates.x + object_size.width
        , object_coordinates.y + object_size.height);

    if (coordinates.x >= object_coordinates.x && coordinates.x <= object_bottom_coordinates.x) &&
        (coordinates.y >= object_coordinates.y && coordinates.y <= object_bottom_coordinates.y) {
        return true;
    }
    return false;
}

impl CharacterObject {
    pub fn new(position: Coordinates, size: Size) -> CharacterObject {
        CharacterObject {
            position,
            size
        }
    }

    fn change_position(&mut self, new_position: Coordinates) {
        self.position.change(new_position);
    }
}

impl GameboardObject for CharacterObject {
    fn execute_operation(&mut self, operation: GameboardObjectOperation) -> Result<(), String> {
        match operation {
            GameboardObjectOperation::Move(coordinates) => {
                self.change_position(coordinates);
                return Ok(())
            }
            _ => return Ok(())
        }
    }

    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_size(&self) -> &Size {
        &self.size
    }
}

#[derive(PartialEq, Debug)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64
}

impl Coordinates {
    pub fn new(x: f64, y: f64) -> Coordinates {
        Coordinates {
            x,
            y
        }
    }

    pub fn change(&mut self, new_coordinates: Coordinates) {
        self.x = new_coordinates.x;
        self.y = new_coordinates.y;
    }

    pub fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

pub struct Size {
    width: f64,
    height: f64
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size{
        Size {
            width,
            height
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::setup_board_with_one_selectable_object;
    use super::*;
    use crate::mocks::MockGameObject;

    #[test]
    fn add_object_returns_correct_id() {
        let mut gameboard = Gameboard::new();
        
        let mock_object = MockGameObject::new();
        let mut id = gameboard.add_object(GameObjectType::Selectable, mock_object);
        assert_eq!(id, 1);
    }

    #[test]
    fn execute_operation_correct_id_returns_ok() {
        let mut gameboard = setup_board_with_one_selectable_object();
        let result = gameboard.execute_operation(1, GameboardObjectOperation::None, GameObjectType::Selectable);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn execute_operation_incorrect_id_returns_err() {
        let mut gameboard = setup_board_with_one_selectable_object();
        let result = gameboard.execute_operation(3, GameboardObjectOperation::None, GameObjectType::Selectable);

        assert_eq!(result.is_err(), true);
    }
    
    #[test]
    fn get_object_correct_id_returns_some() {
        let gameboard = setup_board_with_one_selectable_object();
        let result = gameboard.get_object(1);

        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn get_object_incorrct_id_returns_none() {
        let gameboard = setup_board_with_one_selectable_object();
        let result = gameboard.get_object(2);

        assert_eq!(result.is_none(), true);
    }

    fn setup_gameboard_with_one_character_object() -> Gameboard {
        let mut gameboard = Gameboard::new();
        let character_object = CharacterObject::new(Coordinates::new(0.0, 0.0), Size::new(0.0, 0.0));
        gameboard.add_object(GameObjectType::Selectable ,character_object);
        gameboard
    }

    #[test]
    fn character_object_move_operation_returns_ok_coordinates_change() {
        let mut gameboard = setup_gameboard_with_one_character_object();
        let operation = GameboardObjectOperation::Move(Coordinates::new(1.0, 1.0));
        let result = gameboard.execute_operation(1, operation, GameObjectType::Selectable);

        assert_eq!(result.is_ok(), true);
        
        let object = gameboard.get_object(1).unwrap().as_ref();
        let position = object.get_position();
        
        let expected_position = Coordinates::new(1.0, 1.0);

        assert_eq!(position, &expected_position);
    }

    // mock position x: 0.0, y: 0.0
    // mock size width: 0.0, height: 0.0
    #[test]
    fn is_selected_coordinates_with_object_provided_object_is_returned() {
        let gameboard = setup_board_with_one_selectable_object();
        let coordinates = Coordinates::new(0.0, 0.0);

        let result = gameboard.is_selected(coordinates);
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn is_selected_coordinates_with_no_object_provided_object_is_not_returned() {
        let gameboard = setup_board_with_one_selectable_object();
        let coordinates = Coordinates::new(60.0, 60.0);

        let result = gameboard.is_selected(coordinates);
        assert_eq!(result.is_none(), true);
    }
}