// use graphics::types::Color;
// use graphics::{Context, Graphics};

// use std::collections::HashMap;

// use crate::gameboard;
// use crate::gameboard::GameboardObject;

// pub fn render<G: Graphics>(objects: Vec<&Box<dyn GameboardObject>>, c: &Context, g: &mut G){
//     use graphics::{Rectangle};

//     for object in objects {
//         let position = object.get_position();
//         let size = object.get_size();
//         let object_dimensions = [position.x, position.y, size.width, size.height];

//         Rectangle::new([0.0, 0.0, 0.2, 1.0])
//             .draw(object_dimensions, &c.draw_state, c.transform, g);
//     }
// }