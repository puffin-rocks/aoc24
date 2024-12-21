use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::geometry::{Direction, Point2D};
use crate::utils::{Solve, Label};

pub(crate) struct Advent {
    label: Label,
    codes: Vec<Vec<char>>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(21),
            codes: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
       self.codes.push(line.chars().collect());
       Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of codes: {:?}", self.codes.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let nkp = numeric_keypad();
        let dkp = directional_keypad();

        let result:usize = self.codes.iter().map(|code| {
            if let Some(shortest_seqs) = keypad_input(&code, &nkp) {
                let mut distinct_seq: HashSet<Vec<char>> = HashSet::new();
                for seq in shortest_seqs {
                    if let Some(seqs) = keypad_input(&seq, &dkp) {
                        distinct_seq.extend(seqs);
                    }
                }
                let min_len = distinct_seq.iter().map(|s| s.len()).min();
                let shortest_seqs: HashSet<Vec<char>> = distinct_seq.into_iter().filter(|x| Some(x.len()) == min_len).collect();

                let mut distinct_seq: HashSet<Vec<char>> = HashSet::new();
                for seq in shortest_seqs {
                    if let Some(seqs) = keypad_input(&seq, &dkp) {
                        distinct_seq.extend(seqs);
                    }
                }
                let min_len = distinct_seq.iter().map(|s| s.len()).min().expect(format!("No sequence found for code {:?}", code).as_str());
                let num = code[0..code.len()-1].iter().collect::<String>().parse::<usize>().expect(format!("Cannot parse number from code {:?}", code).as_str());
                min_len*num
            }else{
                panic!("No sequence found for code {:?}", code)
            }
        }).sum();
        println!("{:?}", result);
        //217662
        //how to speed up
        Err(String::from("Not solved yet"))
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}

fn keypad_input(output: &Vec<char>, keypad: &HashMap<Rc<Point2D>, char>) ->Option<HashSet<Vec<char>>>{
    let mut start_position: Option<Rc<Point2D>> = None;
    let prev_char = 'A';
    for (k, v) in keypad.iter(){
        if v == &prev_char{
            start_position = Some(k.clone());
            break;
        }
    }
    let curr_pos = start_position.expect("Cannot find starting position");
    let mut result: HashMap<Rc<Point2D>, Vec<Vec<char>>> = HashMap::new();
    result.insert(curr_pos.clone(), vec![Vec::new()]);
    for curr_ch in output.iter(){
        let mut stack= result.clone();
        result.clear();
        loop{
            let mut next_stack: HashMap<Rc<Point2D>, Vec<Vec<char>>> = HashMap::new();
            for (p, seqs) in stack.iter_mut() {
                if keypad.get(p)==Some(curr_ch){
                    for seq in seqs.iter_mut() {
                        seq.push('A');
                    }
                    result.insert(p.clone(), seqs.clone());
                }else {
                    for d in Direction::base() {
                        let next_pos = p + &d;
                        if keypad.get(&next_pos).is_some() {
                            let mut next_seqs = seqs.clone();
                            for seq in next_seqs.iter_mut() {
                                seq.push(d.to_char());
                            }
                            next_stack.entry(next_pos.clone()).or_insert_with(Vec::new).extend(next_seqs);
                        }
                    }
                }
            }
            if !result.is_empty(){
                break;
            }
            stack = next_stack;
        }
        //println!("{:?}", (curr_ch, &result));
    }
    if result.len()==1{
        result.into_iter().next().map(|(_, v)| v.into_iter().collect::<HashSet<Vec<char>>>())
    }else{
        None
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn point(x: isize, y: isize) -> Rc<Point2D>{
    Rc::new(Point2D::new(x,y))
}
fn numeric_keypad() -> HashMap<Rc<Point2D>, char>{
    let mut keypad:HashMap<Rc<Point2D>, char> = HashMap::new();
    keypad.insert(point(0,0), '7');
    keypad.insert(point(1,0), '8');
    keypad.insert(point(2,0), '9');
    keypad.insert(point(0,1), '4');
    keypad.insert(point(1,1), '5');
    keypad.insert(point(2,1), '6');
    keypad.insert(point(0,2), '1');
    keypad.insert(point(1,2), '2');
    keypad.insert(point(2,2), '3');
    keypad.insert(point(1,3), '0');
    keypad.insert(point(2,3), 'A');
    keypad
}

fn directional_keypad() -> HashMap<Rc<Point2D>, char>{
    let mut keypad:HashMap<Rc<Point2D>, char> = HashMap::new();
    keypad.insert(point(1,0), '^');
    keypad.insert(point(2,0), 'A');
    keypad.insert(point(0,1), '<');
    keypad.insert(point(1,1), 'v');
    keypad.insert(point(2,1), '>');
    keypad
}