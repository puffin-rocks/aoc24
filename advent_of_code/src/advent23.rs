use std::collections::HashSet;
use itertools::Itertools;
use crate::utils::{Solve, Label};

pub(crate) struct Advent {
    label: Label,
    edges: Vec<HashSet<String>>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(23),
            edges: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((el1, el2)) = line.split_once("-"){
            let mut set: HashSet<String> = HashSet::new();
            set.insert(el1.to_string());
            set.insert(el2.to_string());
            self.edges.push(set);
        }else{
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of edges: {}", self.edges.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        //super slow
        //1238
        self.check_input(Some(1))?;
        let mut result:HashSet<Vec<String>> = HashSet::new();
        for edge0 in self.edges.iter(){
            for e in edge0.iter(){
                let mut s = HashSet::new();
                s.insert(e.clone().to_string());
                let other0 = edge0.difference(&s).cloned().collect::<HashSet<String>>();
                for edge1 in self.edges.iter(){
                    if edge1.contains(e){
                        let mut other1 = edge1.difference(&s).cloned().collect::<HashSet<String>>();
                        other1.extend(other0.iter().cloned());
                        if self.edges.contains(&other1){
                            other1.extend(s.iter().cloned());
                            let mut vec:Vec<String> = other1.iter().cloned().collect();
                            vec.sort_unstable();
                            result.insert(vec);
                        }
                    }
                }
            }
        }
        let mut cnt = 0;
        for set in result.iter(){
            for e in set.iter(){
                if e.starts_with('t'){
                    cnt+=1;
                    break;
                }
            }
        }
        println!("{}", cnt);
        Err(String::from("Not solved yet"))
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}