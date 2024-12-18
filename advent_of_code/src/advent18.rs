use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use crate::geometry::{Direction, Point2D, ScoredPosition};
use crate::utils::{Solve, Label, assert_display, write_vec_to_file};



pub(crate) struct Advent {
    label: Label,
    bytes: Vec<Point2D>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(18),
            bytes: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((x,y)) = line.split_once(","){
            self.bytes.push(Point2D::new(x.parse::<isize>()?, y.parse::<isize>()?));
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Input length: {}", self.bytes.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let (width, height, n_bytes): (usize, usize, usize) = if test_mode{
            (7, 7, 12)
        }else{
            (71, 71, 1024)
        };
        let mut obstacles: HashSet<Rc<Point2D>> = HashSet::new();

        //insert boarders
        for i in 0..width{
            obstacles.insert(Rc::new(Point2D::new(i as isize,-1)));
            obstacles.insert(Rc::new(Point2D::new(i as isize, height as isize)));
        }
        for i in 0..height{
            obstacles.insert(Rc::new(Point2D::new(-1, i as isize)));
            obstacles.insert(Rc::new(Point2D::new(width as isize, i as isize)));
        }
        for i in 0..n_bytes{
            obstacles.insert(Rc::new(self.bytes[i]));
        }

        let finish_pos = Rc::new(Point2D::new((width-1) as isize, (height-1) as isize));
        let start_dir = Direction::Right;
        let start_pos = Rc::new(Point2D::new(0, 0));
        let mut visited: HashMap<Rc<Point2D>, usize> = HashMap::new();

        let mut queue: BinaryHeap<Reverse<ScoredPosition>> = BinaryHeap::new();
        let mut path: HashSet<Rc<Point2D>> = HashSet::new();
        let collect_paths = false;
        if collect_paths {
            path.insert(start_pos.clone());
        }
        let mut id: usize = 0;
        queue.push(Reverse(ScoredPosition::new(id, 0, start_dir, start_pos, path)));

        let mut min_score: Option<usize> = None;
        let mut threshold: usize = 0;

        while let Some(Reverse(p)) = queue.pop() {
            // if p.score > threshold {
            //     //println!("{:?}", &p.score);
            //     threshold += 1;
            // }

            if p.location == finish_pos {
                min_score = Some(p.score);
                break;
            }

            Direction::base().iter().for_each(|&d| {
                let next_p = &p.location + &d;

                if !obstacles.contains(&next_p) {
                    let next_score = 1 + p.score;
                    let mut continue_path = true;

                    if let Some(s) = visited.get(&next_p.clone()) {
                        if next_score >= *s {
                            continue_path = false;
                        }
                    }

                    if continue_path {
                        visited.insert(next_p.clone(), next_score);
                        let next_path = if collect_paths {
                            let mut path = p.path.clone();
                            path.insert(next_p.clone());
                            path
                        } else {
                            HashSet::new()
                        };
                        id+=1;
                        queue.push(Reverse(ScoredPosition::new(id, next_score, d, next_p, next_path)));
                    }
                }
            });
        }
        if let Some(min_score) = min_score {
            assert_display(min_score, Some(22), 348, "Shortest path", test_mode)
        }
        else{
            Err(String::from("No shortest path found"))
        }
//354 too high

    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}


//     let mut rows: Vec<Vec<char>> = Vec::new();
//     for j in 0..height{
//         let mut row: Vec<char> = Vec::new();
//         for i in 0..width{
//             let po =Rc::new(Point2D::new(i, j));
//             if obstacles.contains(&po){
//                 row.push('#');
//             }
//             else if p.path.contains(&po){
//                 row.push('O');
//             }
//             else{
//                 row.push('.');
//             }
//         }
//         rows.push(row);
//     }
//     write_vec_to_file(rows, "bytes.txt").expect(&format!("Cannot draw boxes"));