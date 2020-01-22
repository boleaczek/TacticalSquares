use graphics::types::Color;
use graphics::{Context, Graphics};

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