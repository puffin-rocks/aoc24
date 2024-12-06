use itertools::izip;
use crate::utils::{line2vec_i32, Solve, Label, no_solution_message};

struct Report{
    levels: Vec<i32>,
    use_bruteforce: bool
}

fn check_level_safety(levels: Vec<&i32>) -> (bool, Option<usize>) {
    let n = levels.len();
    if n<1 {return (false, None)}
    let mut direction:Option<bool> = None;
    for (e, (&current, &next)) in izip!(levels[..(n-1)].iter(), levels[1..].iter()).enumerate(){
        let step = next-current;
        if (step==0) | (step.abs()>3) {return (false, Some(e+1))}
        match direction {
            None => {direction = Some(step>0)}
            Some(d) => {
                if (step>0)!=d {return (false, Some(e+1))}
            }
        }
    }
    (true, None)
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Self {
            levels,
            use_bruteforce: false
        }
    }

    fn check_safety(&self, zero_tolerance: bool) -> bool {
        let (result, index) = check_level_safety(self.levels.iter().collect::<Vec<&i32>>());
        match index {
            None => {result}
            Some(e) => {
                if zero_tolerance {return false}

                if self.use_bruteforce {
                    for shift in 0..self.levels.len() {
                        let fix: Vec<&i32> = self.levels.iter()
                            .take(shift)
                            .chain(self.levels.iter().skip(shift + 1))
                            .collect();
                        let (result_fix, _) = check_level_safety(fix);
                        if result_fix { return result_fix }
                    }
                    false
                }
                else {
                    for shift in 0..=1 {
                        let fix: Vec<&i32> = self.levels.iter()
                            .take(e - shift)
                            .chain(self.levels.iter().skip(e - shift + 1))
                            .collect();
                        let (result_fix, _) = check_level_safety(fix);
                        if result_fix { return result_fix }
                    }
                    let fix: Vec<&i32> = self.levels.iter().skip(1).collect();
                    let (result, _) = check_level_safety(fix);
                    result
                }
            }
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    reports: Vec<Report>,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(2),
            reports: Vec::new(),
        }
    }
}

impl Advent {
    fn count_safe_report(&self, zero_tolerance: bool) -> u32{
        let mut n_safe_reports = 0;
        for r in &self.reports{
            if r.check_safety(zero_tolerance){
                n_safe_reports+=1;
            }
        }
        n_safe_reports
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        self.reports.push(Report::new(line2vec_i32(line)?));
        Ok(())
    }

    fn info(&self){
        if !self.label.has_input { return println!("Advent is missing input")}
        println!("Number of reports is: {}", self.reports.len());
    }

    fn compute_part1_answer(&self, verbose: bool, _: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }
        let n_safe_reports = self.count_safe_report(true);
        assert_eq!(n_safe_reports, 224);
        if verbose {
            println!("Number of save reports (zero tolerance) is: {}", n_safe_reports);
        }
        true
    }

    fn compute_part2_answer(&self, verbose: bool, _: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 2) }
        let n_safe_reports = self.count_safe_report(false);
        assert_eq!(n_safe_reports, 293);
        if verbose {
            println!("Number of save reports (single bad level) is: {}", n_safe_reports);
        }
        true
    }
}