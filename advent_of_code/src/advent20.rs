use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::sync::Arc;
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
            canvas: CanvasAsync::default()
        }
    }
}

impl Advent {
    fn shortest_path(&self, obstacles: &BTreeSet<Arc<Point2D>>, start_pos: &Arc<Point2D>) -> HashMap<Arc<Point2D>, usize>
    {
        let mut visited: HashMap<Arc<Point2D>, usize> = HashMap::new();
        let mut queue: BinaryHeap<Reverse<ScoredPositionAsync>> = BinaryHeap::new();
        queue.push(Reverse(ScoredPositionAsync::new(0, start_pos.clone())));

        while let Some(Reverse(p)) = queue.pop() {
            Direction::base().iter().for_each(|&d| {
                let next_p = &p.location + &d;
                if !obstacles.contains(&next_p) {
                    if visited.get(&next_p.clone()).is_none() {
                        let next_score = 1 + p.score;
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
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_async_mut(&mut self) -> Option<&mut CanvasAsync> {
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
         self.check_input(Some(1))?;
         let start = self.canvas.try_locate_element(&'S')?;
         let finish = self.canvas.try_locate_element(&'E')?;
         let obstacles = self.canvas.try_locate_element(&'#')?;
         if start.len()==1 && finish.len()==1 {
             let finish_pos = finish.first().unwrap();
             let visited_finish = self.shortest_path(obstacles, finish_pos);
             let start_pos = start.first().unwrap();
             let visited_start = self.shortest_path(obstacles, start_pos);
             if let Some(&benchmark) = visited_start.get(finish_pos){
                 let mut n_threshold: usize = 0;
                 for p in obstacles.iter(){
                     for sd in Direction::base(){
                         let cheat_entry = p + &sd;
                         if let Some(&s_dist) = visited_start.get(&cheat_entry){
                             for fd in sd.complimentary_base(){
                                 let cheat_exit = p + &fd;
                                 if let Some(&f_dist) = visited_finish.get(&cheat_exit){
                                     if benchmark.saturating_sub(f_dist + s_dist + 2) >=100{
                                         n_threshold+=1;
                                     }
                                 }
                             }
                         }
                     }
                 }
                 assert_display(n_threshold, Some(0), 1369, "Number of cheats better than 99", test_mode)
             }else{
                 Err(String::from("Finish position not reached"))
             }
         }else{
            Err(String::from("Multiple start or end locations"))
         }
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}