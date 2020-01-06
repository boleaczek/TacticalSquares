extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::{Events, EventSettings, EventLoop};
use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::RenderEvent;

pub mod game_controller;
pub mod gameboard_view;
pub mod gameboard;
pub mod mocks;

fn build_controller() -> game_controller::GameboardController {
    let mut gameboard = gameboard::Gameboard::new();
    let gameboard_object0 = gameboard::CharacterObject::new(gameboard::Coordinates::new(0.0, 0.0), gameboard::Size::new(50.0, 50.0));
    let gameboard_object1 = gameboard::CharacterObject::new(gameboard::Coordinates::new(70.0, 0.0), gameboard::Size::new(50.0, 50.0));
    gameboard.add_object(gameboard::GameObjectType::Selectable, gameboard_object0);
    gameboard.add_object(gameboard::GameObjectType::Selectable, gameboard_object1);
    game_controller::GameboardController::new(gameboard)
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Tactical Squares", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut gameboard_controller = build_controller();

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                gameboard_view::render(gameboard_controller.gameboard.get_all_objects(), &c, g);
                clear([1.0; 4], g);
            });
        }
    }
}
