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
    current_location: Point2D,
    mut current_direction: Direction,
    width: usize,
    height: usize,
    obstacles: &HashSet<Point2D>,
) -> (HashSet<Vector>, bool) {
    let mut path: HashSet<Vector> = HashSet::new();
    let mut vector = Vector::new(current_direction, current_location);
    let mut is_out_of_bounds: bool = true;
    loop {
        let next_location = vector.get_point(1);
        if next_location.is_out_of_bounds(width, height){
            path.insert(vector);
            break;
        }
        if obstacles.contains(&next_location) {
            current_direction = match current_direction {
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                _ => Direction::None,
            };
            vector = Vector::new(current_direction, *vector.anchor());
        } else {
            if path.contains(&vector){
                is_out_of_bounds = false;
                break;
            }
            path.insert(vector);
            vector = Vector::new(current_direction, next_location);
        }
    };
    (path, is_out_of_bounds)
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

            let (path, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles);
            let points: HashSet<Point2D> = path
                .iter()
                .map(|v| *v.anchor())
                .collect();
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

    fn compute_part2_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input  { return no_solution_message(verbose, 2) }
        let result = self.canvas.locate_element('^').iter().next().copied().map_or(0, |guard_location| {
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            let obstacles = self.canvas.locate_element('#');

            let (path, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles);
            if !is_out_of_bounds {
                return 0;
            }

            let points = path.iter().map(|v| *v.anchor()).collect::<HashSet<_>>();

            points
                .iter()
                .filter(|&&p| {
                    let mut obstacles_upd = obstacles.clone();
                    obstacles_upd.insert(p);
                    !follow_path(guard_location, Direction::Down, w, h, &obstacles_upd).1
                })
                .count()
        });
        assert_eq!(result, match test_mode{
            true => 6,
            false => 1530
        });
        if verbose {
            println!("Number of looping obstacles is {}", result)
        }
        true
    }
}