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
        // path = pathfinding::find_path(&current_position, &destination, game_objects, path);
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
use crate::algebra_basics::{Coordinates, Size, RectangleVertexes};
use crate::algebra_basics;
use crate::game_data::gameboard::Gameboard;
use crate::game_data::game_object::{GameObject, GameObjectType};
use crate::game_data::gameboard;

    #[derive(PartialEq, Debug)]
    enum Node {
        Blocked(Coordinates),
        Free(Coordinates)
    }

    #[derive(PartialEq, Debug)]
    struct NodeMap {
        nodes: Vec<Node>
    }

    pub struct Area {
        upper_left_vertex: Coordinates,
        size: Size
    }

    impl Area {
        pub fn new(upper_left_vertex: Coordinates, size: Size) -> Area {
            Area {
                upper_left_vertex,
                size
            }
        }
    }

    impl NodeMap {
        pub fn get(area: Area, game_objects: Vec<&GameObject>, node_size: &Size) -> NodeMap {
            let objects_in_the_area: Vec<&GameObject> = game_objects.into_iter().filter(|object| {
                let vertexes = RectangleVertexes::get(&object.position, &object.size);
                return check_if_object_contains_rectanlge(&vertexes, object);
            }).collect();

            let cols = (area.size.height / node_size.height) as usize;
            let rows = (area.size.width / node_size.width) as usize;
            
            let mut current_y = area.upper_left_vertex.y;

            let mut nodes = Vec::new();
            for _ in 0..rows {
                let mut current_x = area.upper_left_vertex.x;
                for _ in 0..cols {
                    let node_coordinates = algebra_basics::get_middle(&Coordinates::new(current_x, current_y), node_size);
                    let node_vertexes = RectangleVertexes::get(&node_coordinates, node_size);
                    
                    let is_node_free = objects_in_the_area.iter().find(|object| {
                        return check_if_object_contains_rectanlge(&node_vertexes, object);
                    });

                    if let Some(_) = is_node_free {
                        // println!("blocked for: row: {}, col: {}", row, col);
                        nodes.push(Node::Blocked(node_coordinates));
                    }
                    else {
                        nodes.push(Node::Free(node_coordinates));
                    }

                    current_x += node_size.width;
                }

                current_y += node_size.height;
            }
            
            NodeMap {
                nodes
            }
        }
    }

    fn check_if_object_contains_rectanlge(rectangle_vertexes: &RectangleVertexes, object: &GameObject) -> bool {
        return  gameboard::check_if_object_area_contains_coordinates(&object, &rectangle_vertexes.lower_left) ||
                gameboard::check_if_object_area_contains_coordinates(&object, &rectangle_vertexes.lower_right) ||
                gameboard::check_if_object_area_contains_coordinates(&object, &rectangle_vertexes.upper_left) ||
                gameboard::check_if_object_area_contains_coordinates(&object, &rectangle_vertexes.upper_right);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn node_map_get_returns_correct_node_map_no_obstacles_provided_every_node_is_free() {
            let objects = vec![];
            let area = Area::new(Coordinates::new(4.0, 2.0), Size::new(10.0, 8.0));
            let node_size = Size::new(2.0, 2.0);

            let node_map = NodeMap::get(area, objects, &node_size);
            let blocked_nodes: Vec<&Node> = node_map.nodes.iter().filter(|node| {
                if let Node::Blocked(_) = node {
                    return true;
                }
                return false;
            }).collect();

            assert_eq!(node_map.nodes.len(), 20);
            assert_eq!(blocked_nodes.len(), 0);
        }

        #[test]
        fn node_map_get_returns_correct_node_map_obstacles_provided_correct_nodes_are_blocked() {
            let game_object = GameObject::new(GameObjectType::Static, Coordinates::new(4.0, 2.0), Size::new(2.0, 4.0));
            let objects = vec![&game_object];
            let area = Area::new(Coordinates::new(4.0, 2.0), Size::new(10.0, 8.0));
            let node_size = Size::new(2.0, 2.0);

            let node_map = NodeMap::get(area, objects, &node_size);
            let blocked_nodes: Vec<&Node> = node_map.nodes.iter().filter(|node| {
                if let Node::Blocked(_) = node {
                    return true;
                }
                return false;
            }).collect();

            let blocked_node_0 = Node::Blocked(Coordinates::new(5.0, 3.0));
            let blocked_node_1 = Node::Blocked(Coordinates::new(5.0, 5.0));
            let expected_blocked_nodes = vec![&blocked_node_0, &blocked_node_1];

            assert_eq!(node_map.nodes.len(), 20);
            assert_eq!(blocked_nodes.len(), 2);
            assert_eq!(blocked_nodes, expected_blocked_nodes);
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