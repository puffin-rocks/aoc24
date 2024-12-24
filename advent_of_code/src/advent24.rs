use std::collections::{BTreeMap, HashMap, VecDeque};
use itertools::Itertools;
use crate::utils::{Solve, Label, assert_display};

#[derive(Debug, Clone)]
enum Gate{
    AND([char;3], [char;3], [char;3]),
    OR([char;3], [char;3], [char;3]),
    XOR([char;3], [char;3], [char;3]),
}

impl Gate{
    fn evaluate(&self, wires: &mut HashMap<[char;3],bool>)->Result<(),()>{
        match self{
            Gate::AND(w1, w2, out ) =>{
                if let (Some(&w1), Some(&w2)) = (wires.get(w1), wires.get(w2)){
                    wires.insert(*out, w1 & w2);
                    Ok(())
                }else{
                    Err(())
                }
            },
            Gate::OR(w1, w2, out )  => {
                if let (Some(&w1), Some(&w2)) = (wires.get(w1), wires.get(w2)){
                    wires.insert(*out, w1 | w2);
                    Ok(())
                }else{
                    Err(())
                }
            },
            Gate::XOR(w1, w2, out )  =>{
                if let (Some(&w1), Some(&w2)) = (wires.get(w1), wires.get(w2)){
                    wires.insert(*out, w1 ^ w2);
                    Ok(())
                }else{
                    Err(())
                }
            }
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    wires: HashMap<[char;3],bool>,
    gates: Vec<Gate>,
    read_wires: bool
}

// fn bools_to_binary(bools: &[bool]) -> u32 {
//     bools.iter().fold(0, |acc, &b| (acc << 1) | (b as u32))
// }


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(24),
            wires: HashMap::new(),
            gates: Vec::new(),
            read_wires: true
        }
    }
}

fn to_char3(v: &str) -> Option<[char;3]>{
    let mut chars = v.chars();
    Some([
        chars.next()?,
        chars.next()?,
        chars.next()?,
    ])
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if line.is_empty(){
            self.read_wires = false;
        }else{
            if self.read_wires{
                if let Some((k,v)) = line.split_once(": "){
                    if let Some(k) = to_char3(k) {
                        self.wires.insert(k, v.parse::<u8>()? != 0);
                    }
                }
            }else{
                if let Some((gate_def,out)) = line.split_once(" -> "){
                    if let Some(out) = to_char3(out) {
                        if let Some((w1, w2)) = gate_def.split_once(" AND ") {
                            if let (Some(w1), Some(w2)) = (to_char3(w1), to_char3(w2)){
                                self.gates.push(Gate::AND(w1,w2,out));
                            }
                        } else if let Some((w1, w2)) = gate_def.split_once(" XOR ") {
                            if let (Some(w1), Some(w2)) = (to_char3(w1), to_char3(w2)){
                                self.gates.push(Gate::XOR(w1,w2,out));
                            }
                        }
                        else if let Some((w1, w2)) = gate_def.split_once(" OR ") {
                            if let (Some(w1), Some(w2)) = (to_char3(w1), to_char3(w2)){
                                self.gates.push(Gate::OR(w1,w2,out));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of wires {}", self.wires.len());
        println!("Number of gates {}", self.gates.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut wires = self.wires.clone();
        let mut queue: VecDeque<Gate> = VecDeque::new();
        queue.extend(self.gates.iter().cloned());
        while !queue.is_empty(){
            if let Some(g) = queue.pop_front(){
                match g.evaluate(&mut wires){
                    Err(_) => {
                        queue.push_back(g);
                    }
                    Ok(_) => {}
                }
            }
        }
        let result: usize = wires.iter()
            .filter(|(k,_)| k[0]=='z')
            .sorted_by(|(&k0,_), (k1, _)| k0.cmp(k1))
            .map(|(&k, &v)| (k[1..3].iter().collect::<String>().parse().unwrap(), v)) // Ensure owned keys and values
            .collect::<BTreeMap<usize, bool>>()
            .values().rev().fold(0, |acc, &b| (acc << 1) | (b as usize));
        assert_display(result, Some(2024), 50411513338638, "Z-binary output", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}