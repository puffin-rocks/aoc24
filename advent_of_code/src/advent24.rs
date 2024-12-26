use std::collections::{BTreeMap, HashMap, VecDeque};
use std::rc::Rc;
use itertools::Itertools;
use crate::utils::{Solve, Label, assert_display, vec2line};

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
enum Operator{
    AND,
    OR,
    XOR
}

impl Eq for Operator {}
#[derive(Debug, Clone)]
struct Gate{
    input1: [char;3],
    input2: [char;3],
    output: [char;3],
    operator: Operator
}

impl Gate{
    fn new(input1: [char;3], input2: [char;3], output: [char;3], operator: Operator)->Self{
        Self{
            input1,
            input2,
            output,
            operator
        }
    }
    fn write(&self, gate_map: &mut HashMap<([char;3], [char;3], Operator), Rc<Gate>>){
        let g = Rc::new(self.clone());
        gate_map.insert((self.input1, self.input2, self.operator), g.clone());
        gate_map.insert((self.input2, self.input1, self.operator), g.clone());
    }
    fn evaluate(&self, wires: &mut HashMap<[char;3],bool>)->Result<(),()>{
        if let (Some(&w1), Some(&w2)) = (wires.get(&self.input1), wires.get(&self.input2)){
            let value = match &self.operator {
                Operator::AND => {
                    w1 & w2
                },
                Operator::OR => {
                    w1 | w2
                },
                Operator::XOR => {
                    w1 ^ w2
                }
            };
            wires.insert(self.output, value);
            Ok(())
        }else{
            Err(())
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    wires: HashMap<[char;3],bool>,
    gates: Vec<Gate>,
    read_wires: bool
}

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
                        let (w1, w2, operator) = if let Some((w1, w2)) = gate_def.split_once(" AND ") {
                            (w1,w2, Operator::AND)
                        } else if let Some((w1, w2)) = gate_def.split_once(" XOR ") {
                            (w1,w2, Operator::XOR)
                        }
                        else if let Some((w1, w2)) = gate_def.split_once(" OR ") {
                            (w1,w2, Operator::OR)
                        }else{
                            unreachable!()
                        };
                        if let (Some(w1), Some(w2)) = (to_char3(w1), to_char3(w2)){
                            self.gates.push(Gate::new(w1,w2,out, operator));
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
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let width = self.wires.len()/2;
        let mut gate_map: HashMap<([char;3], [char;3], Operator), Rc<Gate>> = HashMap::new();
        for g in self.gates.iter(){
            g.write(&mut gate_map);
        }
        let mut wires = self.wires.clone();
        let mut to_swap: Vec<String> = Vec::new();
        while let Some((g1, g2)) = check(&gate_map, &mut wires, width){
            to_swap.push(g1.output.iter().join(""));
            to_swap.push(g2.output.iter().join(""));
            gate_map.insert((g1.input1, g1.input2, g1.operator), g1.clone());
            gate_map.insert((g1.input2, g1.input1, g1.operator), g1.clone());
            gate_map.insert((g2.input1, g2.input2, g2.operator), g2.clone());
            gate_map.insert((g2.input2, g2.input1, g2.operator), g2.clone());
        }
        to_swap.sort_unstable();
        assert_display(vec2line(to_swap),None, String::from("gfv,hcm,kfs,tqm,vwr,z06,z11,z16"), "Wires to swap", test_mode)
    }
}

fn check(
    gate_map: &HashMap<([char; 3], [char; 3], Operator), Rc<Gate>>,
    wires: &mut HashMap<[char; 3], bool>,
    width: usize,
) -> Option<(Rc<Gate>, Rc<Gate>)> {
    let mut curr_position = 0;
    let mut carry;
    let mut x = to_char3(&format!("x{:02}", curr_position))?;
    let mut y = to_char3(&format!("y{:02}", curr_position))?;

    if let (Some(z), Some(c)) = (
        gate_map.get(&(x, y, Operator::XOR)),
        gate_map.get(&(x, y, Operator::AND)),
    ) {
        z.evaluate(wires).ok()?;
        c.evaluate(wires).ok()?;

        let key1 = to_char3(&format!("z{:02}", curr_position))?;
        if wires.get(&key1) != wires.get(&z.output) {
            let g_c = Rc::new(Gate::new(x, y, z.output, Operator::AND));
            let g_z = Rc::new(Gate::new(x, y, c.output, Operator::XOR));
            return Some((g_z, g_c));
        }
        carry = c.output;
        curr_position += 1;
    } else {
        return None;
    }

    while curr_position < width {
        x = to_char3(&format!("x{:02}", curr_position))?;
        y = to_char3(&format!("y{:02}", curr_position))?;

        if let (Some(v), Some(t)) = (
            gate_map.get(&(x, y, Operator::XOR)),
            gate_map.get(&(x, y, Operator::AND)),
        ) {
            if let (Some(z), Some(f)) = (
                gate_map.get(&(v.output, carry, Operator::XOR)),
                gate_map.get(&(v.output, carry, Operator::AND)),
            ) {
                if let Some(c) = gate_map.get(&(f.output, t.output, Operator::OR)) {
                    v.evaluate(wires).ok()?;
                    t.evaluate(wires).ok()?;
                    z.evaluate(wires).ok()?;
                    f.evaluate(wires).ok()?;
                    c.evaluate(wires).ok()?;

                    let key1 = to_char3(&format!("z{:02}", curr_position))?;
                    if wires.get(&key1) != wires.get(&z.output) {
                        let g_c = Rc::new(Gate::new(f.output, t.output, z.output, Operator::OR));
                        let g_z = Rc::new(Gate::new(v.output, carry, c.output, Operator::XOR));
                        return Some((g_c, g_z));
                    }
                    carry = c.output;
                    curr_position += 1;
                } else if gate_map.get(&(f.output, z.output, Operator::OR)).is_some() {
                    let g_t = Rc::new(Gate::new(x, y, z.output, Operator::AND));
                    let g_z = Rc::new(Gate::new(v.output, carry, t.output, Operator::XOR));
                    return Some((g_t, g_z));
                } else if gate_map.get(&(t.output, z.output, Operator::OR)).is_some() {
                    let g_f = Rc::new(Gate::new(v.output, carry, z.output, Operator::AND));
                    let g_z = Rc::new(Gate::new(v.output, carry, f.output, Operator::XOR));
                    return Some((g_f, g_z));
                } else {
                    unreachable!();
                }
            } else {
                let g_t = Rc::new(Gate::new(x, y, v.output, Operator::AND));
                let g_v = Rc::new(Gate::new(x, y, t.output, Operator::XOR));
                return Some((g_v, g_t));
            }
        } else {
            unreachable!();
        }
    }

    None
}