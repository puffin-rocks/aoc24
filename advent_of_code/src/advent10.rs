use std::collections::{HashMap, HashSet};
use crate::geometry::{Canvas, Direction, Vector};
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

impl Advent {
    fn solve(&self,
             distinct: bool,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8
    ) -> Result<String, String>{
        self.check_input(Some(part))?;
        let header = if part == 1 {"Sum of scores"} else {"Sum of ratings"};

        let mut score_sum = 0;

        if let Ok(trailheads) = self.canvas.try_locate_element(&'0'){
            let mut stack: HashSet<Vec<Vector>> = trailheads
                .iter()
                .map(|p| vec![Vector::new(Direction::None, **p)])
                .collect();

            let mut next_dirs: HashMap<Direction, HashSet<Direction>> = Direction::base()
                .into_iter()
                .map(|d| {
                    let mut s: HashSet<Direction> = Direction::base().into_iter().collect();
                    s.remove(&d.mirror());
                    (d, s)
                })
                .collect();
            next_dirs.insert(Direction::None, Direction::base().into_iter().collect());

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
                                        if distinct{
                                            let mut v_next = ve.clone();
                                            v_next.push(Vector::new(d, p));
                                            next_stack.insert(v_next);
                                        }
                                        else {
                                            if let Some(f) = ve.first() {
                                                if i == 9 {
                                                    next_stack.insert(vec![f.clone(), Vector::new(Direction::None, p)]);
                                                } else {
                                                    next_stack.insert(vec![f.clone(), Vector::new(d, p)]);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                stack = next_stack;
            }
            score_sum = stack.len();
        };

        assert_display(score_sum, Some(result_test), result_prd, header, test_mode)
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
        self.solve(false, 36, 822, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(true, 81, 1801, test_mode, 2)
    }
}