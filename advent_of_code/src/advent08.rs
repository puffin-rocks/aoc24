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
        let (w, h) = (*self.canvas.width(), *self.canvas.height());
        let mut antinodes: HashSet<Point2D> = HashSet::new();
        for (_, v) in &self.antennas{
            let n = v.len();
            if n<2 { continue;}
            for i in 0..n-1{
                for j in i+1..n{
                    let d = &v[i]-&v[j];
                    for c in [&d*2 + &v[j], &d*-2 + &v[i]] {
                        if !c.is_out_of_bounds(w, h) {
                            antinodes.insert(c);
                        }
                    }
                }
            }
        }
        assert_display(antinodes.len(), Some(14), 357, "Number of antinodes", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        let (w, h) = (*self.canvas.width(), *self.canvas.height());
        let mut antinodes: HashSet<Point2D> = HashSet::new();
        for (_, v) in &self.antennas{
            let n = v.len();
            if n<2 { continue;}
            for i in 0..n-1{
                for j in i+1..n{
                    let d = &v[i]-&v[j];
                    let mut s = 2;
                    loop{
                        let p = &d*s + &v[j];
                        if p.is_out_of_bounds(w, h){
                            break;
                        }
                        else{
                            antinodes.insert(p);
                            s+=1;
                        }
                    }
                    let mut s = 2;
                    loop{
                        let p = &d*((-1)*s) + &v[i];
                        if p.is_out_of_bounds(w, h){
                            break;
                        }
                        else{
                            antinodes.insert(p);
                            s+=1;
                        }
                    }
                }
            }
            for p in v{
                antinodes.insert(*p);
            }
        }
        println!("{}", antinodes.len());
        assert_display(antinodes.len(), Some(34), 1266, "Number of antinodes", test_mode)
    }
}