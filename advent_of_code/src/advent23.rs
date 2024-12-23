use std::collections::HashSet;
use std::rc::Rc;
use crate::hashset;
use crate::utils::{Solve, Label};
use crate::utils::*;

pub(crate) struct Advent {
    label: Label,
    edges: Vec<HashSet<Rc<String>>>,
    vertices: HashSet<Rc<String>>,
    d_edges: HashSet<(Rc<String>, Rc<String>)>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(23),
            edges: Vec::new(),
            vertices: HashSet::new(),
            d_edges: HashSet::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((el1, el2)) = line.split_once("-"){
            let (el1, el2) = (el1.to_string(), el2.to_string());
            let (el1, el2) = match (self.vertices.get(&el1), self.vertices.get(&el2)){
                (Some(el1), Some(el2)) => (el1.clone(), el2.clone()),
                (Some(el1), None) => {
                    let el2 = Rc::new(el2);
                    (el1.clone(), el2)
                },
                (None, Some(el2)) => {
                    let el1 = Rc::new(el1);
                    (el1, el2.clone())
                },
                (None, None) => {
                    let el1 = Rc::new(el1);
                    let el2 = Rc::new(el2);
                    (el1, el2)
                }
            };
            self.vertices.insert(el1.clone());
            self.vertices.insert(el2.clone());
            self.d_edges.insert((el1.clone(), el2.clone()));
            self.d_edges.insert((el2.clone(), el1.clone()));
            self.edges.push(hashset!(el1, el2));
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
        let mut result: HashSet<Vec<Rc<String>>> = HashSet::new();
        for v in self.vertices.iter(){
            for e in self.edges.iter(){
                if !e.contains(v) &&
                    e.iter().all( |v_other|
                        self.d_edges.contains(&(v.clone(), v_other.clone()))){
                    let mut vec:Vec<Rc<String>> = e.iter().cloned().collect();
                    vec.push(v.clone());
                    vec.sort_unstable();
                    result.insert(vec);
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
        assert_display(cnt, Some(7), 1238, "Number of sets", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}