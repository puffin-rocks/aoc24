use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use crate::geometry::{CanvasAsync, Direction, Point2D, Vector};
use crate::utils::{Solve, Label, assert_display};

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
struct PositionPath{
    position: Vector,
    path: HashSet<Arc<Point2D>>
}

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
        return  Err(String::from("Skip Part 1"));
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
            let mut threshold: usize = 1_000;
            loop{
                if let Some(p) = queue.pop_first(){
                    if p.score> threshold {
                        println!("{:?}", (&p.score));
                        threshold+=1_000;
                    }
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
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let start = self.canvas.try_locate_element(&'S')?;
        let finish = self.canvas.try_locate_element(&'E')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        if start.len() == 1 && finish.len() == 1 {

            //Marco B
            // let elements = self.canvas.elements();
            // let mut map0: HashMap<(isize, isize), char> = HashMap::new();
            // for (ch, points) in elements.iter(){
            //     let chout = if (**ch == '#'){
            //         '#'
            //     }else{
            //         '.'
            //     };
            //     for p in points{
            //         let tmp = **p;
            //         map0.insert((*tmp.x(), *tmp.y()), chout);
            //     }
            // }
            // let f_p = **finish.first().unwrap();
            // let s_p = **start.first().unwrap();
            //
            // let (_, paths) = traverse(&map0, &(*s_p.x(), *s_p.y()), &(*f_p.x(), *f_p.y()));
            // println!("{}", paths.into_iter().collect::<HashSet<(isize, isize)>>().len());


            let finish_p = finish.first().unwrap();
            let start_p = start.first().unwrap();

            let v = Vector::new(Direction::Left, (**start_p).clone());
            let mut path: HashSet<Arc<Point2D>> = HashSet::new();
            path.insert(start_p.clone());

            let mut visited: HashMap<Vector, usize> = HashMap::new();
            visited.insert(v.clone(), 0);


            type PathsToPoint = HashMap<Arc<Point2D>, Vec<PositionPath>>;
            let mut ptp: PathsToPoint = HashMap::new();
            let pp = PositionPath{ position: v, path };
            ptp.insert(start_p.clone(), vec![pp]);

            let mut queue: BTreeMap<usize, PathsToPoint> = BTreeMap::new();
            queue.insert(0, ptp);
            let mut min_score: Option<usize> = None;
            let mut threshold: usize = 1_000;

            loop {
                if let Some((p_score, map)) = queue.pop_first() {
                    if p_score > threshold{
                        println!("{:?}", &p_score);
                        threshold += 1_000;
                    }
                    if let Some(paths_to_finish) = map.get(finish_p){
                        min_score = Some(p_score);
                        println!("S {:?}", p_score);
                        let mut points: HashSet<Arc<Point2D>> = HashSet::new();
                        for p in paths_to_finish.iter(){
                            points.extend(p.path.iter().cloned());
                        }
                        println!("N {:?}", points.len());
                        break;
                    }
                    for (_, paths) in map.iter() {
                        for p in paths.iter(){
                            let next_dirs = match p.position.direction() {
                                Direction::Up | Direction::Down => [*p.position.direction(), Direction::Left, Direction::Right],
                                Direction::Right | Direction::Left => [*p.position.direction(), Direction::Up, Direction::Down],
                                _ => unreachable!()
                            };
                            for d in next_dirs.iter() {
                                let next_p = Arc::new(p.position.anchor() + d);
                                let next_v = Vector::new(d.clone(), *next_p);

                                if !obstacles.contains(&next_p) {
                                    let score: usize = if d == p.position.direction() { 1 } else { 1001 };
                                    let next_score = p_score + score;
                                    let skip = match visited.get(&next_v) {
                                        Some(s) => *s < next_score,
                                        None => false
                                    };
                                    if !skip {
                                        let mut next_path: HashSet<Arc<Point2D>> = p.path.iter().cloned().collect();
                                        next_path.insert(next_p.clone());
                                        let next_pp = PositionPath{ position: next_v.clone(), path: next_path};
                                        visited.insert(next_v.clone(), next_score);

                                        if let Some(ptp) = queue.get_mut(&next_score) {
                                            if let Some(other_path) = ptp.get_mut(&next_p) {
                                                other_path.push(next_pp.clone());
                                            } else {
                                                ptp.insert(next_p.clone(), vec![next_pp]);
                                            }
                                        } else {
                                            let mut ptp: PathsToPoint = HashMap::new();
                                            ptp.insert(next_p.clone(), vec![next_pp]);
                                            queue.insert(next_score, ptp);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else{
                    break;
                }
            };
            //518
            Err(String::from("Not solved yet"))
            // if let Some(min_score) = min_score {
            //     assert_display(min_score, Some(11048), 85480, "Lowest score", test_mode)
            // } else {
            //     Err(String::from("No lowest score foung"))
            // }
        } else {
            Err(String::from("Multiple start or end locations"))
        }
    }
}

pub struct Loc {
    pos: (isize, isize),
    d: char,
    points: usize,
    path: Vec<(isize, isize)>,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn traverse(
    map: &HashMap<(isize, isize), char>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> (HashMap<((isize, isize), char), usize>, Vec<(isize, isize)>) {
    let mut visited: HashMap<((isize, isize), char), usize> = HashMap::new();
    let mut min_points = usize::MAX;
    let mut paths = Vec::new();

    let mut queue: VecDeque<Loc> = VecDeque::from([Loc {
        pos: *start,
        d: '>',
        points: 0,
        path: vec![*start],
    }]);

    while let Some(loc) = queue.pop_front() {
        if visited
            .get(&(loc.pos, loc.d))
            .map_or(false, |&p| p < loc.points)
        {
            continue;
        }

        if loc.pos == *end {
            if loc.points < min_points {
                paths = loc.path.clone(); // Reset paths
                min_points = loc.points;
            } else if loc.points == min_points {
                paths.extend(loc.path.clone());
            }
        }

        visited.insert((loc.pos, loc.d), loc.points);
        for &dir in DIRECTIONS.iter() {
            let new_pos = (loc.pos.0 + dir.0, loc.pos.1 + dir.1);
            match map[&new_pos] {
                '#' => continue,
                '.' => {
                    let new_d = match dir {
                        (0, 1) => '>',
                        (1, 0) => 'v',
                        (0, -1) => '<',
                        (-1, 0) => '^',
                        _ => panic!("Invalid direction"),
                    };

                    let mut points = loc.points;
                    if loc.d == new_d {
                        points += 1;
                    } else {
                        points += 1001;
                    }

                    if points <= *visited.get(&(new_pos, new_d)).unwrap_or(&usize::MAX) {
                        let mut path = loc.path.clone();
                        path.push(new_pos);

                        queue.push_back(Loc {
                            pos: new_pos,
                            d: new_d,
                            points,
                            path,
                        });
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
    }

    (visited, paths)
}