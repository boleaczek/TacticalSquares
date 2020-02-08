use crate::algebra_basics::{Coordinates, Size, Vector, LineEquation};
use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::{GameObject, GameObjectType};

pub struct MovementHandler {
    path: Vec<Coordinates>,
    current_position: Coordinates,
    current_vector: Vector
}

impl MovementHandler {
    pub fn start(current_position: Coordinates, destination: Coordinates, game_objects: &Vec<&GameObject>) -> MovementHandler {
        let vector = Vector::get_vector(&current_position, &destination);
        let vector = Vector::to_unit_vector(&vector);
        let mut path = Vec::new();
        path = pathfinding::find_path(&current_position, &destination, game_objects, path);
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
use crate::game_data::game_object::{GameObject, GameObjectType};
    
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

    pub fn find_path(start: &Coordinates, destination: &Coordinates, game_objects: &Vec<&GameObject>, points: Vec<Coordinates>) -> Vec<Coordinates> {
        let point = points.last().unwrap();
        let line_equation = LineEquation::get_line_equation(point, destination);
        let mut points = Vec::new();
        let current_x_direction = MovementDirection::get(point.x, destination.y);
        let current_y_direction = MovementDirection::get(point.y, destination.y);
        
        for object in game_objects {
            if object.object_type == GameObjectType::Static {
                continue;
            }

            if !is_object_on_path(object, destination, &current_x_direction, &current_y_direction) {
                continue;
            }

            let intersected_lines = check_if_path_intersects_object(&line_equation, object);

            if intersected_lines {
                let object_line_equations = RectangleLineEquations::get_square_line_equations(&object.position, &object.size);
                let next_points = get_next_two_points(&object_line_equations, &current_x_direction, &current_y_direction);
                points = find_path(&next_points.0, destination, game_objects, points);
                points = find_path(&next_points.1, destination, game_objects, points);
                points.push(start.clone());

                return points;
            }
        }
        
        return points;
    }

    fn is_object_on_path(object: &GameObject,
        destination_coordinates: &Coordinates,
        direction_x: &MovementDirection,
        direction_y: &MovementDirection) -> bool {
        let object_position = &object.position;
        
        return is_on_path_for_direction(destination_coordinates.x, object_position.x, direction_x) &&
            is_on_path_for_direction(destination_coordinates.y, object_position.y, direction_y);
    }

    fn is_on_path_for_direction(destination_coordinate: f64, object_coordinate: f64, direction: &MovementDirection) -> bool {
        match direction {
            MovementDirection::Forward => {
                return object_coordinate > destination_coordinate;
            },
            MovementDirection::Backward => {
                return object_coordinate < destination_coordinate;
            },
            MovementDirection::None => {
                return false;
            }
        };
    }

    fn get_next_two_points(object_line_equations: &RectangleLineEquations,
        direction_x: &MovementDirection, 
        direction_y: &MovementDirection) -> (Coordinates, Coordinates) {
        match direction_x {
            MovementDirection::Forward => return get_next_two_points_for_x_forward(object_line_equations, direction_y),
            MovementDirection::Backward => return get_next_two_points_for_x_backward(object_line_equations, direction_y),
            MovementDirection::None => return get_next_two_points_for_x_none(object_line_equations, direction_y)
        }
    }

    fn get_next_two_points_for_x_forward(object_line_equations: &RectangleLineEquations, direction_y: &MovementDirection) -> (Coordinates, Coordinates) {
        let (x_0, x_1, y_0, y_1) = object_line_equations.to_floats();
        match direction_y {
            MovementDirection::Forward => { return (Coordinates::new(x_0, y_0), Coordinates::new(x_1, y_1)); },
            MovementDirection::Backward => { return (Coordinates::new(x_1, y_0), Coordinates::new(x_0, y_1)); },
            MovementDirection::None => { return (Coordinates::new(x_0, y_0), Coordinates::new(x_0, y_1)); }
        }
    }

    fn get_next_two_points_for_x_backward(object_line_equations: &RectangleLineEquations, direction_y: &MovementDirection) -> (Coordinates, Coordinates) {
        let (x_0, x_1, y_0, y_1) = object_line_equations.to_floats();
        match direction_y {
            MovementDirection::Forward => { return (Coordinates::new(x_1, y_0), Coordinates::new(x_0, y_1)); },
            MovementDirection::Backward => { return (Coordinates::new(x_0, y_1), Coordinates::new(x_1, y_1)); },
            MovementDirection::None => { return (Coordinates::new(x_1, y_0), Coordinates::new(x_1, y_1)); }
        }
    }

    fn get_next_two_points_for_x_none(object_line_equations: &RectangleLineEquations, direction_y: &MovementDirection) -> (Coordinates, Coordinates) {
        let (x_0, x_1, y_0, y_1) = object_line_equations.to_floats();
        match direction_y {
            MovementDirection::Forward => { return (Coordinates::new(x_0, y_1), Coordinates::new(x_1, y_1)); },
            MovementDirection::Backward => { return (Coordinates::new(x_0, y_0), Coordinates::new(x_0, y_1)); },
            MovementDirection::None => panic!("(None,None) movement direction")
        }
    }

    fn check_if_path_intersects_object(line_equation: &LineEquation, object: &GameObject) -> bool {
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
            return IntersectionPoints::check_if_point_contained_within_area(&self.x_0, area_upper_vertex, area_size) &&
                IntersectionPoints::check_if_point_contained_within_area(&self.x_1, area_upper_vertex, area_size) &&
                IntersectionPoints::check_if_point_contained_within_area(&self.y_0, area_upper_vertex, area_size) &&
                IntersectionPoints::check_if_point_contained_within_area(&self.y_1, area_upper_vertex, area_size);
            
        }

        fn check_if_point_contained_within_area(point: &Option<Coordinates>, 
            area_upper_vertex: &Coordinates, 
            area_size: &Size) -> bool {
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
            // let gameboard = setup_gameboard_with_obstacle(Coordinates::new(150.0, 100.0));
            // let mut path = Vec::new();
            // let result: bool;
            // path.push(Coordinates::new(100.0, 100.0));
            // let (path, result) = find_path(&Coordinates::new(200.0, 100.0), gameboard.get_all_objects(), path);
            // assert_eq!(path.len(), 3); 
            unimplemented!();
        }

        #[test]
        fn find_path_obstacle_behind_start_on_x_axis_correct_path_with_1_point_is_calculated() {
            unimplemented!();
        }

        #[test]
        fn find_path_obstacle_in_front_of_start_on_y_axis_correct_path_with_3_points_is_calculated() {
            unimplemented!();
        }

        #[test]
        fn find_path_obstacle_behind_start_on_y_axis_correct_path_with_1_point_is_calculated() {
            unimplemented!();
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
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn movement_handler_poll_movement_position_changes_correctly() {
        // let a = Coordinates::new(0.0, 0.0);
        // let b = Coordinates::new(50.0, 50.0);
        // let mut movement_handler = MovementHandler::start(a.clone(), b.clone());

        // let vector = Vector::get_vector(&a, &b);
        // let unit_vector = Vector::to_unit_vector(&vector);

        // let expected = Coordinates::new(a.x + unit_vector.x, a.y + unit_vector.y);
        // let result = movement_handler.poll_movement();

        // assert_eq!(result.unwrap(), expected);
        unimplemented!();
    }

    #[test]
    fn movement_handler_poll_movement_returns_none_when_destination_is_reached() {
        // let a = Coordinates::new(0.0, 0.0);
        // let b = Coordinates::new(10.0, 10.0);
        // let mut movement_handler = MovementHandler::start(a.clone(), b.clone());
        
        // for _ in 0..14 {
        //     movement_handler.poll_movement();
        // }

        // assert_eq!(movement_handler.poll_movement().is_some(), true);
        // assert_eq!(movement_handler.poll_movement().is_none(), true);
        unimplemented!();
    }
}