use std::collections::{BTreeMap};
use crate::utils::{Solve, Label, assert_display};
use rayon::prelude::*;

enum Convertable {
    Binary((usize, usize)),
    Trinary((usize, usize)),
}

impl Convertable {
    fn update_input(&mut self, other :usize){
        match self {
            Convertable::Binary((num, _)) | Convertable::Trinary((num, _)) => *num = other
        }
    }
    fn convert(&self)->String{
        match self{
            Convertable::Binary((num, width)) => {
                format!("{:0width$b}", num, width = width)
            }
            Convertable::Trinary((num, width)) => {
                let mut num = *num;
                if num == 0 {
                    return format!("{:0>width$}", "0", width = width);
                }
                let mut trinary_str = String::new();
                while num > 0 {
                    let remainder = num % 3;
                    trinary_str.push_str(&remainder.to_string());
                    num /= 3;
                }
                format!("{:0>width$}", trinary_str.chars().rev().collect::<String>(), width = width)
            }
        }
    }
    fn max_operation_code(&self)-> usize{
        match self {
            Convertable::Binary((_, width)) => 2_usize.pow(*width as u32),
            Convertable::Trinary((_, width)) => 3_usize.pow(*width as u32)
        }
    }
}

#[derive(Debug)]
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

    fn try_solve_bruteforce(&self, convertable: fn((usize, usize)) ->Convertable) -> bool {
        if self.rhs.is_empty() {
            return false;
        }

        let n = self.rhs.len() - 1;
        if n == 0 {
            return self.lhs == self.rhs[0];
        }

        let mut c: Convertable = convertable((0, n));
        for operation_code in 0..c.max_operation_code() {
            c.update_input(operation_code);
            let mut result = self.rhs[0];
            for (v, op) in self.rhs.iter().skip(1).zip(c.convert().chars()) {
                match op{
                    '2' => {
                        let p: u32 = (*v as f64).log(10.0).floor() as u32 + 1;
                        result*=10_usize.pow(p);
                        result+=v;
                    }
                    '1' => result *= v,
                    '0' => result += v,
                    _ => {}
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

    fn try_solve(&self, concat_allowed: bool) -> bool{
        if self.rhs.is_empty() {
            return false;
        }

        let n = self.rhs.len() - 1;
        if n == 0 {
            return self.lhs == self.rhs[0];
        }
        let mut stack: Vec<usize> = Vec::new();
        stack.push(self.rhs[0]);
        let mut ix = 1;

        let mut result = false;
        loop{
            let mut next_stack: Vec<usize> = Vec::new();
            let curr_num = self.rhs[ix];
            while let Some(v) = stack.pop() {
                if !v>self.lhs {
                    if concat_allowed {
                        let p: u32 = (curr_num as f64).log(10.0).floor() as u32 + 1;
                        let u = (v * 10_usize.pow(p)) + curr_num;
                        next_stack.push(u);
                    }
                    let u =v * curr_num;
                    next_stack.push(u);

                    let u =v + curr_num;
                    next_stack.push(u);
                }
            }
            stack = next_stack;
            if ix == n {
                for s in stack{
                    if s == self.lhs{
                        result = true;
                    }
                }
                break;
            }
            ix+=1;
        }
        result
    }
}

pub(crate) struct Advent {
    label: Label,
    equations: Vec<Equation>,
    do_bruteforce: bool
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(7),
            equations: Vec::new(),
            do_bruteforce: false
        }
    }
}

impl Advent{
    fn solve_bruteforce(&self, convertable: fn((usize, usize))->Convertable) -> usize{
        self.equations
            .par_iter()
            .filter(|&e| {
                e.try_solve_bruteforce(convertable)
            })
            .map(|e|{e.lhs})
            .sum::<usize>()
    }

    fn solve_stack(&self, concat_allowed: bool) -> usize{
        self.equations
            .par_iter()
            .filter(|&e| {
                e.try_solve(concat_allowed)
            })
            .map(|e|{e.lhs})
            .sum::<usize>()
    }

    fn solve(&self,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8) -> Result<String, String>{
        self.check_input(Some(part))?;
        let result = {
            if part == 1 {
                if self.do_bruteforce {
                    self.solve_bruteforce(Convertable::Binary)
                } else {
                    self.solve_stack(false)
                }
            }
            else if part == 2 {
                if self.do_bruteforce {
                    self.solve_bruteforce(Convertable::Trinary)
                } else {
                    self.solve_stack(true)
                }
            }
            else{
                return Err(format!("Unknown part {}", part));
            }
        };
        assert_display(result, Some(result_test), result_prd, "Sum of solvable equations", test_mode)
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

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of equations: {}", self.equations.len());
        let mut count = BTreeMap::new();
        self.equations.iter().for_each(|e| *count.entry(e.rhs.len()).or_insert(0) += 1);

        for (key, value) in count {
            println!(
                "\tNumber of equations with {} argument{}: {}",
                key,
                if key > 1 { "s" } else { "" },
                value
            );
        }
        Ok(())
    }

    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(3749,
                   1582598718861,
                   test_mode,
                   1
            )
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(11387,
                              165278151522644,
                              test_mode,
                              2
        )
    }
}


