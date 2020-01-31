use std::time;

use crate::algebra_basics::{Coordinates};
use crate::game_controller::game_managers::{BasicState, UserInput};

#[derive(PartialEq, Debug)]
pub struct DebugState {
    pub debug_line: LineObject,
    pub debug_enabled: bool,
    pub last_print_time: time::SystemTime,
    pub debug_tick_time: time::Duration
}

pub trait DebugStateContainer {
    fn get_debug_state(&mut self) -> &mut DebugState;
}

#[derive(PartialEq, Debug)]
pub struct LineObject {
    pub a: Coordinates,
    pub b: Coordinates
}

pub fn process_debug_line(basic_state: &BasicState, mut debug_state: DebugState) -> DebugState {
    let cursor_pos = basic_state.external_event.get_coordinates_if_mouse_input();
    let current_selected_coordinates = current_selected_pos(basic_state);

    if let (Some(cursor_pos), Some(object_pos)) = (cursor_pos, current_selected_coordinates) {
        debug_state.debug_line = LineObject {
            a: object_pos.clone(),
            b: cursor_pos.clone()
        }
    }

    return debug_state;
}

pub fn process_debug_enabled(basic_state: &BasicState, mut debug_state: DebugState) -> DebugState {
    match basic_state.external_event {
        UserInput::D => debug_state.debug_enabled = true,
        _ => {}
    }

    return debug_state;
}

pub fn print_object_positions_and_sizes(basic_state: &BasicState, debug_state: &mut DebugState) {
    if debug_state.debug_enabled == false {
        return;
    }

    if debug_state.last_print_time.elapsed().unwrap() <= debug_state.debug_tick_time {
        return;
    }

    debug_state.last_print_time = time::SystemTime::now();
    let objects = &basic_state.gameboard.game_objects;

    for (id, object) in objects {
        println!("Id: {}, Position: x: {}, y: {} | size: width: {}, height: {}",
            id,
            object.position.x,
            object.position.y,
            object.size.width,
            object.size.height);
    }

    if let Some(coordinates) = basic_state.external_event.get_coordinates_if_mouse_input() {
        println!("Cursor pos: x {}, y: {}", coordinates.x, coordinates.y);
    }
}

fn current_selected_pos(basic_state: &BasicState) -> Option<&Coordinates> {
    let current_id = basic_state.current_selected_id;

    if let Some(object) = basic_state.gameboard.get_object_by_id(current_id) {
        return Some(&object.position);
    }

    return None;
}