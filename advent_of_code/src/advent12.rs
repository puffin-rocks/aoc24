use std::collections::{BTreeSet, HashMap, HashSet};
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

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_async_mut(&mut self) -> Option<&mut CanvasAsync> {
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        // for (k, v) in self.canvas.elements() {
        //     println!("{:?}", (k, v.len()));
        // }
        // let mut t: HashMap<&Rc<char>,usize> = HashMap::new();
        // for (k, v) in self.canvas.elements(){
        //     println!("{:?}", (k, v.len()));
        //     for p in self.canvas.iter() {
        //         if (*v).contains(&Rc::new(p)) {
        //             println!("{:?}", self.canvas.get_element(&p));
        //         }
        //     }
        //     t.insert(k, v.len());
        // }
        // println!("{:?}", t);

        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        type APoint = Arc<Point2D>;
        type AChar = Arc<char>;
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
        //5.7 sec
        assert_display(result, Some(1930), 1486324, "Total price of fencing", test_mode)
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}