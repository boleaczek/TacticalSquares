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

#[derive(PartialEq, Debug)]
pub enum LineEquation {
    Vertical(f64), // x = constant
    Horizontal(f64), // y = constant
    Curve{slope: f64, y_intercept: f64} // y = m * x + b
}

#[derive(PartialEq, Debug)]
pub struct Vector {
    a: Coordinates,
    b: Coordinates,
    magnitude: f64
}

impl LineEquation {
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

    pub fn get_point_of_intersection(line_a: LineEquation, line_b: LineEquation) -> Option<Coordinates> {
        match line_a {
            LineEquation::Vertical(x) => return LineEquation::calculate_intersection_for_vertical(x, &line_b),
            LineEquation::Horizontal(y) => return LineEquation::calculate_intersection_for_horizontal(y, &line_b),
            LineEquation::Curve{slope, y_intercept} => return LineEquation::calculate_intersection_for_curve(slope, y_intercept, &line_b)
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

        let result = LineEquation::get_point_of_intersection(curve_a, curve_b);
        let expected = Coordinates::new(2.0, 4.0);
        
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_non_corssing_vertical_lines_provided_returns_none() {
        let line_a = LineEquation::Vertical(0.0);
        let line_b = LineEquation::Vertical(1.0);

        let result = LineEquation::get_point_of_intersection(line_a, line_b);

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn line_equation_get_point_of_intersection_non_corssing_horizontal_lines_provided_returns_none() {
        let line_a = LineEquation::Horizontal(0.0);
        let line_b = LineEquation::Horizontal(1.0);

        let result = LineEquation::get_point_of_intersection(line_a, line_b);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn line_equation_get_point_of_intersection_vertical_and_horizontal_lines_provided_returns_point_of_intersection() {
        let line_a = LineEquation::Vertical(1.0);
        let line_b = LineEquation::Horizontal(2.0);

        let result = LineEquation::get_point_of_intersection(line_a, line_b);
        let expected = Coordinates::new(1.0, 2.0);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_curve_and_vertical_line_provided_returns_point_of_intersection() {
        let curve = LineEquation::Curve{slope: 2.0, y_intercept: 1.0};
        let line = LineEquation::Vertical(2.0);

        let result = LineEquation::get_point_of_intersection(curve, line);
        let expected = Coordinates::new(2.0, 5.0);

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn line_equation_get_point_of_intersection_curve_and_horizontal_line_provided_returns_point_of_intersection() {
        let curve = LineEquation::Curve{slope: 2.0, y_intercept: 1.0};
        let line = LineEquation::Horizontal(2.0);

        let result = LineEquation::get_point_of_intersection(curve, line);
        let expected = Coordinates::new(0.5, 2.0);

        assert_eq!(result.unwrap(), expected);
    }
}