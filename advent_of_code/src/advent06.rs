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
            let mut direction = Direction::Down;
            let mut vector = Vector::new(direction, guard_location);
            let obstacles = self.canvas.locate_element('#');
            let mut step: usize = 1;
            let mut next_location = guard_location;
            let mut points: HashSet<Point2D> = HashSet::new();
            points.insert(next_location);
            while !next_location.is_out_of_bounds(w, h) {
                let test_location = vector.get_point(step);
                if obstacles.contains(&test_location){
                    direction = match direction{
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        _ => Direction::None
                    };
                    vector = Vector::new(direction, next_location);
                    step = 1;
                }
                else{
                    next_location = test_location;
                    points.insert(next_location);
                    step+=1;
                }
            }
            points.remove(&next_location);
            result = points.len();
        }
        assert_eq!(result, 4663);
        if verbose {
            println!("Number of visited points is {}", result)
        }
        true
    }
    //
    // fn compute_part2_answer(&self, verbose: bool) -> bool{
    //     if !self.label.has_input  { return no_solution_message(verbose, 2) }
    //     false
    // }
}