extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::{Events, EventSettings, EventLoop};
use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::RenderEvent;

use std::collections::HashMap;
use std::time;

pub mod game_controller;
pub mod gameboard_view;
pub mod game_data;
pub mod testing;
pub mod algebra_basics;
pub mod debug_utils;

use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::{GameObject, GameObjectType};
use crate::algebra_basics::{Coordinates, Size};
use crate::game_controller::game_managers::*;
use crate::game_controller::utils::PistonEventTranslator;
use crate::debug_utils::{DebugState, LineObject, ConsoleCommand};

struct MainState {
    basic_state: BasicState,
    debug_state: DebugState
}

impl BasicStateContainer for MainState {
    fn get_basic_state(&mut self) -> &mut BasicState {
        &mut self.basic_state
    }
}

fn build_state() -> MainState {
    let mut gameboard = Gameboard::new();
    let game_object0 = GameObject::new(GameObjectType::Selectable, Coordinates::new(0.0, 0.0), Size::new(50.0, 50.0));
    let game_object1 = GameObject::new(GameObjectType::Selectable, Coordinates::new(60.0, 0.0), Size::new(50.0, 150.0));
    let game_object2 = GameObject::new(GameObjectType::Selectable, Coordinates::new(100.0, 0.0), Size::new(150.0, 50.0));
    gameboard.add_object(game_object0);
    gameboard.add_object(game_object1);
    gameboard.add_object(game_object2);

    let basic_state = BasicState {
        current_selected_id: 0,
        external_event: UserInput::NoInputCursorPos(Coordinates::new(0.0, 0.0)),
        gameboard: gameboard,
        movements: HashMap::new()
    };

    let initial_line = LineObject {
        a: Coordinates::new(0.0, 0.0),
        b: Coordinates::new(0.0, 0.0)
    };

    let debug_state = DebugState {
        debug_line: initial_line,
        debug_prints_enabled: false,
        console_commands_enabled: false,
        last_print_time: time::SystemTime::now(),
        debug_tick_time: time::Duration::new(20, 0),
        last_command: ConsoleCommand::None
    };

    MainState {
        basic_state,
        debug_state
    }
}

fn build_piston_translator() -> PistonEventTranslator {
    PistonEventTranslator {
        current_cursor_pos: Coordinates::new(0.0, 0.0)
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Tactical Squares", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut state = build_state();
    let mut translator = build_piston_translator();

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        state.basic_state.external_event = translator.piston_event_to_internal_user_input(&e);
        state = process_selection(state);
        state = process_player_movement(state);
        state = proces_movement(state);
        
        // debug
        state.debug_state = debug_utils::process_debug_line(&state.basic_state, state.debug_state);
        state.debug_state = debug_utils::process_debug_enabled(&state.basic_state, state.debug_state);
        debug_utils::print_object_positions_and_sizes(&state.basic_state, &mut state.debug_state);
        state.debug_state = debug_utils::process_console_command(state.debug_state);
        state.basic_state = debug_utils::apply_console_command_to_basic_state(state.basic_state, &state.debug_state);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                gameboard_view::render(state.get_basic_state().gameboard.get_all_objects(), &c, g);
                gameboard_view::render_debug(&state.debug_state.debug_line, &c, g);
                clear([1.0; 4], g);
            });
        }
    }
}
