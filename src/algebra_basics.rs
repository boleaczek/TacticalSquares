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

#[derive(PartialEq, Debug, Clone)]
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

pub fn check_if_point_is_contained_within_rectangle(point: &Coordinates, rectangle_upper_vertex: &Coordinates, size: &Size) -> bool {
    let rectangle_bottom_vertex = Coordinates::new(rectangle_upper_vertex.x + size.width, rectangle_upper_vertex.y + size.height);

    if point.x > rectangle_upper_vertex.x && point.x < rectangle_bottom_vertex.x &&
        point.y > rectangle_upper_vertex.y && point.y < rectangle_bottom_vertex.y {
            return true;
    }

    return false;
}

pub fn get_middle(position: &Coordinates, size: &Size) -> Coordinates {
    let x = position.x + size.width / 2.0;
    let y = position.y + size.height / 2.0;
    return Coordinates::new(x, y);
}

#[derive(PartialEq, Debug, Clone)]
pub struct RectangleVertexes {
    pub upper_left: Coordinates,
    pub lower_left: Coordinates,
    pub upper_right: Coordinates,
    pub lower_right: Coordinates
}

impl RectangleVertexes {
    pub fn get(upper_left_vertex: &Coordinates, size: &Size) -> RectangleVertexes {
        RectangleVertexes {
            upper_left: upper_left_vertex.clone(),
            lower_left: Coordinates::new(upper_left_vertex.x, upper_left_vertex.y + size.height),
            upper_right: Coordinates::new(upper_left_vertex.x + size.width, upper_left_vertex.y),
            lower_right: Coordinates::new(upper_left_vertex.x + size.width, upper_left_vertex.y + size.height)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum LineEquation {
    Vertical(f64), // x = constant
    Horizontal(f64), // y = constant
    Curve{slope: f64, y_intercept: f64} // y = m * x + b
}

impl LineEquation {
    pub fn unwrap_to_vertical(&self) -> f64 {
        if let LineEquation::Vertical(x) = self {
            return *x;
        }
        panic!();
    }

    pub fn unwrap_to_horizontal(&self) -> f64 {
        if let LineEquation::Horizontal(y) = self {
            return *y;
        }
        panic!();
    }

    pub fn get_line_equation(a: &Coordinates, b: &Coordinates) -> LineEquation {
        if a.x == b.x {
            return LineEquation::Vertical(a.x);
        }
        
        if a.y == b.y {
            return LineEquation::Horizontal(a.y);
        }

        let slope = (a.y - b.y) / (a.x - b.x);
        let y_intercept = -1.0 * ((a.x * slope) - a.y);

        LineEquation::Curve{slope, y_intercept}
    }

    pub fn get_point_of_intersection(line_a: &LineEquation, line_b: &LineEquation) -> Option<Coordinates> {
        match line_a {
            LineEquation::Vertical(x) => return LineEquation::calculate_intersection_for_vertical(*x, &line_b),
            LineEquation::Horizontal(y) => return LineEquation::calculate_intersection_for_horizontal(*y, &line_b),
            LineEquation::Curve{slope, y_intercept} => return LineEquation::calculate_intersection_for_curve(*slope, *y_intercept, &line_b)
        }
    }

    fn calculate_intersection_for_vertical(x: f64, line_b: &LineEquation) -> Option<Coordinates> {
        match line_b {
            LineEquation::Vertical(_) => return None,
            LineEquation::Horizontal(y) => return Some(Coordinates::new(x, *y)),
            LineEquation::Curve{slope, y_intercept} => Some(LineEquation::curve_x_intersection(*slope, *y_intercept, x))
        }
    }

    fn calculate_intersection_for_horizontal(y: f64, line_b: &LineEquation) -> Option<Coordinates> {
        match line_b {
            LineEquation::Vertical(x) => return Some(Coordinates::new(*x, y)),
            LineEquation::Horizontal(_) => return None,
            LineEquation::Curve{slope, y_intercept} => return Some(LineEquation::curve_y_intersection(*slope, *y_intercept, y))
        }
    }

    fn calculate_intersection_for_curve(slope_a: f64, y_intercept_a: f64, line_b: &LineEquation) -> Option<Coordinates> {
        match line_b {
            LineEquation::Vertical(x) => return Some(LineEquation::curve_x_intersection(slope_a, y_intercept_a, *x)),
            LineEquation::Horizontal(y) => return Some(LineEquation::curve_y_intersection(slope_a, y_intercept_a, *y)),
            LineEquation::Curve{slope, y_intercept} => return Some(LineEquation::curve_curve_intersection(slope_a, y_intercept_a, *slope, *y_intercept))
        }
    }

    fn curve_curve_intersection(slope_a: f64, y_intercept_a: f64, slope_b: f64, y_intercept_b: f64) -> Coordinates {
        let x = (y_intercept_a - y_intercept_b) / (slope_a - slope_b) * -1.0;
        let y = slope_a * x + y_intercept_a;
        Coordinates::new(x, y)
    }

    fn curve_x_intersection(slope: f64, y_intercept: f64, x: f64) -> Coordinates {
        let y = slope * x + y_intercept;
        Coordinates::new(x, y)
    }

    fn curve_y_intersection(slope: f64, y_intercept: f64, y: f64) -> Coordinates {
        let x = (y_intercept - y) / slope * -1.0;
        Coordinates::new(x, y)
    }
}

#[derive(PartialEq, Debug)]
pub struct RectangleLineEquations {
    pub x_0: LineEquation,
    pub x_1: LineEquation,
    pub y_0: LineEquation,
    pub y_1: LineEquation
}

impl RectangleLineEquations {
    pub fn get_square_line_equations(position: &Coordinates, size: &Size) -> RectangleLineEquations {
        RectangleLineEquations {
            x_0: LineEquation::Vertical(position.x),
            x_1: LineEquation::Vertical(position.x + size.width),
            y_0: LineEquation::Horizontal(position.y),
            y_1: LineEquation::Horizontal(position.y + size.height)
        }
    }

    pub fn to_floats(&self) -> (f64, f64, f64, f64) {
        (self.x_0.unwrap_to_vertical(), 
        self.x_1.unwrap_to_vertical(), 
        self.y_0.unwrap_to_horizontal(), 
        self.y_1.unwrap_to_horizontal())
    }
}

#[derive(PartialEq, Debug)]
pub struct RectangleIntersectionPoints {
    pub x_0: Option<Coordinates>,
    pub x_1: Option<Coordinates>,
    pub y_0: Option<Coordinates>,
    pub y_1: Option<Coordinates>,
}

impl RectangleIntersectionPoints {
    fn get(line_equation: &LineEquation, rectangle_line_equations: &RectangleLineEquations) -> RectangleIntersectionPoints {
        let x_0 = LineEquation::get_point_of_intersection(line_equation, &rectangle_line_equations.x_0);
        let x_1 = LineEquation::get_point_of_intersection(line_equation, &rectangle_line_equations.x_1);
        let y_0 = LineEquation::get_point_of_intersection(line_equation, &rectangle_line_equations.y_0);
        let y_1 = LineEquation::get_point_of_intersection(line_equation, &rectangle_line_equations.y_1);

        RectangleIntersectionPoints {
            x_0,
            x_1,
            y_0,
            y_1
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    magnitude: f64
}

impl Vector {
    pub fn get_vector(a: &Coordinates, b: &Coordinates) -> Vector {
        let x = b.x - a.x; 
        let y = b.y - a.y;
        let magnitude = (x * x + y * y).sqrt();
        
        Vector {
            x,
            y,
            magnitude
        }
    }

    pub fn to_unit_vector(vector: &Vector) -> Vector {
        let x = vector.x / vector.magnitude;
        let y = vector.y / vector.magnitude;
        let magnitude = 1.0;

        Vector {
            x,
            y,
            magnitude
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn line_equation_get_line_equation_coordinates_for_vertical_vertical_returned() {
        let a = Coordinates::new(0.0, 1.0);
        let b = Coordinates::new(0.0, 2.0);

        let equation = LineEquation::get_line_equation(&a, &b);
        let expected = LineEquation::Vertical(0.0);
        
        assert_eq!(equation, expected);
    }

    #[test]
    fn line_equation_get_line_equation_coordinates_for_horizontal_horizontal_returned() {
        let a = Coordinates::new(1.0, 0.0);
        let b = Coordinates::new(2.0, 0.0);

        let equation = LineEquation::get_line_equation(&a, &b);
        let expected = LineEquation::Horizontal(0.0);

        assert_eq!(equation, expected);
    }

    #[test]
    fn line_equation_get_line_equation_coordinates_for_curve_provided_curve_returned() {
        let a = Coordinates::new(0.0, 1.0);
        let b = Coordinates::new(2.0, 3.0);

        let equation = LineEquation::get_line_equation(&a, &b);
        let expected = LineEquation::Curve{slope: 1.0, y_intercept: 1.0};

        assert_eq!(equation, expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_curves_provided_returns_point_of_intersection() {
        let curve_a = LineEquation::Curve{slope: 2.0, y_intercept: 0.0};
        let curve_b = LineEquation::Curve{slope: -1.0, y_intercept: 6.0};

        let result = LineEquation::get_point_of_intersection(&curve_a, &curve_b);
        let expected = Coordinates::new(2.0, 4.0);
        
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_non_corssing_vertical_lines_provided_returns_none() {
        let line_a = LineEquation::Vertical(0.0);
        let line_b = LineEquation::Vertical(1.0);

        let result = LineEquation::get_point_of_intersection(&line_a, &line_b);

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn line_equation_get_point_of_intersection_non_corssing_horizontal_lines_provided_returns_none() {
        let line_a = LineEquation::Horizontal(0.0);
        let line_b = LineEquation::Horizontal(1.0);

        let result = LineEquation::get_point_of_intersection(&line_a, &line_b);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn line_equation_get_point_of_intersection_vertical_and_horizontal_lines_provided_returns_point_of_intersection() {
        let line_a = LineEquation::Vertical(1.0);
        let line_b = LineEquation::Horizontal(2.0);

        let result = LineEquation::get_point_of_intersection(&line_a, &line_b);
        let expected = Coordinates::new(1.0, 2.0);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_curve_and_vertical_line_provided_returns_point_of_intersection() {
        let curve = LineEquation::Curve{slope: 2.0, y_intercept: 1.0};
        let line = LineEquation::Vertical(2.0);

        let result = LineEquation::get_point_of_intersection(&curve, &line);
        let expected = Coordinates::new(2.0, 5.0);

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_curve_and_horizontal_line_provided_returns_point_of_intersection() {
        let curve = LineEquation::Curve{slope: 2.0, y_intercept: 1.0};
        let line = LineEquation::Horizontal(2.0);

        let result = LineEquation::get_point_of_intersection(&curve, &line);
        let expected = Coordinates::new(0.5, 2.0);

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn vector_get_vector() {
        let a = Coordinates::new(2.0, 3.0);
        let b = Coordinates::new(6.0, 8.0);

        let vector = Vector::get_vector(&a, &b);
        let expected = Vector{x: 4.0, y: 5.0, magnitude: 6.4031242374328485};

        assert_eq!(vector, expected);
    }

    #[test]
    fn vector_get_unit_vector() {
        let vector = Vector{x: 4.0, y: 5.0, magnitude: 6.4031242374328485};
        
        let unit_vector = Vector::to_unit_vector(&vector);
        let expected = Vector{x: 0.6246950475544243, y: 0.7808688094430304, magnitude: 1.0};

        assert_eq!(unit_vector, expected);
    }

    #[test]
    fn square_line_equations_get_square_line_equations_returns_correct_equation() {
        let position = Coordinates::new(50.0, 50.0);
        let size = Size::new(50.0, 50.0);
        
        let square_line_equation = RectangleLineEquations::get_square_line_equations(&position, &size);
        let expected = RectangleLineEquations {
            x_0: LineEquation::Vertical(50.0),
            x_1: LineEquation::Vertical(100.0),
            y_0: LineEquation::Horizontal(50.0),
            y_1: LineEquation::Horizontal(100.0)
        };

        assert_eq!(square_line_equation, expected);
    }

    #[test]
    fn get_middle_returns_correct_coordinates() {
        let position = Coordinates::new(0.0, 0.0);
        let size = Size::new(50.0, 50.0);

        let middle = get_middle(&position, &size);
        let expected = Coordinates::new(25.0, 25.0);

        assert_eq!(middle, expected);
    }

    #[test]
    fn check_if_point_is_contained_within_rectangle_contained_point_is_provided_returns_true() {
        let position = Coordinates::new(0.0, 0.0);
        let size = Size::new(50.0, 50.0);
        let point = Coordinates::new(1.0, 1.0);

        let result = check_if_point_is_contained_within_rectangle(&point, &position, &size);
        assert_eq!(result, true);
    }

    #[test]
    fn check_if_point_is_contained_within_rectangle_not_contained_point_is_provided_returns_false() {
        let position = Coordinates::new(0.0, 0.0);
        let size = Size::new(50.0, 50.0);
        let point = Coordinates::new(60.0, 60.0);

        let result = check_if_point_is_contained_within_rectangle(&point, &position, &size);
        assert_eq!(result, false);
    }

    #[test]
    fn rectangle_vertexes_get_returns_correct_vertexes() {
        let rect_position = Coordinates::new(1.0, 0.0);
        let rect_size = Size::new(5.0, 5.0);

        let vertexes = RectangleVertexes::get(&rect_position, &rect_size);
        let expected = RectangleVertexes {
            upper_left: Coordinates::new(1.0, 0.0),
            lower_left: Coordinates::new(1.0, 5.0),
            upper_right: Coordinates::new(6.0, 0.0),
            lower_right: Coordinates::new(6.0, 5.0)
        };

        assert_eq!(vertexes, expected);
    }
}