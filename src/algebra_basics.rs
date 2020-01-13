#[derive(PartialEq, Debug, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64
}

impl Coordinates {
    pub fn new(x: f64, y: f64) -> Coordinates {
        Coordinates {
            x,
            y
        }
    }

    pub fn change(&mut self, new_coordinates: Coordinates) {
        self.x = new_coordinates.x;
        self.y = new_coordinates.y;
    }

    pub fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

pub struct Size {
    pub width: f64,
    pub height: f64
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size{
        Size {
            width,
            height
        }
    }
}

pub enum LineEquation {
    Vertical(f64),
    Horizontal(f64),
    Curve{m: f64, b: f64}
}