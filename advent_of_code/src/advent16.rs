use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use rayon::prelude::*;
use crate::geometry::{Canvas, Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};


#[derive(Debug, Clone, PartialEq)]
struct ScoredPosition{
    id: usize,
    score: usize,
    direction: Direction,
    location: Rc<Point2D>,
    path: HashSet<Rc<Point2D>>
}

impl ScoredPosition{
    fn new(id:usize, score: usize, direction: Direction, location: Rc<Point2D>, path: HashSet<Rc<Point2D>>)->Self{
        Self{
            id,
            score,
            direction,
            location,
            path
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
        self.score.cmp(&other.score).then_with(|| self.id.cmp(&other.id))
    }
}

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(16),
            canvas: Canvas::default()
        }
    }
}

impl Advent {
    fn solve(&self,
             collect_paths: bool,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let header = if part == 1 {"Lowest score"} else {"Number of tiles"};
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        if start.len()==1 && finish.len()==1 {
            let finish_p = finish.first().unwrap();
            let start_dir = Direction::Right;
            let start_pos = start.first().unwrap().clone();
            let mut visited: HashMap<(Direction, Rc<Point2D>), usize> = HashMap::new();

            let mut queue: BinaryHeap<Reverse<ScoredPosition>> = BinaryHeap::new();
            let mut path: HashSet<Rc<Point2D>> = HashSet::new();
            if collect_paths {
                path.insert(start_pos.clone());
            }
            queue.push(Reverse(ScoredPosition::new(0, 0, start_dir, start_pos, path)));

            let mut min_score: Option<usize> = None;
            let mut threshold: usize = 20_000;
            let mut points: HashSet<Rc<Point2D>> = HashSet::new();

            while let Some(Reverse(p)) = queue.pop() {
                if p.score > threshold {
                    println!("{:?}", &p.score);
                    threshold += 20_000;
                }

                if let Some(s) = min_score {
                    if p.score > s {
                        break;
                    }
                    if p.score == s && p.location == *finish_p {
                        points.extend(p.path.iter().cloned());
                    }
                }

                visited.insert((p.direction, p.location.clone()), p.score);

                if p.location == *finish_p {
                    min_score = Some(p.score);
                    if collect_paths {
                        points.extend(p.path.iter().cloned());
                    } else {
                        break;
                    }
                }

                let next_dirs = match p.direction {
                    Direction::Up | Direction::Down => [p.direction, Direction::Left, Direction::Right],
                    Direction::Right | Direction::Left => [p.direction, Direction::Up, Direction::Down],
                    _ => unreachable!(),
                };

                next_dirs.iter().for_each(|&d| {
                    let next_p = &p.location + &d;

                    if !obstacles.contains(&next_p) {
                        let next_score = if d == p.direction { 1 } else { 1001 } + p.score;
                        let mut continue_path = true;

                        if let Some(s) = visited.get(&(d, next_p.clone())) {
                            if next_score > *s {
                                continue_path = false;
                            }
                        }

                        if continue_path {
                            let next_path = if collect_paths {
                                let mut path = p.path.clone();
                                path.insert(next_p.clone());
                                path
                            } else {
                                HashSet::new()
                            };
                            queue.push(Reverse(ScoredPosition::new(0, next_score, d, next_p, next_path)));
                        }
                    }
                });
            }
            if let Some(min_score) = min_score {
                let result = if collect_paths {
                    points.len()
                }else{
                    min_score
                };
                assert_display(result, Some(result_test), result_prd, header, test_mode)
            }
            else{
                Err(String::from("No lowest score foung"))
            }
        }else{
            Err(String::from("Multiple start or end locations"))
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas> {
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        self.solve(false, 11048, 85480, test_mode, 1)

    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        self.solve(true, 64, 518, test_mode, 2)
    }
}
