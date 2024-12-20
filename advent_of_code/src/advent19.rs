use std::collections::{BTreeMap, BTreeSet};
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    read_patterns: bool,
    patterns: BTreeSet<String>,
    towels: Vec<String>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(19),
            read_patterns: true,
            patterns: BTreeSet::new(),
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
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let n: usize = self.towels.iter().map(|t|
             match_towel_count(t, &self.patterns)
        ).sum();
        assert_display(n, Some(16), 635018909726691, "Number of possible matchings", test_mode)
    }
}

fn match_towel(towel: &String, patterns: &BTreeSet<String>) -> bool{
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

fn match_towel_count(towel: &String, patterns: &BTreeSet<String>) -> usize{
    let mut queue: BTreeMap<String, (String, usize)> = BTreeMap::new();
    queue.insert(String::from(""),(towel.clone(), 1));
    while let Some((head, (tail, cnt))) = queue.pop_first(){
        for p in patterns.iter(){
            if tail.starts_with(p){
                let mut new_key = head.clone();
                new_key.push_str(p);
                match queue.get_mut(&new_key){
                    None =>{
                        queue.insert(new_key, (tail[p.len()..].to_string().clone(), cnt));
                    }
                    Some(entry)=>{
                        entry.1+=cnt;
                    }
                }
            }
        }
        if queue.len()==1 && queue.contains_key(towel){
            break;
        }
    }
    queue.get(towel).unwrap_or(&(String::new(),0)).1
}
