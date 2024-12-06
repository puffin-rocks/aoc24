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

fn follow_path(
    mut current_location: Point2D,
    mut current_direction: Direction,
    width: usize,
    height: usize,
    obstacles: &HashSet<Point2D>,
) -> (HashSet<Point2D>, bool) {
    let mut path_direction: HashSet<(Point2D, Point2D)> = HashSet::new();
    let mut vector = Vector::new(current_direction, current_location);
    let mut step = 1;

    while !current_location.is_out_of_bounds(width, height)
        && !path_direction.contains(&(current_location, current_direction.to_point()))
    {
        let next_location = vector.get_point(step);
        if obstacles.contains(&next_location) {
            current_direction = match current_direction {
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                _ => Direction::None,
            };
            vector = Vector::new(current_direction, current_location);
            step = 1; // Reset step for the new direction.
        } else {
            path_direction.insert((current_location, current_direction.to_point()));
            current_location = next_location;
            step += 1; // Increment step for continuous movement.
        }
    }
    let path: HashSet<Point2D> = path_direction
        .iter()
        .map(|(first, _)| *first)
        .collect();
    (path, current_location.is_out_of_bounds(width, height))
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

    fn compute_part1_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }
        let result = self.canvas.locate_element('^').iter().next().copied().map_or(0, |guard_location| {
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            let obstacles = self.canvas.locate_element('#');

            let (points, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles);
            if is_out_of_bounds { points.len() } else { 0 }
        });
        assert_eq!(result, match test_mode{
            true => 41,
            false => 4663
        });
        if verbose {
            println!("Number of visited points is {}", result)
        }
        true
    }

    // fn compute_part2_answer(&self, verbose: bool, test_mode: bool) -> bool{
    //     if !self.label.has_input  { return no_solution_message(verbose, 2) }
    //     let result = self.canvas.locate_element('^').iter().next().copied().map_or(0, |guard_location| {
    //         let (w, h) = (*self.canvas.width(), *self.canvas.height());
    //         let obstacles = self.canvas.locate_element('#');
    //
    //         let (points, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles);
    //         if !is_out_of_bounds {
    //             return 0;
    //         }
    //
    //         points.iter().filter(|&p| {
    //             let mut obstacles_upd = obstacles.clone();
    //             obstacles_upd.insert(*p);
    //             let (_, out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles_upd);
    //             !out_of_bounds
    //         }).count()
    //     });
    // assert_eq!(result, match test_mode{
    //     true => 6,
    //     false => 1530
    // });
    //     if verbose {
    //         println!("Number of looping obstacles is {}", result)
    //     }
    //     true
    // }
}