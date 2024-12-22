use crate::advent22::Operation::{Div32, Mult2048, Mult64};
use crate::utils::{Solve, Label, assert_display};

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
        let result: usize = self.numbers.iter().map(|n|{
            let mut sn = *n;
            let mut cnt = 0usize;
            while cnt<2000{
                sn = operation(operation(operation(sn,Mult64),Div32),Mult2048);
                cnt+=1;
            }
            sn
        }).sum();
        assert_display(result, Some(37327623), 18261820068, "Sum of 2000th numbers", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}

fn mix(secret_number: usize, component: usize) -> usize {
    secret_number ^ component
}

fn prune(secret_number: usize) -> usize {
    secret_number%16777216
}

enum Operation{
    Mult64,
    Div32,
    Mult2048
}

fn operation(secret_number: usize, operation: Operation)->usize{
    prune(mix(secret_number, match operation{
        Mult64 => secret_number*64,
        Div32 => secret_number/32,
        Mult2048 =>secret_number*2048
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
