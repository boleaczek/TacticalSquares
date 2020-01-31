use crate::algebra_basics::{Coordinates};
use crate::game_controller::game_managers::{BasicStateContainer, UserInput};

#[derive(PartialEq, Debug)]
pub struct DebugState {
    debug_line: LineObject
}

pub trait DebugStateContainer {
    fn get_debug_state(&mut self) -> &mut DebugState;
}

#[derive(PartialEq, Debug)]
pub struct LineObject {
    pub a: Coordinates,
    pub b: Coordinates
}

pub fn process_debug<S>(state: S) -> S
where S: DebugStateContainer + BasicStateContainer {
    

    return state;
}