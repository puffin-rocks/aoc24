use crate::utils::{Solve, Label, no_solution_message};

pub(crate) struct Advent {
    label: Label,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(7),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, _: String) -> Result<(), std::num::ParseIntError> {
        "invalid".parse::<i32>()?;
        Ok(())
    }

    fn info(&self){
        if !self.label.has_input {println!("Advent is missing input")}
    }

    // fn compute_part1_answer(&self, verbose: bool, test_mode: bool) -> bool{
    //     if !self.label.has_input { return no_solution_message(verbose, 1) }
    //     false
    // }
    //
    // fn compute_part2_answer(&self, verbose: bool, test_mode: bool) -> bool{
    //     if !self.label.has_input  { return no_solution_message(verbose, 2) }
    //     false
    // }
}