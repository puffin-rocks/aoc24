use std::collections::HashSet;
use crate::geometry::{Canvas, Direction, Point2D, Vector};
use crate::utils::{Solve, Label, assert_display};
use rayon::prelude::*;

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
}

impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(6),
            canvas: Canvas::default(),
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
        if next_location.is_out_of_bounds(width, height) {
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
            vector.change_direction(current_direction);
        } else {
            if path.contains(&vector) {
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
    fn get_label(&self) -> &Label { &self.label }
    fn get_label_mut(&mut self) -> &mut Label { &mut self.label }

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        self.canvas.add_row(line.chars().collect());
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Canvas height: {}", self.canvas.height());
        println!("Canvas width: {}", self.canvas.width());
        println!("Location of the quard is {:?}", self.canvas.locate_element('^').iter().next().expect("Guard not found"));
        Ok(())
    }

    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String> {
        self.check_input(Some(1))?;
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
        assert_display(result,
                       Some(41),
                       4663,
                       "Number of visited points",
                       test_mode,
        )
    }

    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String> {
        self.check_input(Some(2))?;
        let result = self.canvas.locate_element('^').iter().next().copied().map_or(0, |guard_location| {
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            let obstacles = self.canvas.locate_element('#');

            let (path, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles);
            if !is_out_of_bounds {
                return 0;
            }

            let mut points = path.iter().map(|v| *v.anchor()).collect::<HashSet<_>>();
            points.remove(&guard_location);

            points
                .par_iter()
                .filter(|&&p| {
                    let mut obstacles_upd = obstacles.clone();
                    obstacles_upd.insert(p);
                    let is_looping = !follow_path(guard_location, Direction::Down, w, h, &obstacles_upd).1;
                    is_looping
                })
                .count()
        });
        assert_display(result,
                       Some(6),
                       1530,
                       "Number of looping obstacles",
                       test_mode,
        )
    }
}