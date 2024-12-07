use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    memory: String,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(3),
            memory: String::new(),
        }
    }
}

impl Advent {
    fn sum_uncorrupted_instructions(&self,
                                    always_enabled: bool,
                                    result_prd: usize,
                                    part: u8
    ) -> Result<String, String>{
        self.check_input(Some(part))?;

        let mut sum = 0;
        let mut enabled = 1;

        for (e, p) in self.memory.split("mul(").enumerate() {
            if e == 0 {
                continue; // Skip the first one
            }

            if let Some(index) = p.find(')') { //find closing bracket
                if let [x1, x2] = p[..index].split(",").collect::<Vec<&str>>().as_slice() { //check that there is exactly two parts
                    if let (Ok(v1), Ok(v2)) = (x1.parse::<u32>(), x2.parse::<u32>()) {
                        sum += v1 * v2 * enabled;
                    }
                }
            }
            if !always_enabled {
                enabled = match (p.rfind("don't()"), p.rfind("do()")) {
                    (Some(ix_disable), Some(ix_enable)) => (ix_disable < ix_enable) as u32,
                    (None, Some(_)) => 1,
                    (Some(_), None) => 0,
                    (None, None) => enabled,
                };
            }
        }
        let header = if always_enabled{
            "Total sum of uncorrupted mul instructions"
        }else{
            "Total sum of uncorrupted enabled mul instructions"
        };
        assert_display(sum as usize, None, result_prd, header, false)
    }
}


impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.memory.push('\n');
        self.memory.push_str(line.as_str());
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Length of memory is {}", self.memory.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.sum_uncorrupted_instructions( true,
                                                     187825547,
                                                     1
        )
    }
    fn compute_part2_answer(&self, _: bool) -> Result<String, String>{
        self.sum_uncorrupted_instructions( false,
                                           85508223,
                                           2
        )
    }
}



