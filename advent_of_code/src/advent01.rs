use std::collections::HashMap;
use itertools::izip;
use crate::utils::{no_solution_message, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    left: Vec<i32>,
    right: Vec<i32>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(1),
            left: Vec::new(),
            right: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        for v in [&mut self.right, &mut self.left] {
            if let Some(p) = parts.pop(){
                v.push(p.parse::<i32>()?);
            }
        }
        Ok(())
    }

    fn info(&self){
        if !self.label.has_input { return println!("Advent is missing input")}
        println!("Right vector has length: {}", self.right.len());
        println!("Left vector has length: {}", self.left.len());
    }

    fn compute_part1_answer(&self, verbose: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }
        let (mut right, mut left) = (self.right.clone(), self.left.clone());
        right.sort_unstable();
        left.sort_unstable();
        let mut sum: i32 = 0;
        for (n, m) in izip!(&left, &right){
            sum+=(n-m).abs();
        }
        assert_eq!(sum, 2285373);
        if verbose {
            println!("Total distance between the lists: {}", sum);
        }
        true
    }

    fn compute_part2_answer(&self, verbose: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 2)  }
        let mut num_count:HashMap<i32, i32> = HashMap::new();
        let mut score: i32 = 0;
        for &num in &self.right {
            let entry = num_count.entry(num).or_insert(0);
            *entry += 1;
        }
        for num in &self.left{
            if let Some(count) = num_count.get(num) {
                if score>(i32::MAX - num*count) {
                    panic!("Score overflow!")
                }
                score+=num*count;
            }
        }
        assert_eq!(score, 21142653);
        if verbose {
            println!("Similarity score: {}", score);
        }
        true
    }
}