use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::{Add};
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
            label: Label::new(12),
            canvas: CanvasAsync::default(),
        }
    }
}
type APoint = Arc<Point2D>;
type AChar = Arc<char>;

impl Advent{
    fn check_enclosed(&self, set: &BTreeSet<Point2D>)-> (Option<Point2D>, Option<char>){
        let mut check_letter: Option<char> = None;
        let mut result: Option<Point2D> = None;
        for p in set{
            for d in [&Direction::base()[..], &Direction::diagonal()[..]].concat() {
                let n_point = p + &d;
                if set.contains(&n_point){
                    continue;
                }
                let neighbour = self.canvas.get_element(&n_point);
                match neighbour {
                    Some(&letter) => {
                        match check_letter{
                            Some(ch_letter)=>{
                                if ch_letter!=letter{
                                    return (None, None);
                                }
                            },
                            None =>{
                                check_letter = Some(letter);
                                result = Some(n_point);
                            }
                        }
                    }
                    None => {return (None, None);}
                }
            }
        }
        (result, check_letter)
    }

    fn compute_buckets_and_walls(&self) -> (HashMap<&APoint, usize>, HashMap<&AChar, Vec<HashSet<&APoint>>>) {
        let (vec_walls, vec_bucket_map): (Vec<HashMap<&APoint, usize>>, Vec<HashMap<&AChar, Vec<HashSet<&APoint>>>>) =
            self.canvas.elements().par_iter()
                .map(|(k, v)| {
                    let mut bucket_map: HashMap<&AChar, Vec<HashSet<&APoint>>> = HashMap::new();
                    let mut walls: HashMap<&APoint, usize> = HashMap::new();
                    let mut stack: BTreeSet<&APoint> = v.iter().collect();
                    let mut search_from: usize = 0;

                    loop {
                        let n = stack.len();
                        let mut new_stack: BTreeSet<&APoint> = BTreeSet::new();
                        for &p in stack.iter() {
                            let mut n_walls = 0;
                            let mut in_bucket = false;
                            if !bucket_map.contains_key(k) {
                                // start new bucket
                                let mut new_bucket: HashSet<&APoint> = HashSet::new();
                                new_bucket.insert(p);
                                bucket_map.entry(k).or_insert_with(Vec::new).push(new_bucket);
                                in_bucket = true;
                            }
                            // count walls
                            for d in Direction::base() {
                                let n_point = &**p + &d;
                                let neighbour = self.canvas.get_element(&n_point);
                                match neighbour {
                                    Some(&letter) => {
                                        if letter == **k {
                                            if !in_bucket {
                                                // try to add in the bucket of neighbour
                                                if let Some(bucket_list) = bucket_map.get_mut(&*k) {
                                                    for bucket in &mut bucket_list[search_from..] {
                                                        if bucket.contains(&Arc::new(n_point)) {
                                                            bucket.insert(p);
                                                            in_bucket = true;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            n_walls += 1;
                                        }
                                    },
                                    None => n_walls += 1,
                                };
                            }
                            walls.insert(p, n_walls);

                            if !in_bucket {
                                // start new bucket
                                new_stack.insert(p);
                            }
                        }

                        if new_stack.is_empty() {
                            break;
                        }
                        if n == new_stack.len() {
                            // no points were bucketed
                            let p = new_stack.pop_last().unwrap();
                            let mut new_bucket: HashSet<&APoint> = HashSet::new();
                            new_bucket.insert(p);
                            if let Some(bucket_list) = bucket_map.get(&*k) {
                                search_from = bucket_list.len();
                            }
                            bucket_map.entry(k).or_insert_with(Vec::new).push(new_bucket);
                        }
                        stack = new_stack;
                    }
                    (walls, bucket_map)
                })
                .unzip();  // This will give us two separate Vecs: one for walls, one for bucket_map

        // Merging the results
        let walls: HashMap<&APoint, usize> = vec_walls.into_iter().fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        });

        let bucket_map: HashMap<&AChar, Vec<HashSet<&APoint>>> = vec_bucket_map.into_iter().fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        });
        (walls, bucket_map)
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
        //return Err(String::from("Skip"));
        let (walls, bucket_map) = self.compute_buckets_and_walls();
        // Merging result calculation
        let mut result = 0;
        for (_, bucket_vec) in bucket_map.iter() {
            for v in bucket_vec {
                let mut per = 0;
                for &p in v.iter() {
                    if let Some(&n_walls) = walls.get(&p) {
                        per += n_walls;
                    }
                }
                result += v.len() * per;
            }
        }
        assert_display(result, Some(1930), 1486324, "Total price of fencing", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let (_, bucket_map) = self.compute_buckets_and_walls();
        // Merging result calculation
        let mut result = 0;
        for (_, bucket_vec) in bucket_map.iter() {

            for v in bucket_vec {
                let object: BTreeSet<Point2D> = v.iter().map(|&p| *p.clone()).collect();

                let mut pset: HashSet<usize> = HashSet::new();
                for po in &object {
                    let pr = get_price(&object, po, false);
                    if pr>0 {
                        pset.insert(pr);
                    }
                }

                let p = *pset.iter().max().unwrap();

                if let (Some(pt), Some(ch)) = self.check_enclosed(&object) {
                    let bucket_vec_out = bucket_map.get(&Arc::new(ch)).unwrap();
                    for vo in bucket_vec_out{
                        if vo.contains(&Arc::new(pt)){
                            result += vo.len()*p;
                            break
                        }
                    }
                }
                result += v.len()*p;
            }
        }
        assert_display(result, Some(1206), 898684, "Total price of fencing", test_mode)
    }
}

fn get_price(bucket: &BTreeSet<Point2D>, starting_point: &Point2D, verbose: bool) -> usize{
    let mut current_point = get_upper_right_corner(starting_point);
    let stop = current_point.clone();
    let mut current_direction: Option<Direction> = None;
    let mut stop_dir = None;
    let mut cnt = 0;
    loop {
        if verbose {
            println!("Corner {:?}", current_point);
        }
        let p = get_neighbour_picture(&bucket, &current_point);
        if verbose {
            println!("{:?}", p);
        }
        let cd = get_next_side(&p, current_direction);

        if verbose {
            println!("{:?}", cd);
        }
        current_point = &current_point + &cd;
        if current_point == stop{
            if verbose {
                println!("{:?}", (cd, current_direction, stop_dir));
            }
            if let Some(pd) = current_direction{
                if cd !=pd {
                    cnt+=1;
                }
                if Some(cd)==stop_dir{
                    cnt-=1;
                }
            }
            break;
        }else{
            if let Some(pd) = current_direction{
                if cd !=pd{
                    cnt+=1;
                }
            }
            else{
                cnt+=1;
                stop_dir = Some(cd);
            }
        }
        //
        if verbose {
            println!("{}", cnt);
        }
        current_direction = Some(cd);
    }
    cnt
}

type GridPoint = (f32, f32);

impl Add<&Direction> for &GridPoint {
    type Output = GridPoint;

    fn add(self, other: &Direction) -> GridPoint {
        let p = other.to_point();
        (
            self.0 + (*p.x() as f32),
            self.1 + (*p.y() as f32)
        )
    }
}

fn get_upper_right_corner(p: &Point2D) ->  GridPoint{
    (*p.x() as f32 + 0.5, *p.y() as f32 + 0.5)
}

fn get_neighbour_picture(object: &BTreeSet<Point2D>, p: &GridPoint) -> Vec<bool>{
    [Direction::DownRight, Direction::UpRight, Direction::UpLeft, Direction::DownLeft].iter()
        .map(|d|{
            let z = d.to_point();
            let np: Point2D = Point2D::new((p.0 + *z.x() as f32*0.5) as isize, (p.1 + *z.y() as f32*0.5) as isize);
            object.contains(&np)
        }).collect()
}

fn get_possible_directions(picture: &Vec<bool>) -> HashSet<Direction>{
    let n_points = picture.iter().filter(|&&x| x).count();
    if n_points == 0{
        panic!("{}", format!("Cannot have number of points {} in picture", n_points))
    }
    if n_points == 4{
        return HashSet::new();
    }
    let v = if n_points == 2{
        let picture_copy: Vec<bool> = if picture[0] == false {
            picture.iter().map(|x| !x ).collect()
        }else{
            (*picture.clone()).to_owned()
        };
        match &picture_copy[..] {
            [true, false, false, true] => [Direction::Right, Direction::Left].to_vec(),
            [true, true, false, false] => [Direction::Up, Direction::Down].to_vec(),
            [true, false, true, false] => Direction::base().to_vec(),
            _ => {Vec::new()}
        }
    }else{
        let picture_copy = if n_points == 1{
            picture.iter().map(|x| !x ).collect()
        }else{
            (*picture.clone()).to_owned()
        };
        match &picture_copy[..] {
            [true, true, true, false] => [Direction::Down, Direction::Left].to_vec(),
            [false, true, true, true] => [Direction::Down, Direction::Right].to_vec(),
            [true, false, true, true] => [Direction::Up, Direction::Right].to_vec(),
            [true, true, false, true] => [Direction::Up, Direction::Left].to_vec(),
            _ => {Vec::new()}
        }
    };
    v.iter().map(|d| *d).collect()
}

fn get_next_side(picture: &Vec<bool>, curr_direction: Option<Direction>) -> Direction{
    let mut dirs = get_possible_directions(picture);
    match curr_direction{
        Some(d) => {
            let d_mirror = d.mirror();
            if !dirs.contains(&d_mirror){
                // println!("{:?}", dirs);
                // println!("{:?}", d);
                panic!("{}", "Cannot have possible directions not including previous direction")
            }
            dirs.remove(&d_mirror);
            if dirs.len() == 1{
                *dirs.iter().next().unwrap()
            }
            else{
                match d{
                    Direction::Up =>Direction::Left,
                    Direction::Right =>Direction::Up,
                    Direction::Down =>Direction::Right,
                    Direction::Left =>Direction::Down,
                    _ => Direction::None
                }
            }
        },
        None =>{
            for d in Direction::base() {
                if dirs.contains(&d) {
                    return d;
                }
            }
            Direction::None
        }
    }
}