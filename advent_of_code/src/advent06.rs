use std::collections::HashSet;
use crate::geometry::{Canvas, Direction, Point2D, Vector};
use crate::utils::{Solve, Label, no_solution_message};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(6),
            canvas: Canvas::default()
        }
    }
}

fn follow_path(current_location: Point2D, current_direction: Direction,
                   width: usize, heigth: usize, obstacles: &HashSet<Point2D>,
                   path_direction: &mut HashSet<(Point2D, Point2D)>, path: &mut HashSet<Point2D>) -> bool{
        let mut vector = Vector::new(current_direction, current_location);
        let mut location = current_location;
        let mut direction = current_direction;
        let mut step = 1;
        while !location.is_out_of_bounds(width, heigth) &
            !path_direction.contains(&(location, direction.to_point())) {

            let test_location = vector.get_point(step);
            if obstacles.contains(&test_location){
                direction = match direction{
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    _ => Direction::None
                };
                vector = Vector::new(direction, location);
                step = 1;
            }
            else{
                &path_direction.insert((location, direction.to_point())); //save history;
                &path.insert((location)); //save history;
                location = test_location;
                step+=1;
            }
        }
        location.is_out_of_bounds(width, heigth)
    }

fn follow_path_check_loop(current_location: Point2D, current_direction: Direction,
               width: usize, heigth: usize, obstacles: &HashSet<Point2D>,
               path_direction: &mut HashSet<(Point2D, Point2D)>, path: &mut HashSet<Point2D>) -> HashSet<Point2D>{
    let mut vector = Vector::new(current_direction, current_location);
    let mut location = current_location;
    let mut direction = current_direction;
    let mut step = 1;
    let mut looping_obstacles: HashSet<Point2D> = HashSet::new();
    while !location.is_out_of_bounds(width, heigth) &
        !path_direction.contains(&(location, direction.to_point())) {

        let mut path_direction_clone = path_direction.clone();
        let mut path_clone = path.clone();


        let test_location = vector.get_point(step);
        if obstacles.contains(&test_location){
            direction = match direction{
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                _ => Direction::None
            };
            vector = Vector::new(direction, location);
            step = 1;
        }
        else{
            &path_direction.insert((location, direction.to_point())); //save history;
            &path.insert((location)); //save history;
            let mut obstacles_upd = obstacles.clone();
            obstacles_upd.insert(test_location);
            let out_of_bounds = follow_path(location, direction, width, heigth, &obstacles_upd, &mut path_direction_clone, &mut path_clone);
            if !out_of_bounds{
                looping_obstacles.insert(test_location);
               // println!("Obstacle {:?}", test_location)
            }
            location = test_location;
            step+=1;
        }
    }
    looping_obstacles
}


impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        self.canvas.add_row(line.chars().collect());
        Ok(())
    }

    fn info(&self){
        if !self.label.has_input {println!("Advent is missing input")}
        println!{"Canvas height: {}", self.canvas.height()}
        println!{"Canvas width: {}", self.canvas.width()}
        println!{"Location of the quard is {:?}", self.canvas.locate_element('^').iter().next().expect("Guard not found")}
    }

    fn compute_part1_answer(&self, verbose: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }
        let guard_location = self.canvas.locate_element('^').iter().next().copied();
        let mut result =0;
        if let Some(guard_location) = guard_location {
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            let obstacles = self.canvas.locate_element('#');
            let mut path_direction: HashSet<(Point2D, Point2D)> = HashSet::new();
            let mut points: HashSet<Point2D> = HashSet::new();
            let flag = follow_path(guard_location, Direction::Down, w, h, &obstacles, &mut path_direction, &mut points);
            println!("flag {}", flag);
            result = points.len();
        }
        //assert_eq!(result, 4663);
        //assert_eq!(result, 41);
        if verbose {
            println!("Number of visited points is {}", result)
        }
        true
    }
    //
    fn compute_part2_answer(&self, verbose: bool) -> bool{
        if !self.label.has_input  { return no_solution_message(verbose, 2) }
        let guard_location = self.canvas.locate_element('^').iter().next().copied();
        let mut looping_obstacles: HashSet<Point2D> = HashSet::new();
        if let Some(guard_location) = guard_location {
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            let obstacles = self.canvas.locate_element('#');
            let mut path_direction: HashSet<(Point2D, Point2D)> = HashSet::new();
            let mut points: HashSet<Point2D> = HashSet::new();
            looping_obstacles = follow_path_check_loop(guard_location, Direction::Down, w, h, &obstacles, &mut path_direction, &mut points);
            println!("Has guard {:?}", looping_obstacles.contains(&guard_location));
            let mut count = 0;
            for p in &looping_obstacles{
               let mut obstacles = self.canvas.locate_element('#');
                obstacles.insert(p.clone());
                let mut path_direction: HashSet<(Point2D, Point2D)> = HashSet::new();
                let mut points: HashSet<Point2D> = HashSet::new();
                if !follow_path(guard_location, Direction::Down, w, h, &obstacles, &mut path_direction, &mut points){
                    count+=1;
                }
            }
            println!("{}", count);
        }
        //1530
        println!("{}", looping_obstacles.len());

        false
    }
}