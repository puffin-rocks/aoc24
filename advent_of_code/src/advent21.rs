use std::collections::{HashMap};
use std::rc::Rc;
use itertools::Itertools;
use crate::geometry::{Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};

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

impl Advent{
    fn solve(
        &self,
        max_depth: usize,
        result_test: usize,
        result_prd: usize,
        test_mode: bool,
        part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let nkp = numeric_keypad();
        let dkp = directional_keypad();
        let mut memory_len_base: MemoryLen = HashMap::new();
        let mut memory_seq_base: MemorySeq = HashMap::new();
        let mut unresolved_values: Vec<Vec<Vec<char>>> = Vec::new();
        let mut unresolved_keys: Vec<(char, char)> = Vec::new();
        for from in dkp.values() {
            for to in dkp.values() {
                if let Some(mut seqs) = process_pair(from, to, &dkp) {
                    memory_len_base.insert((max_depth, *from, *to), seqs.first()
                        .unwrap_or_else(|| panic!("No sequences returned for pair {:?}", (from, to))).len());
                    if seqs.len() > 1 {
                        seqs.sort_unstable();
                        unresolved_values.push(seqs);
                        unresolved_keys.push((*from, *to));
                    } else {
                        memory_seq_base.insert([*from, *to], seqs.first()
                            .unwrap_or_else(|| panic!("No sequences returned for pair {:?}", (from, to)))
                            .clone());
                    }
                }
            }
        }
        let result = unresolved_values.into_iter().multi_cartesian_product().map(|combination| {
            let mut memory_len = memory_len_base.clone();
            let mut memory_seq = memory_seq_base.clone();
            let zipped = unresolved_keys.iter().zip(combination.iter());
            for (k, v) in zipped {
                memory_seq.insert([k.0, k.1], v.clone());
            }
            self.codes.iter().map(|code| {
                let shortest_seqs = keypad_input(&code, &nkp)
                    .unwrap_or_else(|| panic!("Got no recursion input for code {:?}", code));
                let min_len = shortest_seqs.iter().map(
                    |seq| {
                        let mut seq = seq.clone();
                        seq.insert(0, 'A');
                        recursion(&seq, &mut memory_len, &memory_seq, 1, max_depth)
                    }
                ).min()
                    .unwrap_or_else(|| panic!("No min length can be computed"));
                let num = code[0..code.len() - 1].iter()
                    .collect::<String>().parse::<usize>()
                    .expect(format!("Cannot parse number from code {:?}", code).as_str());
                min_len * num
            }
            ).sum::<usize>()
        }).min().unwrap();
        assert_display(result, Some(result_test), result_prd, "Sum of complexities", test_mode)
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
        self.solve(2, 126384, 217662, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(25, 154115708116294, 263617786809000, test_mode, 2)
    }
}

type MemoryLen = HashMap<(usize, char, char), usize>;
type MemorySeq = HashMap<[char;2], Vec<char>>;
type Keypad = HashMap<Rc<Point2D>, char>;

fn recursion(seq: &Vec<char>, memory_len: &mut MemoryLen, memory_seq: &MemorySeq, depth: usize, max_depth: usize) ->usize{
    if depth == max_depth{
        seq.windows(2).map(|els|{
            *memory_len.get(&(max_depth, els[0], els[1]))
                .unwrap_or_else( || panic!("Value for {:?} not found in result memory", (depth, els)))
        }).sum()
    }else{
        let mut prefix = 'A';
        seq.windows(2).map(|els|{
            let mut next_seq = memory_seq.get(els)
                .unwrap_or_else( || panic!("Value for {:?} not found in sequence memory", (depth, els)))
                .clone();
            next_seq.insert(0, prefix);
            prefix = *next_seq.last().unwrap();

            memory_len
                .get(&(depth, els[0], els[1]))
                .cloned()
                .unwrap_or_else(|| {
                    let v = recursion(&next_seq, memory_len, memory_seq, depth + 1, max_depth);
                    memory_len.insert((depth, els[0], els[1]), v);
                    v
                })
        }
        ).sum()
    }
}

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
}

fn keypad_input(output: &Vec<char>, keypad: &Keypad) -> Option<Vec<Vec<char>>>{
    let mut prev_char = 'A';
    let mut result: Vec<Vec<char>> = vec![Vec::new()];
    for curr_ch in output.iter(){
        let result_ch = process_pair(&prev_char, curr_ch, keypad)?;
        let mut next_result = Vec::with_capacity(result.len() * result_ch.len());
        for seq_next in result_ch.iter(){
            for seq_prev in result.iter(){
                let mut seq = seq_prev.clone();
                seq.extend(seq_next);
                next_result.push(seq);
            }
        }
        result = next_result;
        prev_char = *curr_ch;
    }
    Some(result)
}

fn point(x: isize, y: isize) -> Rc<Point2D>{
    Rc::new(Point2D::new(x,y))
}
fn numeric_keypad() -> Keypad{
    let mut keypad: Keypad = HashMap::new();
    let positions = [
        ((0, 0), '7'), ((1, 0), '8'), ((2, 0), '9'),
        ((0, 1), '4'), ((1, 1), '5'), ((2, 1), '6'),
        ((0, 2), '1'), ((1, 2), '2'), ((2, 2), '3'),
        ((1, 3), '0'), ((2, 3), 'A')
    ];

    for ((x, y), ch) in positions.iter() {
        keypad.insert(point(*x, *y), *ch);
    }
    keypad
}

fn directional_keypad() -> Keypad{
    let mut keypad: Keypad = HashMap::new();

    let positions = [
        ((1, 0), '^'), ((2, 0), 'A'),
        ((0, 1), '<'), ((1, 1), 'v'), ((2, 1), '>'),
    ];

    for ((x, y), ch) in positions.iter() {
        keypad.insert(point(*x, *y), *ch);
    }
    keypad
}