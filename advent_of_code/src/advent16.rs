use std::cmp::Ordering;
use std::collections::BTreeSet;
use crate::geometry::{CanvasAsync, Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
struct ScoredPosition{
    score: usize,
    direction: Direction,
    location: Arc<Point2D>
}

impl ScoredPosition{
    fn new(score: usize, direction: Direction, location: Arc<Point2D>)->Self{
        Self{
            score,
            direction,
            location
        }
    }
}

impl Eq for ScoredPosition {}

impl PartialOrd<Self> for ScoredPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredPosition{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score == other.score{
            if self.direction == other.direction{
                self.location.cmp(&other.location)
            }else{
                self.direction.to_point().cmp(&other.direction.to_point())
            }
        }else{
            self.score.cmp(&other.score)
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    canvas: CanvasAsync
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(16),
            canvas: CanvasAsync::default()
        }
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

            // let finish_p = finish.first().unwrap();
            // let queue = Arc::new(Mutex::new(BTreeSet::new()));
            // queue.lock().unwrap().insert(ScoredPosition::new(0, Direction::Right, start.first().unwrap().clone()));
            // let mut min_score: Option<usize> = None;
            // loop {
            //     let current_queue: Vec<ScoredPosition> = {
            //         let mut queue_guard = queue.lock().unwrap();
            //         if queue_guard.is_empty(){
            //             break;
            //         }
            //         let head = queue_guard.pop_first().unwrap();
            //         if head.location == *finish_p{
            //             min_score = Some(head.score);
            //             break;
            //         }
            //         vec![head]
            //     };
            //
            //     current_queue
            //         .iter()
            //         .for_each(|p| {
            //
            //             let next_dirs = match p.direction {
            //                 Direction::Up | Direction::Down => [p.direction, Direction::Left, Direction::Right],
            //                 Direction::Right | Direction::Left => [p.direction, Direction::Up, Direction::Down],
            //                 _ => unreachable!(),
            //             };
            //
            //             next_dirs.par_iter().for_each(|d| {
            //                 let next_p = &p.location + d;
            //                 if !obstacles.contains(&next_p) {
            //                     let score = if *d == p.direction { 1 } else { 1001 };
            //                     let new_position = ScoredPosition::new(score + p.score, *d, next_p);
            //
            //                     // Lock the queue to safely insert the new position
            //                     let mut queue_guard = queue.lock().unwrap();
            //                     queue_guard.insert(new_position);
            //                 }
            //             });
            //         });
            // }
            // if let Some(min_score) = min_score {
            //     assert_display(min_score, Some(11048), 85480, "Lowest score", test_mode)
            // }
            // else{
            //     Err(String::from("No lowest score foung"))
            // }

            let finish_p = finish.first().unwrap();
            let mut queue: BTreeSet<ScoredPosition> = BTreeSet::new();
            queue.insert(ScoredPosition::new(0, Direction::Right, start.first().unwrap().clone()));
            let mut min_score: Option<usize> = None;
            loop{
                if let Some(p) = queue.pop_first(){
                    if p.location == *finish_p{
                        min_score = Some(p.score);
                        break;
                    }
                    let next_dirs = match p.direction{
                        Direction::Up|Direction::Down => [p.direction, Direction::Left, Direction::Right],
                        Direction::Right|Direction::Left => [p.direction, Direction::Up, Direction::Down],
                        _ => unreachable!()
                    };
                    for d in next_dirs.iter(){
                        let next_p = &p.location + d;
                        if !obstacles.contains(&next_p){
                            let score = if *d == p.direction { 1} else{1001};
                            queue.insert(ScoredPosition::new(score+p.score, *d, next_p));
                        }
                    }
                }
            };
            if let Some(min_score) = min_score {
                assert_display(min_score, Some(11048), 85480, "Lowest score", test_mode)
            }
            else{
                Err(String::from("No lowest score foung"))
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
