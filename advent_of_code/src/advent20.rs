use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use rayon::prelude::*;
use crate::geometry::{CanvasAsync, Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    canvas: CanvasAsync,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(20),
            canvas: CanvasAsync::default(),
        }
    }
}

impl Advent {
    fn shortest_path(&self, obstacles: &BTreeSet<Arc<Point2D>>, start_pos: &Arc<Point2D>, n_steps: Option<usize>) -> HashMap<Arc<Point2D>, usize>
    {
        let mut visited: HashMap<Arc<Point2D>, usize> = HashMap::new();
        let mut step: usize = 0;
        visited.insert(start_pos.clone(), step);
        let mut stack: HashSet<Arc<Point2D>> = HashSet::new();
        stack.insert(start_pos.clone());
        loop{
            step+=1;

            let mut next_stack: HashSet<Arc<Point2D>> = HashSet::new();
            for p in stack.iter(){
                Direction::base().iter().for_each(|&d| {
                    let next_p = p + &d;
                    if !obstacles.contains(&next_p) {
                        let continue_search = if let Some(n_steps) = n_steps {
                            step < n_steps
                        } else {
                            true
                        };
                        if visited.get(&next_p.clone()).is_none() && continue_search {
                            visited.insert(next_p.clone(), step);
                            next_stack.insert(next_p);
                        }
                    }
                })
            }
            if next_stack.is_empty(){
                break;
            }
            stack = next_stack;
        }
        visited
    }

    fn solve(
        &self,
        max_cheats: usize,
        result_test: usize,
        result_prd: usize,
        test_mode: bool,
        part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let threshold = if test_mode{
            19
        }else{
            99
        };
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        if start.len() == 1 && finish.len() == 1 {
            let finish_pos = finish.first().unwrap();
            let visited_finish = self.shortest_path(obstacles, finish_pos, None);
            let start_pos = start.first().unwrap();
            let visited_start = self.shortest_path(obstacles, start_pos, None);

            if let Some(&benchmark) = visited_start.get(finish_pos) {
                let result: usize = visited_start.iter().collect::<Vec<_>>().par_iter().map(|(cheat_entry, &s_dist)|{
                    let mut cheats: HashMap<(Arc<Point2D>, Arc<Point2D>), usize> = HashMap::new();
                    let reachable = self.shortest_path(&BTreeSet::new(), &cheat_entry, Some(max_cheats));
                    for (p, &length) in reachable.iter() {
                        for fd in Direction::base() {
                            let cheat_exit = p + &fd;
                            if let Some(&f_dist) = visited_finish.get(&cheat_exit) {
                                let gain = benchmark.saturating_sub(f_dist + s_dist + length + 1);
                                let cheat_key = ((*cheat_entry).clone(), cheat_exit.clone());
                                if gain>threshold {
                                    let insert_cheat = if let Some(max_gain) = cheats.get(&cheat_key){
                                        gain>*max_gain
                                    }else{
                                        true
                                    };
                                    if insert_cheat{
                                        cheats.insert(cheat_key, gain);
                                    }
                                }
                            }
                        }
                    }
                    cheats.len()
                }).sum();
                assert_display(result, Some(result_test), result_prd, format!("Number of cheats better than {}", threshold).as_str(), test_mode)
            } else {
                Err(String::from("Finish position not reached"))
            }
        } else {
            Err(String::from("Multiple start or end locations"))
        }
    }

}


impl Solve for Advent {
    fn get_label(&self) -> &Label { &self.label }
    fn get_label_mut(&mut self) -> &mut Label { &mut self.label }

    fn get_canvas_async_mut(&mut self) -> Option<&mut CanvasAsync> {
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String> {
        self.solve(2,  5, 1369, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(20,  1449, 979012, test_mode, 2)
    }
}
