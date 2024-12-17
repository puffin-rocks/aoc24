use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use crate::geometry::{Canvas, Direction, Point2D, Vector};
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
        if self.score == other.score{
            self.id.cmp(&other.id)
        }else{
            self.score.cmp(&other.score)
        }
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
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        if start.len()==1 && finish.len()==1 {

            let finish_p = finish.first().unwrap();

            let start_dir = Direction::Right;
            let start_pos = start.first().unwrap().clone();
            let start_score: usize = 0;
            let mut visited: HashMap<(Direction, Rc<Point2D>), usize> = HashMap::new();


            let mut queue: BTreeSet<ScoredPosition> = BTreeSet::new();
            let mut id: usize = 0;
            queue.insert(ScoredPosition::new(id, start_score, start_dir, start_pos, HashSet::new()));

            let mut min_score: Option<usize> = None;
            let mut threshold: usize = 1_000;
            loop{
                if let Some(p) = queue.pop_first(){
                    if p.score> threshold {
                        println!("{:?}", (&p.score));
                        threshold+=1_000;
                    }
                    visited.insert((p.direction, p.location.clone()), p.score);
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
                            let next_score = if *d == p.direction { 1} else{1001} + p.score;
                            let mut continue_path = true;
                            if let Some(s) = visited.get( &(*d, next_p.clone())){
                                if next_score>*s{
                                    continue_path = false; //arrived via loop
                                }
                            }
                            if continue_path {
                                id += 1;
                                queue.insert(ScoredPosition::new(id, next_score, *d, next_p, HashSet::new()));
                            }
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
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        if start.len() == 1 && finish.len() == 1 {
            Err(String::from("Skip Part 2"))
        } else {
            Err(String::from("Multiple start or end locations"))
        }
    }
}
