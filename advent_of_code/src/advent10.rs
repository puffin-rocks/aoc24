use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use crate::geometry::{Canvas, Direction, Point2D, Vector};
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(10),
            canvas: Canvas::default(),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut next_el: BTreeMap<char,char> = BTreeMap::new();
        for i in 0u8..9u8{
            next_el.insert((b'0' + i) as char, (b'0' + i+1) as char);
        }
        let mut score_sum = 0;
        if let Ok(trailheads) = self.canvas.try_locate_element(&'0'){
            let mut stack: HashSet<Vec<Vector>> = HashSet::new();
            for p in trailheads.iter(){
                stack.insert(vec!(Vector::new(Direction::None, **p)));
            }
            let mut next_dirs: HashMap<Direction, HashSet<Direction>> = HashMap::new();
            next_dirs.insert(Direction::None, Direction::base().into_iter().collect());

            for d in Direction::base(){
                let mut s: HashSet<Direction> = Direction::base().into_iter().collect();
                s.remove(&d.mirror());
                next_dirs.insert(d, s);
            }

            for i in 1..=9 {
                let ch_expected = (b'0' + i) as char;
                let mut next_stack: HashSet<Vec<Vector>> = HashSet::new();

                for ve in stack.iter() {
                    if let Some (v) = ve.last() {
                        if let Some(dirs) = next_dirs.get(&v.direction()) {
                            for &d in dirs.iter() {
                                let p = v.anchor().get_point(d, 1);
                                if let Some(&el) = self.canvas.get_element(&p) {
                                    if el == ch_expected {
                                        //println!("{:?}", (ch_expected, p, d));
                                        let mut v_next = ve.clone();
                                        v_next.push(Vector::new(d, p));
                                        next_stack.insert(v_next);
                                    }
                                }
                            }
                        }
                    }
                }
                stack = next_stack;
            }
            let mut stack2: HashSet<Vec<&Point2D>> = HashSet::new();
            for s in stack.iter(){
                if let (Some(f), Some(l))=(s.first(), s.last()){
                    stack2.insert(vec!(f.anchor(),l.anchor()));
                }
            }
            score_sum = stack2.len();
        };
        assert_display(score_sum, Some(36), 822, "Score sum", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    // }
}