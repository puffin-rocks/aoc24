use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::geometry::{Canvas, CanvasAsync};

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

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        None
    }

    fn get_canvas_async_mut(&mut self) -> Option<&mut CanvasAsync>{
        None
    }

    fn apply_bruteforce(&mut self){}

    fn check_input(&self, part: Option<u8>) -> Result<(), String> {
        if self.get_label().has_input {
            Ok(())
        }
        else
        {
            match part{
                Some(part) => Err(no_solution_message(part)),
                None => Err(String::from("Advent is missing input"))
            }
        }
    }
    fn add_record_from_line(&mut self, line : String) -> Result<(), std::num::ParseIntError> {
        match self.get_canvas_mut(){
            None => {
                match self.get_canvas_async_mut(){
                    None => {
                        "invalid".parse::<i32>()?;
                    },
                    Some(canvas) => {
                        canvas.add_row(line.chars().collect());
                    }
                };
            },
            Some(canvas) => {
                canvas.add_row(line.chars().collect());
            }
        };

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

    fn info(&self) -> Result<(), String> {
        Err(String::from("Advent is missing input"))
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        Err(no_solution_message(1))
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        Err(no_solution_message(2))
    }
}


pub fn no_solution_message(part: u8) -> String{
    format!("Part {} not solved", part)
}

pub fn assert_display<T: Debug+Display+PartialEq>(result: T,
         result_test: Option<T>,
         result_prd: T,
         header: &str,
         test_mode: bool
) -> Result<String, String>{
    match result_test {
        Some(result_test) => {
            assert_eq!(result, match test_mode{
                true =>  result_test,
                false => result_prd
            });
        },
        None => {
            match test_mode{
                true =>  {return Err(String::from("Test mode not implemented"));},
                false => assert_eq!(result, result_prd)
            }
        }
    };
    Ok(format!("{}: {}", header, result))
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
#[allow(dead_code)]
pub fn write_vec_to_file(vec: Vec<Vec<char>>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for line in vec {
        writeln!(file, "{}", line.iter().collect::<String>())?;
    }
    Ok(())
}