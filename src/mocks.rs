use crate::gameboard::*;

pub struct MockGameObject {
    position: Coordinates,
    size: Size
}

impl MockGameObject {
    pub fn new() -> MockGameObject {
        MockGameObject {
            position: Coordinates::new(0.0, 0.0),
            size: Size::new(50.0, 50.0)
        }
    }
}

impl GameboardObject for MockGameObject {
    fn execute_operation(&mut self, operation: GameboardObjectOperation) -> Result<(), String> {
        return Ok(())
    }

    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_size(&self) -> &Size {
        &self.size
    }
}

pub fn setup_board_with_one_selectable_object() -> Gameboard {
    let mut gameboard = Gameboard::new();
    
    let mock_object = MockGameObject::new();
    let mut id = gameboard.add_object(GameObjectType::Selectable, mock_object);
    
    gameboard
}