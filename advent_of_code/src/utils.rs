use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) const PUZZLE_INPUT: &str  = "src/puzzle_input/";

pub (crate) struct Label {
    pub(crate) number: u8,
    pub(crate) has_input: bool
}


impl Label {
    pub fn new(number: u8) -> Self{
        Self{
            number,
            has_input: false
        }
    }

    pub fn get_puzzle_input_path(&self, test_mode: bool) -> String{
        if test_mode {
            PUZZLE_INPUT.to_owned() + format!("day{:0>2}_test.txt", self.number).as_str()
        }
        else{
            PUZZLE_INPUT.to_owned() + format!("day{:0>2}.txt", self.number).as_str()
        }
    }
}

pub(crate) trait Solve
{
    fn get_label(&self) -> &Label;
    fn get_label_mut(&mut self) -> &mut Label;
    fn add_record_from_line(&mut self, _: String) -> Result<(), std::num::ParseIntError> {
        "invalid".parse::<i32>()?;
        Ok(())
    }

    fn read_input(&mut self, test_mode: bool) -> Result<(), std::num::ParseIntError>{
        let filename = self.get_label().get_puzzle_input_path(test_mode);

        if let Ok(lines) = read_lines(filename) {
            for line in lines.flatten() {
                self.add_record_from_line(line)?;
            }
            self.get_label_mut().has_input = true;
        }
        Ok(())
    }

    fn info(&self){
        println!("Advent is missing input");
    }
    fn compute_part1_answer(&self, verbose: bool, _test_mode: bool) -> bool{
        no_solution_message(verbose, 1)
    }
    fn compute_part2_answer(&self, verbose: bool, _test_mode: bool) -> bool{
        no_solution_message(verbose, 2)
    }
}


pub fn no_solution_message(verbose: bool, part: u8) -> bool {
    if verbose {
        println!("Part {} not solved", part);
    }
    false
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn line2vec_i32(line: String) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut v: Vec<i32> = Vec::new();
    for p in line.split_whitespace() {
        v.push(p.parse::<i32>()?); // Propagate error using `?`
    }
    Ok(v)
}