pub mod gameboard;

pub mod game_object {
    use crate::algebra_basics::{Coordinates, Size};
    
    pub struct GameObject {
        pub object_type: GameObjectType,
        pub position: Coordinates,
        pub size: Size
    }
    
    impl GameObject {
        pub fn new(object_type: GameObjectType
            , position: Coordinates
            , size: Size) -> GameObject {
                GameObject {
                    object_type,
                    position,
                    size
                }
        }
    }
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum GameObjectType {
        Static,
        Interactable,
        Selectable
    }
}