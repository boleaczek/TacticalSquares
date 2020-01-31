use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::debug_utils::LineObject;
use crate::game_data::game_object::GameObject;

pub fn render<G: Graphics>(objects: Vec<&GameObject>, c: &Context, g: &mut G){
    use graphics::{Rectangle};

    for object in objects {
        let position = &object.position;
        let size = &object.size;
        let object_dimensions = [position.x, position.y, size.width, size.height];

        Rectangle::new([0.0, 0.0, 0.2, 1.0])
            .draw(object_dimensions, &c.draw_state, c.transform, g);
    }
}

pub fn render_debug<G: Graphics>(line_object: &LineObject, c: &Context, g: &mut G) {
    use graphics::Line;
    let line = [line_object.a.x, line_object.a.y, line_object.b.x, line_object.b.y];
    Line::new([255.0, 0.0, 0.2, 1.0], 1.0)
        .draw(line, &c.draw_state, c.transform, g);
}