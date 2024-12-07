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
    ordered_path: Option<&[Vector]>, // Optional parameter
) -> (Vec<Vector>, bool) {
    let mut path: Vec<Vector> = Vec::new();
    let mut path_set: HashSet<Vector> = HashSet::new();
    let (ordered_path_set, push_to_path) =
        if let Some(ordered_path) = ordered_path {
        (ordered_path.iter().collect(), false)
    } else {
        (HashSet::new(), true)
    };
    let mut vector = Vector::new(current_direction, current_location);
    let mut is_out_of_bounds = true;

    loop {
        let next_location = vector.get_point(1);
        if next_location.is_out_of_bounds(width, height) {
            if push_to_path {
                path.push(vector);
            }
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
            if path_set.contains(&vector) || ordered_path_set.contains(&&vector) {
                is_out_of_bounds = false;
                break;
            }
            if push_to_path {
                path.push(vector.clone());
            }
            path_set.insert(vector);
            vector = Vector::new(current_direction, next_location);
        }
    }
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

            let (path, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles, None);
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

            let (path, is_out_of_bounds) = follow_path(guard_location, Direction::Down, w, h, &obstacles, None);
            if !is_out_of_bounds {
                return 0;
            }

            path
                .par_windows(2)
                .filter_map(|window| {
                    if let [prev_vector, curr_vector] = window {
                        let start_point = *prev_vector.anchor();
                        let direction = *prev_vector.direction();
                        let current_point = *curr_vector.anchor();

                        let path_part = &path[..path.iter().position(|x| x == prev_vector).unwrap()];

                        // Check if current_point is already in path_part
                        if path_part.iter().any(|p| *p.anchor() == current_point) {
                            return None;
                        }

                        let mut updated_obstacles = obstacles.clone();
                        updated_obstacles.insert(current_point);

                        // Check for looping path
                        let is_looping = !follow_path(start_point, direction, w, h, &updated_obstacles, Some(path_part)).1;
                        if is_looping {
                            return Some([current_point].iter().cloned().collect::<HashSet<_>>());
                        }
                    }
                    None
                })
                .reduce(HashSet::new, |mut acc, set| {
                    acc.extend(set);
                    acc
                })
                .len()

        });
        assert_display(result,
                       Some(6),
                       1530,
                       "Number of looping obstacles",
                       test_mode,
        )
    }
}