use crate::utils::{Solve, Label, no_solution_message};
use crate::geometry::{Point2D, Direction, Vector, Canvas};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
    word: Vec<char>,
    solve_via_rotation: bool
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(4),
            canvas: Canvas::default(),
            word: vec!['X','M','A','S'],
            solve_via_rotation: true
        }
    }
}

impl Advent{
    fn check_match(&self, location: &Vector, word: Option<&Vec<char>>, n_rotations: Option<u8>) -> bool{
        let height = *self.canvas.height();
        let width = *self.canvas.width();
        let word = word.unwrap_or(&self.word);
        let n_chars = word.len();

        if location.is_out_of_bounds(n_chars, width, height) {
            return false;
        }

        for (e, ch) in word.iter().enumerate() {
            let mut p = location.get_point(e);
            if let Some(n) = n_rotations {
                p = p.rotate90(n, width, height);
            }
            if &self.canvas.get_element(&p) != ch {
                return false;
            }
        }
        true
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
    }

    fn compute_part1_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }
        let first_letter = self.word[0];
        let mut count = 0;
        if self.solve_via_rotation{
            //canvas rotation (slower)
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            assert_eq!(w, h); //via rotation of points of original canvas works only for squares
            for p in self.canvas.iter() {
                for n_rotations in 0..4 {
                    let p_rotated = p.rotate90(n_rotations, w, h);
                    if first_letter != self.canvas.get_element(&p_rotated) {
                        continue;
                    }

                    for &d in [Direction::Up, Direction::UpRight].iter() {
                        let shifted_loc = Vector::new(d, p);
                        if self.check_match(&shifted_loc, None, Some(n_rotations)) {
                            count += 1;
                        }
                    }
                }
            }
        }
        else{
            for p in self.canvas.iter() {
                if first_letter != self.canvas.get_element(&p) {
                    continue;
                }

                for d in Direction::list_valid() {
                    let shifted_loc = Vector::new(d, p);
                    if self.check_match(&shifted_loc, None, None) {
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(count, match test_mode{
            true => 18,
            false => 2547
        });

        if verbose {
            println! {"Number of words: {}", count}
        }
        true
    }

    fn compute_part2_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input  { return no_solution_message(verbose, 2) }
        let cut_word = self.word[1..].to_vec();
        let first_letter = cut_word[0];
        let mut count = 0;

        if self.solve_via_rotation {
            //canvas rotation (slower)
            let base_loc = Vector::new(Direction::UpRight, Point2D::new(0, 0));
            let supp_loc = Vector::new(Direction::DownRight, Point2D::new(0, 2));
            let (w, h) = (*self.canvas.width(), *self.canvas.height());
            assert_eq!(w, h); //via rotation of points of original canvas works only for squares
            for p in self.canvas.iter() {
                for n_rotations in 0..4 {
                    let p_rotated = p.rotate90(n_rotations, w, h);
                    if first_letter != self.canvas.get_element(&p_rotated) {
                        continue;
                    }

                    if self.check_match(&base_loc.shift(&p), Some(&cut_word), Some(n_rotations)) &&
                        self.check_match(&supp_loc.shift(&p), Some(&cut_word), Some(n_rotations)) {
                        count += 1;
                    }
                }
            }
        }
        else {
            let base_dir = [
                Direction::DownRight, Direction::UpRight, Direction::UpLeft, Direction::DownLeft
            ];

            for p in self.canvas.iter() {
                if first_letter != self.canvas.get_element(&p) {
                    continue;
                }

                for &d in &base_dir {
                    let shifted_loc = Vector::new(d, p);
                    if self.check_match(&shifted_loc, Some(&cut_word), None) {
                        let supp_loc = match shifted_loc.direction() {
                            Direction::DownRight => Vector::new(Direction::DownLeft, Point2D::new(2, 0)),
                            Direction::UpRight => Vector::new(Direction::DownRight, Point2D::new(0, 2)),
                            Direction::UpLeft => Vector::new(Direction::UpRight, Point2D::new(-2, 0)),
                            Direction::DownLeft => Vector::new(Direction::UpLeft, Point2D::new(0, -2)),
                            _ => Vector::null(),
                        }.shift(&p);

                        if self.check_match(&supp_loc, Some(&cut_word), None) {
                            count += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(count, match test_mode{
            true => 9,
            false => 1939
        });
        if verbose {
            println! {"Number of words: {}", count}
        }
        true
    }
}