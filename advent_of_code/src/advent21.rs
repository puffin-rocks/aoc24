use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;
use itertools::Itertools;
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
        return Err(String::from("Skip"));
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
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let nkp = numeric_keypad();
        let dkp = directional_keypad();
        let max_depth: usize = 25;
        let mut memory_len_base: MemoryLen = HashMap::new();
        let mut memory_seq_base: MemorySeq = HashMap::new();
        let mut unresolved_values: Vec<Vec<Vec<char>>> = Vec::new();
        let mut unresolved_keys: Vec<(char,char)> = Vec::new();
        for from in dkp.values(){
            for to in dkp.values(){
                if let Some(mut seqs) = process_pair(from, to, &dkp){
                    memory_len_base.insert((max_depth, *from, *to), seqs.first().unwrap().len());
                    if seqs.len()>1{
                        seqs.sort_unstable();
                        unresolved_values.push(seqs);
                        unresolved_keys.push((*from, *to));
                    }
                    else{
                        memory_seq_base.insert([*from, *to], seqs.get(0)
                            .expect(format!("Empty sequence for {:?}", (from,to)).as_str())
                            .clone());
                    }

                    // let scores: Vec<(usize, &Vec<char>)> = seqs.iter()
                    //     .map(|vec| (score_sequence(vec), vec)) // Replace `calculate_score` with your scoring function
                    //     .collect();
                    //
                    // let first_lowest_scored = scores
                    //     .iter()
                    //     .sorted_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1))) // Sort by first then second tuple element
                    //     .next().unwrap().1;
                    //memory_seq.insert([*from, *to], first_lowest_scored.clone());
                }
            }
        }
        let mut results: Vec<usize> = Vec::new();
        for x in unresolved_values.into_iter().multi_cartesian_product(){//.collect::<Vec<Vec<Vec<char>>>>()
            let mut memory_len = memory_len_base.clone();
            let mut memory_seq = memory_seq_base.clone();
            let zipped = unresolved_keys.iter().zip(x.iter());
            for (k, v) in zipped{
                memory_seq.insert([k.0, k.1], v.clone());
            }
            let result:usize = self.codes.iter().map(|code| {
                let shortest_seqs = keypad_input(&code, &nkp).expect("got no recursion input");
                let min_len = shortest_seqs.iter().map(
                    |seq| {
                        //println!("{:?}", &seq);
                        let mut seq = seq.clone();
                        seq.insert(0, 'A');
                        recursion(&seq, &mut memory_len, &memory_seq, 1, max_depth)
                    }
                ).min().expect("No min element");
                let num = code[0..code.len() - 1].iter()
                    .collect::<String>().parse::<usize>().expect(format!("Cannot parse number from code {:?}", code).as_str());
                //println!("{:?}", min_len);
                min_len * num
            }
            ).sum();
            results.push(result);
        }
        println!("{:?}", results.iter().min());
        // for e in memory_seq.iter() {
        //     println!("{:?}", e);
        // }
        // let result:usize = self.codes.iter().map(|code| {
        //     let shortest_seqs = keypad_input(&code, &nkp).expect("got no recursion input");
        //     let min_len = shortest_seqs.iter().map(
        //         |seq| {
        //             //println!("{:?}", &seq);
        //             let mut seq = seq.clone();
        //             seq.insert(0, 'A');
        //             recursion(&seq, &mut memory_len, &memory_seq, 1, max_depth)
        //         }
        //     ).min().expect("No min element");
        //     let num = code[0..code.len() - 1].iter()
        //         .collect::<String>().parse::<usize>().expect(format!("Cannot parse number from code {:?}", code).as_str());
        //     println!("{:?}", min_len);
        //     min_len * num
        // }
        // ).sum();
        //119976732336462 too low
        //300324548443822 too high

        //println!("{:?}", result);
        Err(String::from("Not solved yet"))
    }
}

type MemoryLen = HashMap<(usize, char, char), usize>;
type MemorySeq = HashMap<[char;2], Vec<char>>;
type Keypad = HashMap<Rc<Point2D>, char>;

fn recursion(seq: &Vec<char>, memory_len: &mut MemoryLen, memory_seq: &MemorySeq, depth: usize, max_depth: usize) ->usize{
    if depth == max_depth{
        seq.windows(2).map(|els|{
            let &v = memory_len.get(&(max_depth, els[0], els[1]))
                .expect(format!("Value for {:?} not found in memory", (max_depth, els)).as_str());
            //println!("got {} for {:?}", v, (depth, els));
            v
        }).sum()
    }else{
        let mut result: usize = 0;
        let mut prefix = 'A';
        for els in seq.windows(2){
            let mut next_seq = memory_seq.get(els).expect(format!("Value for {:?} not found in memory", (depth, els)).as_str()).clone();
            next_seq.insert(0, prefix);
            prefix = *next_seq.last().expect("Empty sequence");

            result+= if let Some(&v) =  memory_len.get(&(depth, els[0], els[1])){
                //println!("got {} for {:?}", v, (depth, els));
                v
            }else{
                //println!("requested value for {:?}", (&next_seq, depth, els));
                let v = recursion(&next_seq, memory_len, memory_seq, depth+1, max_depth);
                //println!("inserted {} for {:?}", v, (depth, els));
                memory_len.insert((depth, els[0], els[1]), v);
                v
            };
        }
        result
    }
}

fn score_sequence(seq: &Vec<char>) -> usize {
    seq.windows(2).map(|els|{
        if els[0]==els[1] {0 }else{1}
    }).sum()
}
// fn process_sequence(seq:&Vec<char>, memory: &MemorySeq) -> Vec<char> {
//     let mut result: Vec<char> = Vec::new();
//     for els in seq.windows(2){
//         if let Some(seq) = memory.get(&els){
//             result.extend(seq);
//         }
//     }
//     result
// }
fn process_pair(from: &char, to: &char, keypad: &Keypad) -> Option<Vec<Vec<char>>> {
    let from_point = keypad.iter().find_map(|(k, v)| if v == from { Some(k.clone()) } else { None })?;
    let mut result: HashMap<Rc<Point2D>, Vec<Vec<char>>> = HashMap::new();
    let mut stack = HashMap::from([(from_point, vec![Vec::new()])]);

    while result.is_empty() {
        let mut next_stack = HashMap::new();
        for (p, seqs) in stack {
            if keypad.get(&p) == Some(to) {
                result.entry(p.clone()).or_insert_with(Vec::new).extend(seqs.iter().map(|seq| {
                    let mut seq = seq.clone();
                    seq.push('A');
                    seq
                }));
            } else {
                for d in Direction::base() {
                    let next_pos = &p + &d;
                    if keypad.contains_key(&next_pos) {
                        let mut next_seqs = seqs.clone();
                        next_stack.entry(next_pos.clone())
                            .or_insert_with(Vec::new)
                            .extend(next_seqs.iter_mut().map(|seq| {
                                let mut seq = seq.clone();
                                seq.push(d.to_char());
                                seq
                            }));
                    }
                }
            }
        }
        stack = next_stack;
    }

    result.into_iter().next().map(|(_, v)| v.into_iter().collect())
    //.map(|mut x|{x.insert(0, 'A'); x})
}

fn keypad_input(output: &Vec<char>, keypad: &Keypad) ->Option<HashSet<Vec<char>>>{
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
fn numeric_keypad() -> Keypad{
    let mut keypad: Keypad = HashMap::new();
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

fn directional_keypad() -> Keypad{
    let mut keypad: Keypad = HashMap::new();
    keypad.insert(point(1,0), '^');
    keypad.insert(point(2,0), 'A');
    keypad.insert(point(0,1), '<');
    keypad.insert(point(1,1), 'v');
    keypad.insert(point(2,1), '>');
    keypad
}