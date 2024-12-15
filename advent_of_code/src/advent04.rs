use crate::utils::{Solve, Label, assert_display};
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
        let (&width, &height) = self.canvas.shape();
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
            if self.canvas.get_element(&p) != Some(ch) {
                return false;
            }
        }
        true
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }

    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let first_letter = Some(&self.word[0]);
        let mut count = 0;
        if self.solve_via_rotation{
            //canvas rotation (slower)
            let (&w, &h) = self.canvas.shape();
            assert_eq!(w, h); //via rotation of points of original canvas works only for squares
            for p in self.canvas.iter() {
                for n_rotations in 0..4 {
                    let p_rotated = p.rotate90(n_rotations, w, h);
                    if first_letter != self.canvas.get_element(&p_rotated) {
                        continue;
                    }

                    for &d in [Direction::Up, Direction::UpRight].iter() {
                        let shifted_loc = Vector::new(d, **p);
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

                for d in [&Direction::base()[..], &Direction::diagonal()[..]].concat() {
                    let shifted_loc = Vector::new(d, **p);
                    if self.check_match(&shifted_loc, None, None) {
                        count += 1;
                    }
                }
            }
        }
        assert_display(count,
                       Some(18),
                       2547,
                       "Number of words",
                       test_mode,
        )
    }

    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let cut_word = self.word[1..].to_vec();
        let first_letter = Some(&cut_word[0]);
        let mut count = 0;

        if self.solve_via_rotation {
            //canvas rotation (slower)
            let base_loc = Vector::new(Direction::UpRight, Point2D::new(0, 0));
            let supp_loc = Vector::new(Direction::DownRight, Point2D::new(0, 2));
            let (&w, &h) = self.canvas.shape();
            assert_eq!(w, h); //via rotation of points of original canvas works only for squares
            for p in self.canvas.iter() {
                let d = Direction::ToPoint(**p);
                for n_rotations in 0..4 {
                    let p_rotated = p.rotate90(n_rotations, w, h);
                    if first_letter != self.canvas.get_element(&p_rotated) {
                        continue;
                    }

                    if self.check_match(&base_loc.shift(&d), Some(&cut_word), Some(n_rotations)) &&
                        self.check_match(&supp_loc.shift(&d), Some(&cut_word), Some(n_rotations)) {
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
                    let shifted_loc = Vector::new(d, **p);
                    if self.check_match(&shifted_loc, Some(&cut_word), None) {
                        let supp_loc = match shifted_loc.direction() {
                            Direction::DownRight => Vector::new(Direction::DownLeft, Point2D::new(2, 0)),
                            Direction::UpRight => Vector::new(Direction::DownRight, Point2D::new(0, 2)),
                            Direction::UpLeft => Vector::new(Direction::UpRight, Point2D::new(-2, 0)),
                            Direction::DownLeft => Vector::new(Direction::UpLeft, Point2D::new(0, -2)),
                            _ => Vector::null(),
                        }.shift(&Direction::ToPoint(**p));

                        if self.check_match(&supp_loc, Some(&cut_word), None) {
                            count += 1;
                        }
                    }
                }
            }
        }
        assert_display(count,
                       Some(9),
                       1939,
                       "Number of words",
                       test_mode,
        )
    }
}