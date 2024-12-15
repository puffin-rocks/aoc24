use std::collections::{HashMap, HashSet};
use crate::geometry::{Direction, Point2D, Vector};
use crate::utils::{Solve, Label, assert_display, write_vec_to_file};
use rayon::prelude::*;

pub(crate) struct Advent {
    label: Label,
    robots: Vec<Vector>,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(14),
            robots: Vec::new(),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label { &self.label }
    fn get_label_mut(&mut self) -> &mut Label { &mut self.label }

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((point_str, dir_str)) = line.split_once(" ") {
            if let (Some((_, point_coord)), Some((_, dir_coord))) =
                (point_str.split_once("="),
                 dir_str.split_once("="),
                ) {
                if let (Some((x_str, y_str)), Some((dx_str, dy_str))) =
                    (point_coord.split_once(","),
                     dir_coord.split_once(","))
                {
                    let anchor = Point2D::new(x_str.parse::<isize>()?, y_str.parse::<isize>()?);
                    let dir_point = Point2D::new(dx_str.parse::<isize>()?, dy_str.parse::<isize>()?);
                    self.robots.push(Vector::new(Direction::ToPoint(dir_point), anchor));
                }
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of robots: {}", self.robots.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n_seconds = 100;
        let (width, height) = if test_mode{
            (11, 7)
        }else{
            (101, 103)
        };

        let result = self.robots.par_iter().map(|r| {
            let mut sec_elapsed = 0;
            let mut r_curr = r.clone();
            while sec_elapsed < n_seconds {
                let mut p = r_curr.get_point(1);
                p.return_into_bounds(width,height);
                r_curr = Vector::new(*r_curr.direction(),p);
                sec_elapsed +=1;
            }
            r_curr
        }).fold(
            || HashMap::new(), // Initialize empty `HashMap` for each thread
            |mut acc, x| {     // Accumulate counts in each thread's local map
                *acc.entry(x.anchor().clone()).or_insert(0) += 1;
                acc
            },
        ).reduce(
            || HashMap::new(), // Initialize the global accumulator
            |mut acc, partial| { // Merge thread-local results into the global map
                for (k, v) in partial {
                    *acc.entry(k).or_insert(0 as usize) += v;
                }
                acc
            }
        );
        let (med_x, med_y) = ((width/2) as isize, (height/2) as isize);
        // let cnt_q = result.iter().fold(HashMap::new(), |mut acc, (p, count)| {
        //     let q = match (*p.x() < med_x, *p.y() < med_y) {
        //         (false, false) if *p.x() != med_x && *p.y() != med_y => 1,
        //         (false, true)  if *p.x() != med_x => 2,
        //         (true, false) if *p.y() != med_y => 3,
        //         (true, true) => 4,
        //         _ => 0
        //     };
        //     if q>0 {
        //         *acc.entry(q).or_insert(0) += count;
        //     }
        //     acc
        // });

        let cnt_q = result.iter().fold([0_usize;4], |mut acc, (p, count)| {
            count_points_quadrant(&mut acc, p, med_x, med_y, *count);
            acc
        });

        let safety_factor = cnt_q.iter().product::<usize>();
        assert_display(safety_factor, Some(12), 231019008, "Safety factor", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let n_seconds:usize = 10000; //guess
        const DIMS: (usize, usize) = (101, 103);
        let width = DIMS.0;
        let height = DIMS.1;
        let (med_x, med_y) = ((width/2) as isize, (height/2) as isize);

        fn draw(points: &HashSet<&Point2D>, width: usize, height: usize, sec_elapsed: usize) {
            let mut rows: Vec<Vec<char>> = Vec::new();
            for j in 0..height{
                let mut row: Vec<char> = Vec::new();
                for i in 0..width{
                    let p = Point2D::new(i, j);
                    if points.contains(&p){
                        row.push('\u{25A0}');
                    }
                    else{
                        row.push('.');
                    }
                }
                rows.push(row);
            }
            write_vec_to_file(rows, &format!("tree/{}.txt", sec_elapsed)).expect(&format!("Cannot draw {}", sec_elapsed));
        }

        let mut sec_elapsed: usize = 0;
        let mut robots = self.robots.clone();
        let mut measures: Vec<(usize, f32, usize)> = Vec::new();
        while sec_elapsed < n_seconds {
            robots = robots.iter().map(|r| {
                let mut p = r.get_point(1);
                p.return_into_bounds(width, height);
                Vector::new(*r.direction(), p)
            }).collect();
            sec_elapsed += 1;

            let points = robots.iter().map(|x| { x.anchor() }).collect::<HashSet<_>>();
            let mut cnt_points: [usize; DIMS.0] = [0; DIMS.0];
            for p in points.iter() {
                cnt_points[*p.x() as usize]+=1;
            }

            let mut cnt_q = [0;4];
            for r in robots.iter(){
                let p = r.anchor();
                count_points_quadrant(&mut cnt_q, p, med_x, med_y, 1);
            }
            let mut e:f32 = 0.0;
            for v in cnt_q.iter(){
                let prob = *v as f32 / self.robots.len() as f32;
                e -=prob*prob.ln();
            }
           measures.push((sec_elapsed, e, *cnt_points.iter().max().unwrap()));
            //
        }

        measures.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut suspects: HashSet<usize> = HashSet::new();
        for m in &measures[0..10]{
            suspects.insert(m.0);

        }

        //draw suspects
        let mut sec_elapsed: usize = 0;
        let mut robots = self.robots.clone();
        while sec_elapsed < n_seconds {
            robots = robots.iter().map(|r| {
                let mut p = r.get_point(1);
                p.return_into_bounds(width, height);
                Vector::new(*r.direction(), p)
            }).collect();
            sec_elapsed += 1;

            if suspects.contains(&sec_elapsed){
                let points = robots.iter().map(|x| { x.anchor() }).collect::<HashSet<_>>();
                draw(&points, width, height, sec_elapsed);
            }
        }
        
        assert_display(measures[0].0, None, 8280, "Seconds elapsed", test_mode)
    }
}

fn count_points_quadrant(cnt_q: &mut [usize; 4], p: &Point2D, med_x: isize, med_y: isize, count: usize){
    let q = match (*p.x() < med_x, *p.y() < med_y) {
        (false, false) if *p.x() != med_x && *p.y() != med_y => Some(0),
        (false, true)  if *p.x() != med_x => Some(1),
        (true, false) if *p.y() != med_y => Some(2),
        (true, true) => Some(3),
        _ => None
    };
    if let Some(q)=q {
        cnt_q[q] += count;
    }
}