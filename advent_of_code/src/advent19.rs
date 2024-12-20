use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashSet};
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    read_patterns: bool,
    patterns: HashSet<String>,
    towels: Vec<String>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(19),
            read_patterns: true,
            patterns: HashSet::new(),
            towels: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if line.is_empty(){
            self.read_patterns=false;
        }
        else {
            if self.read_patterns {
                self.patterns.extend(line.split(", ").map(|x| String::from(x)).collect::<Vec<_>>());
            } else {
                self.towels.push(line);
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of patterns: {}", self.patterns.len());
        println!("Number of towels: {}", self.towels.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n: usize = self.towels.iter().map(|t|
           if match_towel(t, &self.patterns) {1usize} else {0usize}
        ).sum();
        assert_display(n, Some(6), 238, "Number of matchable towels", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}

fn match_towel(towel: &String, patterns: &HashSet<String>) -> bool{
    let mut queue: BTreeSet<String> = BTreeSet::new();
    queue.insert(towel.clone());

    while let Some(part) = queue.pop_last() {
        for p in patterns.iter(){
            if part.starts_with(p){
                if part==*p{
                    return true
                }
                queue.insert(part[p.len()..].to_string());
            }
        }
    }
    false
}

fn _match_towel(towel: &String, patterns: &HashSet<String>) -> bool{
    let mut queue: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    queue.push(Reverse(towel.clone()));
    let mut matchable: bool = false;

    while let Some(Reverse(part)) = queue.pop() {
        println!("{:?}", part);
        let mut found_next_match = false;
        patterns.iter().for_each(|p| {
            println!("{:?}", p);
            if part==*p{
                matchable = true;
            }
            if part.starts_with(p){
                queue.push(Reverse(part[p.len()..].to_string()));
                found_next_match = true;
            }
        });
        println!("{:?}", queue);
        if !found_next_match || matchable{
            break
        }
    }
    println!("{:?}", matchable);
    matchable
}