use std::collections::HashMap;
use std::cmp::Eq;
use std::boxed::Box;

pub struct Gameboard {
    objects: HashMap<u32, Box<dyn GameboardObject>>,
    last_id: u32
}

impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            objects: HashMap::new(),
            last_id: 0
        }
    }

    pub fn add_object(&mut self, object: impl GameboardObject + 'static) -> u32 {
        self.last_id += 1;
        self.objects.insert(self.last_id, Box::from(object));

        self.last_id
    }

    pub fn remove_object(&mut self, id: u32) -> Result<(), String> {
        if self.objects.contains_key(&id) {
            self.objects.remove(&id);
            return Ok(())
        }

        Err(String::from("Incorrect object id."))
    }

    pub fn get_object(&self, id: u32) -> Result<&Box<dyn GameboardObject>, String> {
        match self.objects.get(&id) {
            Some(object) => Ok(object),
            None => Err(String::from("Incorrect object id."))
        }
    }

    pub fn execute_operation(&mut self, id: u32, operation: GameboardObjectOperation) -> Result<(), String> {
        match self.objects.get_mut(&id) {
            Some(object) => {
                match object.execute_operation(operation) {
                    Ok(_) => return Ok(()),
                    Err(error_message) => return Err(error_message)
                }
            },
            None => {
                Err(String::from("Incorrect object id."))
            }
        }
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
    x: f32,
    y: f32
}

impl Coordinates {
    pub fn new(x: f32, y: f32) -> Coordinates {
        Coordinates {
            x,
            y
        }
    }

    pub fn change(&mut self, new_coordinates: Coordinates) {
        self.x = new_coordinates.x;
        self.y = new_coordinates.y;
    }

    pub fn get(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

pub struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size{
        Size {
            width,
            height
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::setup_board_with_one_object;
    use super::*;
    use crate::mocks::MockGameObject;

    #[test]
    fn add_object_returns_correct_id() {
        let mut gameboard = Gameboard::new();
        
        let mock_object = MockGameObject::new();
        let mut id = gameboard.add_object(mock_object);
        assert_eq!(id, 1);
        
        let mock_object = MockGameObject::new();
        id = gameboard.add_object(mock_object);
        assert_eq!(id, 2);
    }

    #[test]
    fn remove_object_correct_id_returns_ok() {
        let mut gameboard = setup_board_with_one_object();
        let result = gameboard.remove_object(1);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn remove_object_incorrect_id_returns_err() {
        let mut gameboard = setup_board_with_one_object();
        let result = gameboard.remove_object(3);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn execute_operation_correct_id_returns_ok() {
        let mut gameboard = setup_board_with_one_object();
        let result = gameboard.execute_operation(1, GameboardObjectOperation::None);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn execute_operation_incorrect_id_returns_err() {
        let mut gameboard = setup_board_with_one_object();
        let result = gameboard.execute_operation(3, GameboardObjectOperation::None);

        assert_eq!(result.is_err(), true);
    }
    
    #[test]
    fn get_object_correct_id_returns_ok() {
        let gameboard = setup_board_with_one_object();
        let result = gameboard.get_object(1);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn get_object_incorrct_id_returns_err() {
        let gameboard = setup_board_with_one_object();
        let result = gameboard.get_object(2);

        assert_eq!(result.is_err(), true);
    }

    fn setup_gameboard_with_one_character_object() -> Gameboard {
        let mut gameboard = Gameboard::new();
        let character_object = CharacterObject::new(Coordinates::new(0.0, 0.0), Size::new(0.0, 0.0));
        gameboard.add_object(character_object);
        gameboard
    }

    #[test]
    fn character_object_move_operation_returns_ok_coordinates_change() {
        let mut gameboard = setup_gameboard_with_one_character_object();
        let operation = GameboardObjectOperation::Move(Coordinates::new(1.0, 1.0));
        let result = gameboard.execute_operation(1, operation);

        assert_eq!(result.is_ok(), true);
        
        let object = gameboard.get_object(1).unwrap().as_ref();
        let position = object.get_position();
        
        let expected_position = Coordinates::new(1.0, 1.0);

        assert_eq!(position, &expected_position);
    }
}