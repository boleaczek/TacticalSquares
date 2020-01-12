pub mod setup {
    use crate::game_data::game_object::GameObject;
    use crate::game_data::game_object::data::{GameObjectType, Coordinates, Size};
    use crate::game_data::gameboard::Gameboard;
    use crate::game_controller::game_managers::{GameState, UserInput};

    pub fn setup_selectable_object() -> GameObject {
        GameObject::new(GameObjectType::Selectable, Coordinates::new(0.0, 0.0), Size::new(50.0, 50.0))
    }

    pub fn setup_gameboard_with_selectable_object() -> Gameboard {
        let mut gameboard = Gameboard::new();
        gameboard.add_object(setup_selectable_object());
        gameboard
    }

    pub fn setup_game_state_with_one_object() -> GameState {
        let mut gameboard = setup_gameboard_with_selectable_object();
        GameState {
            current_selected_id: 0,
            external_event: UserInput::NoInputCursorPos(Coordinates::new(0.0, 0.0)),
            gameboard: gameboard
        }
    }
}