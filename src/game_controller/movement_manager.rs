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



pub mod pathfinding {
use crate::algebra_basics::{Coordinates, LineEquation, Vector, Size, RectangleLineEquations};
use crate::algebra_basics;
use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::GameObject;
    
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

    pub fn find_path(start: &Coordinates, destination: &Coordinates, gameboard: &Gameboard) -> Vec<Coordinates> {
        let game_objects = gameboard.get_all_objects();
        let line_equation = LineEquation::get_line_equation(start, destination);
        let mut points = Vec::new();
        let mut current_x_direction = MovementDirection::get(start.x, destination.y);
        let mut current_y_direction = MovementDirection::get(start.y, destination.y);

        for object in game_objects {
            
        }

        points.push(destination.clone());
        points
    }

    fn check_if_object_is_an_obstacle(line_equation: &LineEquation,
        direction_x: &MovementDirection,
        direction_y: &MovementDirection,
        object: &GameObject) -> bool {
        let rect_line_eqs = RectangleLineEquations::get_square_line_equations(&object.position, &object.size);
        let intersection_points = IntersectionPoints::get_rectangle_intersection_points(&rect_line_eqs, line_equation);

        return intersection_points.check_if_any_intersection_point_is_in_the_area(&object.position, &object.size);
    }

    struct IntersectionPoints {
        x_0: Option<Coordinates>,
        x_1: Option<Coordinates>,
        y_0: Option<Coordinates>,
        y_1: Option<Coordinates>
    }

    impl IntersectionPoints {
        pub fn get_rectangle_intersection_points(rect_line_equations: &RectangleLineEquations, line_equation: &LineEquation) -> IntersectionPoints {
            IntersectionPoints {
                x_0: LineEquation::get_point_of_intersection(line_equation, &rect_line_equations.x_0),
                x_1: LineEquation::get_point_of_intersection(line_equation, &rect_line_equations.x_1),
                y_0: LineEquation::get_point_of_intersection(line_equation, &rect_line_equations.y_0),
                y_1: LineEquation::get_point_of_intersection(line_equation, &rect_line_equations.y_1)
            }
        }

        pub fn check_if_any_intersection_point_is_in_the_area(&self, area_upper_vertex: &Coordinates, area_size: &Size) -> bool {
            IntersectionPoints::check_if_point_contained_within_area(&self.x_0, area_upper_vertex, area_size) ||
            IntersectionPoints::check_if_point_contained_within_area(&self.x_1, area_upper_vertex, area_size) ||
            IntersectionPoints::check_if_point_contained_within_area(&self.y_0, area_upper_vertex, area_size) ||
            IntersectionPoints::check_if_point_contained_within_area(&self.y_1, area_upper_vertex, area_size)
        }

        fn check_if_point_contained_within_area(point: &Option<Coordinates>, area_upper_vertex: &Coordinates, area_size: &Size) -> bool {
            if let Some(point) = point {
                return algebra_basics::check_if_point_is_contained_within_rectangle(point, area_upper_vertex, area_size);
            }

            return false;
        }
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

        #[test]
        fn check_if_object_is_an_obstacle_x_forward_y_none_obstacle_object_provided_returns_true() {
            let direction_x = MovementDirection::Forward;
            let direction_y = MovementDirection::None;
            let game_object = GameObject::new(GameObjectType::Static, Coordinates::new(50.0, 50.0), Size::new(100.0, 100.0));
            let line = LineEquation::Vertical(75.0);

            let result = check_if_object_is_an_obstacle(&line, &direction_x, &direction_y, &game_object);

            assert_eq!(result, true);
        }

        #[test]
        fn check_if_object_is_an_obstacle_x_forward_y_none_non_obstacle_object_provided_returns_false() {
            let direction_x = MovementDirection::Forward;
            let direction_y = MovementDirection::None;
            let game_object = GameObject::new(GameObjectType::Static, Coordinates::new(50.0, 50.0), Size::new(100.0, 100.0));
            let line = LineEquation::Vertical(0.0);

            let result = check_if_object_is_an_obstacle(&line, &direction_x, &direction_y, &game_object);

            assert_eq!(result, false);
        }

        #[test]
        fn check_if_object_is_an_obstacle_x_forward_y_forward_obstacle_object_provided_returns_true() {
            let direction_x = MovementDirection::Forward;
            let direction_y = MovementDirection::Forward;
            let game_object = GameObject::new(GameObjectType::Static, Coordinates::new(200.0, 100.0), Size::new(150.0, 50.0));
            let line = LineEquation::Curve{slope: 0.9, y_intercept: -160.0};

            let result = check_if_object_is_an_obstacle(&line, &direction_x, &direction_y, &game_object);

            assert_eq!(result, true);
        }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_forward_y_forward_non_obstacle_object_provided_returns_false() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_forward_y_backward_obstacle_object_provided_returns_true() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_forward_y_backward_non_obstacle_object_provided_returns_false() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_backward_y_none_obstacle_object_provided_returns_true() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_backward_y_none_non_obstacle_object_provided_returns_false() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_backward_y_forward_obstacle_object_provided_returns_true() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_backward_y_forward_non_obstacle_object_provided_returns_false() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_backward_y_backward_obstacle_object_provided_returns_true() {
        //     unimplemented!();
        // }

        // #[test]
        // fn check_if_object_is_an_obstacle_x_backward_y_backwawrd_non_obstacle_object_provided_returns_false() {
        //     unimplemented!();
        // }
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
}