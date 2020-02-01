use std::time;
use std::io;
use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::num;

use crate::algebra_basics::{Coordinates};
use crate::game_controller::game_managers::{BasicState, UserInput};

#[derive(PartialEq, Debug)]
pub enum ConsoleCommand {
    None,
    Disable,
    MoveObject{id: u32, new_position: Coordinates}
}

#[derive(PartialEq, Debug)]
pub struct DebugState {
    pub debug_line: LineObject,
    pub debug_prints_enabled: bool,
    pub console_commands_enabled: bool,
    pub last_print_time: time::SystemTime,
    pub debug_tick_time: time::Duration,
    pub last_command: ConsoleCommand
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
        UserInput::D => debug_state.debug_prints_enabled = true,
        UserInput::C => debug_state.console_commands_enabled = true,
        _ => {}
    }

    return debug_state;
}

pub fn print_object_positions_and_sizes(basic_state: &BasicState, debug_state: &mut DebugState) {
    if debug_state.debug_prints_enabled == false {
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

enum ConsoleCommandError{
    ParsingError,
    NotEnoughArguments{expected: usize, got: usize},
    UnknownCommand
}

impl fmt::Display for ConsoleCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ConsoleCommandError::ParsingError => write!(f, "Incorrect argument provided"),
            ConsoleCommandError::NotEnoughArguments{expected, got} => write!(f, "Not enough arguments, expected: {}, got: {}", expected, got),
            ConsoleCommandError::UnknownCommand => write!(f, "Unknown command"),
        }
    }
}

pub fn process_console_command(mut debug_state: DebugState) -> DebugState {
    if !debug_state.console_commands_enabled {
        return debug_state;
    }

    if debug_state.last_command == ConsoleCommand::Disable {
        debug_state.console_commands_enabled = false;
        return debug_state;
    }

    let mut input = String::new();
    let command;
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            command = parse_command(input);
        }
        Err(error) => {
            command = Ok(ConsoleCommand::None);
            println!("error: {}", error)
        }
    }

    if let Ok(command) = command {
        debug_state.last_command = command;
    }
    else if let Err(error) = command {
        println!("Error: {}", error);
    }

    return debug_state;
}

fn parse_command(mut command: String) -> Result<ConsoleCommand, ConsoleCommandError>{
    command.remove(command.len() - 1);
    
    let args: Vec<&str> = command.split(" ").collect();
    
    if args.len() == 0 {
        return Ok(ConsoleCommand::None);
    }

    match args[0] {
        "exit" => return Ok(ConsoleCommand::Disable),
        "move" => {
            return Ok(ConsoleCommand::None);
        }
        _ => return Err(ConsoleCommandError::UnknownCommand)
    }
}

fn parse_arg<T: FromStr>(args: &[&str], arg_position: usize) -> Result<T, ConsoleCommandError> {
    if let Some(arg) = args.get(arg_position) {
        let parsing_result = T::from_str(arg);
        if let Ok(parsed_arg) =  parsing_result {
            return Ok(parsed_arg);
        }

        return Err(ConsoleCommandError::ParsingError);
    }

    return Err(ConsoleCommandError::NotEnoughArguments{expected: arg_position, got: arg_position - 1});
} 