use crate::algebra_basics::{Coordinates, Size, Vector, LineEquation};
use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::{GameObject, GameObjectType};

pub struct MovementHandler {
    path: Vec<Coordinates>,
    current_position: Coordinates,
    current_vector: Vector
}

impl MovementHandler {
    pub fn start(current_position: Coordinates, destination: Coordinates) -> MovementHandler {
        let vector = Vector::get_vector(&current_position, &destination);
        let vector = Vector::to_unit_vector(&vector);
        
        let mut path = Vec::new();
        path.push(destination);

        MovementHandler {
            path,
            current_position,
            current_vector: vector
        }
    }

    pub fn poll_movement(&mut self) -> Option<Coordinates> {
        if self.is_reached() {
            return None;
        }

        self.current_position.x += self.current_vector.x;
        self.current_position.y += self.current_vector.y;
        
        Some(self.current_position.clone())
    }

    fn is_reached(&self) -> bool {
        if self.path.is_empty() {
            return true;
        }

        if (self.current_position.x as u32) == (self.path[0].x as u32) 
            && (self.current_position.y as u32) == (self.path[0].y as u32) {
            return true;
        }

        return false;
    }
}

#[derive(PartialEq, Debug)]
enum MovementDirection {
    None,
    Forward,
    Backward
}

impl MovementDirection {
    fn get(a: f64, b: f64) -> MovementDirection {
        if a < b {
            return MovementDirection::Forward;
        }
        else if a > b {
            return MovementDirection::Backward;
        }
        else {
            return MovementDirection::None;
        }
    }
}

pub mod pathfinding {
use crate::algebra_basics::{Coordinates, LineEquation, Vector, Size, RectangleLineEquations};
use crate::game_data::gameboard::Gameboard;
    
    pub fn find_path(start: &Coordinates, destination: &Coordinates, gameboard: &Gameboard) -> Vec<Coordinates> {
        let game_objects = gameboard.get_all_objects();
        let line_equation = LineEquation::get_line_equation(start, destination);
        let mut points = Vec::new();

        for object in game_objects {
        
        }

        points.push(destination.clone());
        points
    }

    #[derive(PartialEq, Debug)]
    enum IntersectedSide {
        X0,
        X1,
        Y0,
        Y1,
        None
    }

    fn check_if_line_intersects_with_object(line: &LineEquation, square_line_equations: &RectangleLineEquations) -> IntersectedSide {
        let floats = square_line_equations.to_floats();

        if check_if_line_intersects_object_within_opposite_line(&line, &square_line_equations.x_0, floats.2, floats.3) {
            return IntersectedSide::X0;
        }

        if check_if_line_intersects_object_within_opposite_line(&line, &square_line_equations.x_1, floats.2, floats.3) {
            return IntersectedSide::X1;
        }

        if check_if_line_intersects_object_within_opposite_line(&line, &square_line_equations.y_0, floats.0, floats.1) {
            return IntersectedSide::Y0;
        }

        if check_if_line_intersects_object_within_opposite_line(&line, &square_line_equations.y_1, floats.0, floats.1) {
            return IntersectedSide::Y1;
        }

        return IntersectedSide::None;
    }

    fn check_if_line_intersects_object_within_opposite_line(line_a: &LineEquation, 
        line_b: &LineEquation,
        left_opposite: f64,
        right_opposite: f64) -> bool {
        
        if let Some(intersection_point) = LineEquation::get_point_of_intersection(line_a, line_b) {
            match line_b {
                LineEquation::Horizontal(y) => {
                    return intersection_point.y >= left_opposite && intersection_point.y <= right_opposite;
                },
                LineEquation::Vertical(x) => {
                    return intersection_point.x >= left_opposite && intersection_point.x <= right_opposite;
                },
                _ => {}
            }
        }

        return false;
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::game_data::game_object::{GameObject, GameObjectType};

        fn setup_gameboard_with_obstacle(obstacle_coordinates: Coordinates) -> Gameboard {
            let mut gameboard = Gameboard::new();
            let obstacle = GameObject::new(GameObjectType::Static, obstacle_coordinates, Size::new(100.0, 100.0));
            gameboard.add_object(obstacle);
    
            return gameboard;
        }

        fn setup_rectangle_line_equations() -> RectangleLineEquations {
            RectangleLineEquations {
                x_0: LineEquation::Vertical(5.0),
                x_1: LineEquation::Vertical(10.0),
                y_0: LineEquation::Horizontal(5.0),
                y_1: LineEquation::Horizontal(10.0)
            }
        }

        #[test]
        fn check_if_line_intersects_with_rectangle_intersecting_horizontal_line_provided_returns_true() {
            let equation = setup_rectangle_line_equations();
            let line = LineEquation::Horizontal(7.0);

            let result = check_if_line_intersects_with_object(&line, &equation);

            assert_eq!(result, IntersectedSide::X0);
        }

        #[test]
        fn check_if_line_intersects_with_rectangle_non_intersecting_horizontal_line_provided_returns_false() {
            let equation = setup_rectangle_line_equations();
            let line = LineEquation::Horizontal(2.0);
            
            let result = check_if_line_intersects_with_object(&line, &equation);

            assert_eq!(result, IntersectedSide::None);
        }

        #[test]
        fn check_if_line_intersects_with_rectangle_intersecting_vertical_line_provided_returns_true() {
            let equation = setup_rectangle_line_equations();
            let line = LineEquation::Vertical(7.0);

            let result = check_if_line_intersects_with_object(&line, &equation);

            assert_eq!(result, IntersectedSide::Y0);
        }

        #[test]
        fn check_if_line_intersects_with_rectangle_non_intersecting_vertical_line_provided_returns_false() {
            let equation = setup_rectangle_line_equations();
            let line = LineEquation::Vertical(7.0);

            let expected = LineEquation::Vertical(12.0);
            let result = check_if_line_intersects_with_object(&line, &equation);
            
            assert_eq!(result, IntersectedSide::None);
        }

        #[test]
        fn check_if_line_intersects_with_rectangle_intersecting_curve_line_provided_returns_true() {
            let equation = setup_rectangle_line_equations();
            let line = LineEquation::Curve {
                slope: 2.0,
                y_intercept: -7.0
            };
            
            let expected = LineEquation::Horizontal(5.0);
            let result = check_if_line_intersects_with_object(&line, &equation);

            // assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn check_if_line_intersects_with_rectangle_non_intersecting_curve_line_provided_returns_false() {
            let equation = setup_rectangle_line_equations();
            let line = LineEquation::Curve {
                slope: 4.0,
                y_intercept: -5.0
            };
            
            let result = check_if_line_intersects_with_object(&line, &equation);

            // assert_eq!(result.is_none(), true);
        }

        #[test]
        fn find_path_obstacle_in_front_of_start_on_x_axis_correct_path_with_3_points_is_calculated() {
            let gameboard = setup_gameboard_with_obstacle(Coordinates::new(150.0, 100.0));
            let path = find_path(&Coordinates::new(100.0, 100.0), &Coordinates::new(200.0, 100.0), &gameboard);
            assert_eq!(path.len(), 3); 
        }

        #[test]
        fn find_path_obstacle_behind_start_on_x_axis_correct_path_with_1_point_is_calculated() {
            let gameboard = setup_gameboard_with_obstacle(Coordinates::new(100.0, 100.0));
            let path = find_path(&Coordinates::new(150.0, 100.0), &Coordinates::new(200.0, 100.0), &gameboard);
            assert_eq!(path.len(), 1);
        }

        #[test]
        fn find_path_obstacle_in_front_of_start_on_y_axis_correct_path_with_3_points_is_calculated() {
            let gameboard = setup_gameboard_with_obstacle(Coordinates::new(100.0, 150.0));
            let path = find_path(&Coordinates::new(100.0, 100.0), &Coordinates::new(100.0, 200.0), &gameboard);
            assert_eq!(path.len(), 3);
        }

        #[test]
        fn find_path_obstacle_behind_start_on_y_axis_correct_path_with_1_point_is_calculated() {
            let gameboard = setup_gameboard_with_obstacle(Coordinates::new(100.0, 100.0));
            let path = find_path(&Coordinates::new(100.0, 150.0), &Coordinates::new(100.0, 200.0), &gameboard);
            assert_eq!(path.len(), 1);
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn movement_handler_poll_movement_position_changes_correctly() {
        let a = Coordinates::new(0.0, 0.0);
        let b = Coordinates::new(50.0, 50.0);
        let mut movement_handler = MovementHandler::start(a.clone(), b.clone());

        let vector = Vector::get_vector(&a, &b);
        let unit_vector = Vector::to_unit_vector(&vector);

        let expected = Coordinates::new(a.x + unit_vector.x, a.y + unit_vector.y);
        let result = movement_handler.poll_movement();

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn movement_handler_poll_movement_returns_none_when_destination_is_reached() {
        let a = Coordinates::new(0.0, 0.0);
        let b = Coordinates::new(10.0, 10.0);
        let mut movement_handler = MovementHandler::start(a.clone(), b.clone());
        
        for _ in 0..14 {
            movement_handler.poll_movement();
        }

        assert_eq!(movement_handler.poll_movement().is_some(), true);
        assert_eq!(movement_handler.poll_movement().is_none(), true);
    }

    #[test]
    fn movmement_direction_a_lower_than_b_forward_returned() {
        let direction = MovementDirection::get(0.0, 5.0);
        let expected_direction = MovementDirection::Forward;
        
        assert_eq!(direction, expected_direction);
    }

    #[test]
    fn movmement_direction_a_higher_than_b_forward_returned() {
        let direction = MovementDirection::get(0.5, 0.0);
        let expected_direction = MovementDirection::Backward;
        
        assert_eq!(direction, expected_direction);
    }

    #[test]
    fn movmement_direction_a_same_as_b_forward_returned() {
        let direction = MovementDirection::get(0.0, 0.0);
        let expected_direction = MovementDirection::None;
        
        assert_eq!(direction, expected_direction);
    }
}