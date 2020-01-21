pub mod setup {
    use crate::game_data::game_object::{GameObject, GameObjectType};
    use crate::algebra_basics::{Coordinates, Size};
    use crate::game_data::gameboard::Gameboard;
    use crate::game_controller::game_managers::{BasicState, BasicStateContainer, UserInput};

    pub fn setup_selectable_object() -> GameObject {
        GameObject::new(GameObjectType::Selectable, Coordinates::new(0.0, 0.0), Size::new(50.0, 50.0))
    }

    pub fn setup_gameboard_with_selectable_object() -> Gameboard {
        let mut gameboard = Gameboard::new();
        gameboard.add_object(setup_selectable_object());
        gameboard
    }

    pub struct MockMainState {
        pub basic_state: BasicState
    }

    impl BasicStateContainer for MockMainState {
        fn get_basic_state(&mut self) -> &mut BasicState {
            return &mut self.basic_state;
        }
    }

    pub fn setup_game_state_with_one_object() -> MockMainState {
        let mut gameboard = setup_gameboard_with_selectable_object();
        let basic = BasicState {
            current_selected_id: 0,
            external_event: UserInput::NoInputCursorPos(Coordinates::new(0.0, 0.0)),
            gameboard: gameboard,
            movements: vec!()
        };

        MockMainState {
            basic_state: basic
        }
    }
}