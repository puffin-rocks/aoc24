use std::collections::HashMap;
use crate::advent22::Operation::{Div32, Mult2048, Mult64};
use crate::utils::{Solve, Label, assert_display};
use itertools::{izip, Itertools};
use rayon::prelude::*;

pub(crate) struct Advent {
    label: Label,
    numbers: Vec<usize>,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(22),
            numbers: Vec::new(),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        self.numbers.push(line.parse::<usize>()?);
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number count: {}", self.numbers.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        return Err(String::from("Skip"));
        let result: usize = self.numbers.iter().map(|n|{
            let mut sn = *n;
            let mut cnt = 0usize;
            while cnt<2000{
                sn = operation(operation(operation(sn,Mult64),Div32),Mult2048);
                cnt+=1;
            }
            sn
        }).sum();
        assert_display(result, Some(37990510), 18261820068, "Sum of 2000th numbers", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //23: 7+7+9 (-2,1,-1,3)
        self.check_input(Some(2))?;
        let n_changes = 2000;
        let prices_changes: Vec<(Vec<usize>, Vec<isize>)> = self.numbers.par_iter().map(|n|{
            let mut sn = *n;
            let mut curr_price = sn%10;
            let mut prices: Vec<usize> = vec![curr_price];
            let mut changes: Vec<isize> = Vec::new();
            let mut cnt = 0usize;
            while cnt<n_changes{
                sn = operation(operation(operation(sn,Mult64),Div32),Mult2048);
                let price = sn%10;
                changes.push(price as isize - curr_price as isize);
                curr_price = price;
                prices.push(curr_price);
                cnt+=1;
            }
            (prices,changes)
        }).collect();
        let patterns: Vec<HashMap<[isize;4], usize>> = prices_changes.par_iter()
            .map(|(prices, changes)|{
                let mut map: HashMap<[isize;4], usize> = HashMap::new();
                for (p, pattern) in izip!(prices.windows(4).skip(1), changes.windows(4)){
                    if !map.contains_key(pattern){
                        map.insert(<[isize; 4]>::try_from(pattern).unwrap(), p[3]);
                    }
                }
                map
        }).collect();
        let mut pattern_cross_count: HashMap<[isize;4], (usize, usize)> = HashMap::new();
        for m in patterns.iter(){
            for (pattern, price) in m.iter(){
                let entry = pattern_cross_count.entry(pattern.clone()).or_insert((0,0));
                entry.0+=1;
                entry.1+=*price;
            }
        }
        let mut cnt = 0;
        for x in pattern_cross_count.iter().sorted_by(|(_,(_,v0)), (_, (_, v1))| v1.cmp(v0)){
            println!("{:?}", x);
            cnt+=1;
            if cnt>10{
                break
            }
        }
        //println!("{:?}", patterns);
        //2044
        Err(String::from("Not solved yet"))
    }
}

fn mix(secret_number: usize, component: usize) -> usize {
    secret_number ^ component
}

fn prune(secret_number: usize) -> usize {
    //secret_number & ((1<<24) - 1)
    secret_number%16777216
}

enum Operation{
    Mult64,
    Div32,
    Mult2048
}

fn operation(secret_number: usize, operation: Operation)->usize{
    prune(mix(secret_number, match operation{
        Mult64 => secret_number<<6,
        Div32 => secret_number>>5,
        Mult2048 =>secret_number<<11
    }))
}

// Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
// Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
// Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
//
// Each step of the above process involves mixing and pruning:
//
// To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
// To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
