use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::sync::Arc;
use rayon::prelude::*;
use crate::geometry::{CanvasAsync, Direction, Point2D, ScoredPositionAsync};
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
        visited.insert(start_pos.clone(), 0);
        let mut queue: BinaryHeap<Reverse<ScoredPositionAsync>> = BinaryHeap::new();
        queue.push(Reverse(ScoredPositionAsync::new(0, start_pos.clone())));

        while let Some(Reverse(p)) = queue.pop() {
            Direction::base().iter().for_each(|&d| {
                let next_p = &p.location + &d;
                if !obstacles.contains(&next_p) {
                    let next_score = 1 + p.score;
                    let continue_search = if let Some(n_steps) = n_steps{
                        next_score < n_steps
                    }else{
                        true
                    };
                    if visited.get(&next_p.clone()).is_none() && continue_search {
                        visited.insert(next_p.clone(), next_score);
                        queue.push(Reverse(ScoredPositionAsync::new(next_score, next_p)));
                    }
                }
            });
        };
        visited
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
        self.check_input(Some(1))?;
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        if start.len() == 1 && finish.len() == 1 {
            let finish_pos = finish.first().unwrap();
            let visited_finish = self.shortest_path(obstacles, finish_pos, None);
            let start_pos = start.first().unwrap();
            let visited_start = self.shortest_path(obstacles, start_pos, None);
            if let Some(&benchmark) = visited_start.get(finish_pos) {
                let mut n_threshold: usize = 0;
                for p in obstacles.iter() {
                    for sd in Direction::base() {
                        let cheat_entry = p + &sd;
                        if let Some(&s_dist) = visited_start.get(&cheat_entry) {
                            for fd in sd.complimentary_base() {
                                let cheat_exit = p + &fd;
                                if let Some(&f_dist) = visited_finish.get(&cheat_exit) {
                                    if benchmark.saturating_sub(f_dist + s_dist + 2) >99 {
                                        n_threshold += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                assert_display(n_threshold, Some(0), 1369, "Number of cheats better than 99", test_mode)
            } else {
                Err(String::from("Finish position not reached"))
            }
        } else {
            Err(String::from("Multiple start or end locations"))
        }
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        let mut free: BTreeSet<Arc<Point2D>> = self.canvas.try_locate_element(&'.')?.iter().cloned().collect();
        if start.len() == 1 && finish.len() == 1 {
            let finish_pos = finish.first().unwrap();
            let visited_finish = self.shortest_path(obstacles, finish_pos, None);
            let start_pos = start.first().unwrap();
            let visited_start = self.shortest_path(obstacles, start_pos, None);

            let (&width, &height) = self.canvas.shape();
            let mut boarders: BTreeSet<Arc<Point2D>> = BTreeSet::new();
            //insert boarders
            for i in 0..width{
                boarders.insert(Arc::new(Point2D::new(i as isize, -1)));
                boarders.insert(Arc::new(Point2D::new(i as isize, height as isize)));
            }
            for i in 0..height{
                boarders.insert(Arc::new(Point2D::new(-1, i as isize)));
                boarders.insert(Arc::new(Point2D::new(width as isize, i as isize)));
            }

            if let Some(&benchmark) = visited_start.get(finish_pos) {
                free.extend(start.clone());
                free.extend(finish.clone());

                let result: usize = free.iter().collect::<Vec<_>>().par_iter().map(|&cheat_entry|{
                    let mut cheats: HashMap<(Arc<Point2D>, Arc<Point2D>), Vec<usize>> = HashMap::new();
                    if let Some(&s_dist) = visited_start.get(cheat_entry) {
                        let reachable = self.shortest_path(&boarders, cheat_entry, Some(20));
                        for (p, &length) in reachable.iter() {
                            for fd in Direction::base() {
                                let cheat_exit = p + &fd;
                                if let Some(&f_dist) = visited_finish.get(&cheat_exit) {
                                    let gain = benchmark.saturating_sub(f_dist + s_dist + length + 1);
                                    if gain>99 {
                                        cheats.entry((cheat_entry.clone(), cheat_exit.clone())).or_insert_with(Vec::new).push(gain);
                                    }
                                }
                            }
                        }
                    };
                    cheats.iter()
                        .map(|e| (e.0.clone(), *e.1.iter().max().unwrap_or(&0))).count()
                }).sum();
                println!("{:?}", result);


                Err(String::from("Not solved yet"))
            } else {
                Err(String::from("Finish position not reached"))
            }

        } else {
            Err(String::from("Multiple start or end locations"))
        }
    }
}
