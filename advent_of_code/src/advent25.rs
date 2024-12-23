use crate::utils::{Solve, Label};

pub(crate) struct Advent {
    label: Label,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(25),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, _line: String) -> Result<(), std::num::ParseIntError> {
        "invalid".parse::<i32>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        Err(String::from("Advent is missing input"))
    }
    // fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(1))?;
    //     Err(String::from("Not solved yet"))
    // }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}