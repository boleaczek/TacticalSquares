use crate::gameboard::*;

pub struct MockGameObject {}

impl MockGameObject {
    pub fn new() -> MockGameObject {
        MockGameObject {}
    }
}

impl GameboardObject for MockGameObject {
    fn execute_operation(&mut self, operation: GameboardObjectOperation) -> Result<(), String> {
        return Ok(())
    }

    fn get_position(&self) -> &Coordinates {
        unimplemented!();
    }

    fn get_size(&self) -> &Size {
        unimplemented!();
    }
}

pub fn setup_board_with_one_selectable_object() -> Gameboard {
    let mut gameboard = Gameboard::new();
    
    let mock_object = MockGameObject::new();
    let mut id = gameboard.add_object(GameObjectType::Selectable, mock_object);
    
    gameboard
}