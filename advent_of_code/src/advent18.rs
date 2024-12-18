use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use crate::geometry::{Direction, Point2D, ScoredPosition};
use crate::utils::{Solve, Label, assert_display};



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

impl Advent {
    fn shortest_path(&self,
                     borders: &HashSet<Rc<Point2D>>,
                     n_bytes: usize,
                     start_pos: &Rc<Point2D>,
                     finish_pos: &Rc<Point2D>) -> Option<usize>{
        let mut obstacles: HashSet<Rc<Point2D>> = borders.iter().cloned().collect();
        for i in 0..n_bytes {
            obstacles.insert(Rc::new(self.bytes[i]));
        }

        let mut visited: HashMap<Rc<Point2D>, usize> = HashMap::new();
        let mut queue: BinaryHeap<Reverse<ScoredPosition>> = BinaryHeap::new();
        queue.push(Reverse(ScoredPosition::simple(0, start_pos.clone())));
        let mut min_score: Option<usize> = None;

        while let Some(Reverse(p)) = queue.pop() {

            if p.location == *finish_pos {
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
                        queue.push(Reverse(ScoredPosition::simple(next_score, next_p)));
                    }
                }
            });
        }
        min_score
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
        let (n_bytes,
            obstacles,
            start_pos,
            finish_pos) = initialize(test_mode);
        let min_score = self.shortest_path(&obstacles, n_bytes, &start_pos, &finish_pos);
        if let Some(min_score) = min_score {
            assert_display(min_score, Some(22), 348, "Shortest path", test_mode)
        }
        else{
            Err(String::from("No shortest path found"))
        }
    }

    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let (n_bytes,
            obstacles,
            start_pos,
            finish_pos) = initialize(test_mode);

        let mut n_bytes_low = n_bytes;
        let mut n_bytes_high = self.bytes.len();

        let mut r_low = self.shortest_path(&obstacles, n_bytes_low, &start_pos, &finish_pos);
        let mut r_high = self.shortest_path(&obstacles, n_bytes_high, &start_pos, &finish_pos);

        while n_bytes_high - n_bytes_low > 1 {
            let n_bytes_mid = (n_bytes_low + n_bytes_high) / 2;
            let r_mid = self.shortest_path(&obstacles, n_bytes_mid, &start_pos, &finish_pos);

            match (r_low.is_some(), r_mid.is_some(), r_high.is_some()) {
                (true, true, false) => {
                    n_bytes_low = n_bytes_mid;
                    r_low = r_mid;
                }
                (true, false, false) => {
                    n_bytes_high = n_bytes_mid;
                    r_high = r_mid;
                }
                _ => unreachable!(),
            }
        }

        let result = self.bytes[n_bytes_low];
        assert_display(result, Some(Point2D::new(6,1)), Point2D::new(54,44), "Blocking byte", test_mode)
    }
}

fn initialize(test_mode: bool) -> (usize, HashSet<Rc<Point2D>>, Rc<Point2D>, Rc<Point2D>) {
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
    let finish_pos = Rc::new(Point2D::new((width - 1) as isize, (height - 1) as isize));
    let start_pos = Rc::new(Point2D::new(0, 0));
    (n_bytes, obstacles, start_pos, finish_pos)
}