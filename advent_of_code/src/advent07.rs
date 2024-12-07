use std::collections::{BTreeMap};
use crate::utils::{Solve, Label, no_solution_message};
use rayon::prelude::*;

struct Equation{
    lhs: usize,
    rhs: Vec<usize>
}

impl Equation {
    fn new(lhs: usize, rhs: Vec<usize>)->Self{
        Self{
            lhs,
            rhs
        }
    }

    fn try_solve(&self) -> bool {
        if self.rhs.is_empty() {
            return false;
        }

        let n = self.rhs.len() - 1;
        if n == 0 {
            return self.lhs == self.rhs[0];
        }

        let max_operation_code = 2_usize.pow(n as u32);
        for operation_code in 0..max_operation_code {
            let mut result = self.rhs[0];
            let operation_flags = format!("{:0width$b}", operation_code, width = n);

            for (v, op) in self.rhs.iter().skip(1).zip(operation_flags.chars().map(|c| c == '1')) {
                if op {
                    result *= v;
                } else {
                    result += v;
                }

                if result > self.lhs {
                    break;
                }
            }

            if result == self.lhs {
                return true;
            }
        }

        false
    }
}

pub(crate) struct Advent {
    label: Label,
    equations: Vec<Equation>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(7),
            equations: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((lhs, rhs)) = line.split_once(": ") {
            self.equations.push(Equation::new(
                lhs.parse::<usize>()?,
                rhs.split_whitespace()
                    .map(|n| n.parse::<usize>().expect(&format!("Cannot parse equation input {}", n)))
                    .collect(),
            ));
        }
        Ok(())
    }

    fn info(&self){
        if !self.label.has_input {println!("Advent is missing input")}
        println!("Number of equations {}:", self.equations.len());
        let mut count = BTreeMap::new();
        self.equations.iter().for_each(|e| *count.entry(e.rhs.len()).or_insert(0) += 1);

        for (key, value) in count {
            println!(
                "Number of equations with {} argument{}: {}",
                key,
                if key > 1 { "s" } else { "" },
                value
            );
        }
    }

    fn compute_part1_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }

        let result: usize = self.equations
            .par_iter()
            .filter(|&e| {
                e.try_solve()
            })
            .map(|e|{e.lhs})
            .sum();

        assert_eq!(result, match test_mode{
            true =>  3749,
            false => 1582598718861
        });
        if verbose {
            println!("Sum of solvable equations: {}", result);
        }
        true
        // let mut count = BTreeMap::new();
        // self.equations.iter().for_each(|e| *count.entry(e.rhs.len()).or_insert(0) += e.is_feasible() as usize);
        //
        // for (key, value) in count {
        //     println!(
        //         "Number of feasible equations with {} argument{}: {}",
        //         key,
        //         if key > 1 { "s" } else { "" },
        //         value
        //     );
        // }
        // let num: usize = 2_usize.pow(4)-1;
        // let binary_str = format!("{:b}", num);
        // println!("Binary as string: {:0>6}", binary_str);

    }
    //
    // fn compute_part2_answer(&self, verbose: bool, test_mode: bool) -> bool{
    //     if !self.label.has_input  { return no_solution_message(verbose, 2) }
    //     false
    // }
}