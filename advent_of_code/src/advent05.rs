use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::utils::{Solve, Label, no_solution_message};

#[derive(Debug, PartialEq, Clone)]
struct Page {
    number: usize,
    page_numbers_after: Option<HashSet<usize>>
}


impl Page {
    fn new(number: usize, page_numbers_after: Option<HashSet<usize>>) -> Self{
        Self{
            number,
            page_numbers_after
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self==other {
            return Some(Ordering::Equal);
        }
        match (&self.page_numbers_after, &other.page_numbers_after){
            (Some(after), Some(after_other)) =>{
                if after.contains(&other.number) {
                    Some(Ordering::Less)
                }
                else if after_other.contains(&self.number){
                    Some(Ordering::Greater)
                }
                else {
                    None
                }
            }
            (Some(after), None) =>{
                if after.contains(&other.number) {
                    Some(Ordering::Less)
                }
                else {
                    None
                }
            }
            (None, Some(after_other)) =>{
                 if after_other.contains(&self.number){
                    Some(Ordering::Greater)
                }
                else {
                    None
                }
            }
            (None, None) => None
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    le_dict: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<Page>>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(5),
            le_dict: HashMap::new(),
            updates: Vec::new()
        }
    }
}

impl Advent {
    fn sum_middle_pages(&self, skip_correctly_ordered: bool, fix_uncorrectly_ordered: bool) -> usize{
        self.updates.iter().map(|update| {
            let update_len = update.len();
            let middle = update_len / 2;
            let is_ordered = update.windows(2).all(|w| w[0] < w[1]);

            if is_ordered && !skip_correctly_ordered {
                update[middle].number
            } else if !is_ordered && fix_uncorrectly_ordered {
                let mut sorted_update = update.clone();
                sorted_update.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                sorted_update[middle].number
            } else {
                0
            }
        }).sum()
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if line.is_empty() {
            return Ok(());
        }

        let mut parts = line.split('|');
        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            let entry = self.le_dict.entry(x.parse::<usize>()?).or_insert_with(HashSet::new);
            entry.insert(y.parse::<usize>()?);
        } else {
            let pages: Vec<Page> = line
                .split(',')
                .map(|pn| pn.parse::<usize>().expect(&format!("Cannot parse page number {}", pn)))
                .map(|pn| Page::new(pn, self.le_dict.get(&pn).cloned()))
                .collect();
            self.updates.push(pages);
        }
        Ok(())
    }

    fn info(&self){
        if !self.label.has_input {println!("Advent is missing input")}
        println!("Ordering dictionary of length {}", self.le_dict.len());
        println!("Number of updates {}", self.updates.len());
    }

    fn compute_part1_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input { return no_solution_message(verbose, 1) }
        let sum = self.sum_middle_pages(false,false);
        assert_eq!(sum, match test_mode{
            true => 143,
            false => 3608
        });
        if verbose {
            println!("Sum of middle pages of correctly ordered updates: {}", sum);
        }
        true
    }

    fn compute_part2_answer(&self, verbose: bool, test_mode: bool) -> bool{
        if !self.label.has_input  { return no_solution_message(verbose, 2) }
        let sum = self.sum_middle_pages(true,true);
        assert_eq!(sum, match test_mode{
            true => 123,
            false => 4922
        });
        if verbose {
            println!("Sum of middle pages of re-ordered updates: {}", sum);
        }
        true
    }
}