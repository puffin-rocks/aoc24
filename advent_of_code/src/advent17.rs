use std::collections::HashMap;
use crate::utils::{Solve, Label, assert_display};

const A: char = 'A';
const B: char = 'B';
const C: char = 'C';
pub(crate) struct Advent {
    label: Label,
    registers: HashMap<char, usize>,
    program: Vec<u8>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(17),
            registers: HashMap::new(),
            program: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((label, value)) = line.split_once(": ") {
            if label.contains("Register") {
                if let Some((_, name)) = label.split_once(" "){
                    if let Some(ch) = name.chars().nth(0) {
                        self.registers.insert(ch, value.parse::<usize>()?);
                    }
                }
            }
            else {
                self.program = value.split(",").map(|x| x.chars().nth(0).unwrap().to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("{:?}", self.registers);
        println!("{:?}", self.program);

        //calibration
        let mut registers: HashMap<char, usize> = HashMap::new();
        //Case 1: If register C contains 9, the program 2,6 would set register B to 1.
        registers.insert(C, 9);
        let mut program: Vec<u8> = vec![2, 6];
        execute_program(&mut registers, &program);
        assert_eq!(registers.get(&B), Some(&1_usize));
        //Case 2: If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        registers.clear();
        registers.insert(A, 10);
        program = vec![5,0,5,1,5,4];
        let output = execute_program(&mut registers, &program);
        assert_eq!(output, String::from("0,1,2"));
        //Case 3 If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        registers.clear();
        registers.insert(A, 2024);
        program = vec![0,1,5,4,3,0];
        let output = execute_program(&mut registers, &program);
        assert_eq!(output, String::from("4,2,5,6,7,7,7,7,3,1,0"));
        assert_eq!(registers.get(&A), Some(&0_usize));
        //Case 4 If register B contains 29, the program 1,7 would set register B to 26.
        registers.clear();
        registers.insert(B, 29);
        program = vec![1,7];
        execute_program(&mut registers, &program);
        assert_eq!(registers.get(&B), Some(&26_usize));
        //Case 5: If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        registers.clear();
        registers.insert(B, 2024);
        registers.insert(C, 43690);
        program = vec![4,0];
        execute_program(&mut registers, &program);
        assert_eq!(registers.get(&B), Some(&44354_usize));
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut registers = self.registers.clone();
        let output = execute_program(&mut registers, &self.program);
        assert_display(output,
                       Some(String::from("4,6,3,5,6,3,5,2,1,0")),
                       String::from("1,5,0,3,7,3,0,3,1"), "Program output", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}

fn operand_value(operand: &u8, registers: &HashMap<char, usize>, combo: bool) -> usize{
    if combo {
        match operand {
            0 | 1 | 2 | 3 => *operand as usize,
            4 => *registers.get(&'A').expect("Register A does not exist"),
            5 => *registers.get(&'B').expect("Register B does not exist"),
            6 => *registers.get(&'C').expect("Register C does not exist"),
            _ => unreachable!()
        }
    }else{
        *operand as usize
    }
}
fn division(value: usize, registers: &HashMap<char, usize>)-> usize{
    //The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand.
    let num = *registers.get(&A).expect("Register A does not exist");
    let den = 2u32.pow(value as u32) as usize;
    num/den
}

fn execute_program(registers: &mut HashMap<char, usize>, program: &Vec<u8>)->String{
    let mut output: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while let (Some(opcode), Some(operand)) = (program.get(i), program.get(i+1)) {
        //println!("{:?}", (opcode, operand));

        let mut increment = 2;
        match opcode {
            0 => {// The adv instruction (opcode 0) performs division.
                let value = operand_value(operand, registers, true);
                registers.insert(A, division(value, registers));
            },
            1 => {
                let value = operand_value(operand, registers, false);
                registers.insert(B,*registers.get(&B).expect("Register B does not exist") ^ value);
            },
            2 => { // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                let value = operand_value(operand, registers, true);
                registers.insert(B, value % 8);
            },
            3 => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
                // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if *registers.get(&A).expect("Register A does not exist") !=0{
                    i = operand_value(operand, registers, false);
                    increment = 0;
                }
            },
            4 => {
                registers.insert(B,*registers.get(&B).expect("Register B does not exist") ^ *registers.get(&C).expect("Register C does not exist"));
            },
            5 => { //The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let value = operand_value(operand, registers, true);
                output.push((value % 8) as u8);
            }
            6 => {// The adv instruction (opcode 0) performs division.
                let value = operand_value(operand, registers, true);
                registers.insert(B, division(value, registers));
            },
            7 => {// The adv instruction (opcode 0) performs division.
                let value = operand_value(operand, registers, true);
                registers.insert(C, division(value, registers));
            },
            _ => unreachable!()
        }
        i+=increment;
    }
    output.iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// Combo operands
// 0-3: represent literal values 0 through 3.
// 4-6: represents the value of register A-C.
// Combo operand 7 is reserved and will not appear in valid programs.


// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
// then stores the result in register B.

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
// then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)


