use std::collections::HashMap;
use rayon::prelude::*;
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    stones: Vec<usize>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(11),
            stones: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             max_blinks: usize,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8
    ) -> Result<String, String>{
        self.check_input(Some(part))?;
        let result: usize = self.stones
            .par_iter()
            .map(|s| {
                let mut stones: HashMap<usize, usize> = HashMap::new();
                stones.insert(*s, 1);
                let mut n_blinks = 0;

                while n_blinks < max_blinks {

                    let mut next_stones: HashMap<usize, usize> = HashMap::new();

                    for (&stone, &cnt) in stones.iter() {
                        let pow = (stone as f64).log(10.0).floor() as u32 + 1;
                        let values = if stone < 1 {
                            vec![1]
                        } else if pow % 2 == 0 {
                            let div = 10_usize.pow(pow / 2);
                            vec![stone / div, stone % div]
                        } else {
                            vec![stone * 2024]
                        };
                        for v in values {
                            *next_stones.entry(v).or_insert(0) += cnt;
                        }
                    }

                    stones = next_stones;
                    n_blinks += 1;
                }
                stones.values().into_iter().sum::<usize>()
            })
            .sum();
        assert_display(result, Some(result_test), result_prd, "Number of stones", test_mode)
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        self.stones.extend(
            line.split_whitespace()
                .map(|n| n.parse::<usize>().expect(&format!("Cannot parse stone input {}", n)))
                .collect::<Vec<usize>>());
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of stones: {:?}", self.stones.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(25, 55312, 186175, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String> {
        self.solve(75, 65601038650482, 220566831337810, test_mode, 2)
    }
}
