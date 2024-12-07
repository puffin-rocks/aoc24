use std::collections::{HashMap, HashSet};
use crate::geometry::{Canvas, Point2D};
use crate::utils::{Solve, Label, assert_display, read_lines};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
    antennas: HashMap<char, Vec<Point2D>>
}

impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(8),
            canvas: Canvas::default(),
            antennas: HashMap::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             one_step: bool,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8) -> Result<String, String>{
        self.check_input(Some(part))?;
        let (w, h) = (*self.canvas.width(), *self.canvas.height());
        let mut antinodes: HashSet<Point2D> = HashSet::new();

        for antenna in self.antennas.values() {
            let n = antenna.len();
            if n < 2 {
                continue;
            }

            for i in 0..n - 1 {
                for j in i + 1..n {
                    let d = &antenna[i] - &antenna[j];

                    for &start in [&antenna[j], &antenna[i]] {
                        let direction = if start == antenna[j] { 1 } else { -1 };
                        let mut s = 2;

                        loop {
                            let p = start + &(&d * (s * direction));
                            if p.is_out_of_bounds(w, h) {
                                break;
                            }
                            antinodes.insert(p);
                            s += 1;
                            if one_step{
                                break
                            }
                        }
                    }
                }
            }
            if !one_step {
                antinodes.extend(antenna);
            }
        }
        assert_display(antinodes.len(), Some(result_test), result_prd, "Number of antinodes", test_mode)
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        self.canvas.add_row(line.chars().collect());
        Ok(())
    }
    fn read_input(&mut self, test_mode: bool) -> Result<(), std::num::ParseIntError>{
        let filename = self.get_label().get_puzzle_input_path(test_mode);

        if let Ok(lines) = read_lines(filename) {
            for line in lines.flatten() {
                self.add_record_from_line(line)?;
            }
            self.canvas.iter().for_each(|p| self.antennas.entry(self.canvas.get_element(&p)).or_insert_with(Vec::new).push(p));
            self.antennas.remove(&'.');
            self.get_label_mut().has_input = true;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas height: {}", self.canvas.height());
        println!("Canvas width: {}", self.canvas.width());
        println!("Number of antenna types: {}", self.antennas.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(true, 14, 357, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(false, 34, 1266, test_mode, 2)
    }
}