use crate::utils::Solve;
use std::collections::HashMap;
use std::env;

mod utils;
mod advent01;
mod advent02;
mod advent03;
mod advent04;
mod geometry;
mod advent05;
mod advent06;

use std::time::{Duration, Instant};

fn timeit<F>(func: F, n_iterations: u32) -> Duration
where
    F: Fn() -> (),
{
    let start = Instant::now();
    for _ in 0..n_iterations {
        func();
    }
    start.elapsed()/n_iterations
}

fn run(a: &mut Box<dyn Solve>, n_iterations: u32, test_mode: bool){
    match a.read_input(test_mode){
        Ok(_) => {a.info();
            if a.compute_part1_answer(true, test_mode) & (n_iterations>0){
                let d1 = timeit(||{ a.compute_part1_answer(false, test_mode); }, n_iterations);
                println!("Time taken Part 1: {:?}", d1);
            }
            if a.compute_part2_answer(true, test_mode) & (n_iterations>0){
                let d2 = timeit(||{ a.compute_part2_answer(false, test_mode); }, n_iterations);
                println!("Time taken Part 2: {:?}", d2);
            }}
        Err(_) => {println!("{}", "Cannot read puzzle input")}
    }

}

//the 'static lifetime is a special lifetime that signifies the entire duration of the program.
fn add_default_to_collection<T:Default+Solve+ 'static>(collection: &mut HashMap<u8, Box<dyn Solve>>) {
    let a = T::default();
    collection.insert(a.get_label().number, Box::new(a));
}

fn collect_solutions() -> HashMap<u8, Box<dyn Solve>>{
    let mut advents: HashMap<u8, Box<dyn Solve>> = HashMap::new();
    add_default_to_collection::<advent01::Advent>(&mut advents);
    add_default_to_collection::<advent02::Advent>(&mut advents);
    add_default_to_collection::<advent03::Advent>(&mut advents);
    add_default_to_collection::<advent04::Advent>(&mut advents);
    add_default_to_collection::<advent05::Advent>(&mut advents);
    add_default_to_collection::<advent06::Advent>(&mut advents);
    advents
}

fn main() {
    let mut n_iterations = 0;
    let mut test_mode = false;
    let mut first_day: u8 = 1;

    let args: Vec<String> = env::args().collect();
    let mut itr = args.iter().skip(1);

    while let (Some(key), Some(value)) = (itr.next(), itr.next()) {
        match key.as_str() {
            "-t" => test_mode = value.parse::<bool>().unwrap_or(test_mode),
            "-i" => n_iterations = value.parse::<u32>().unwrap_or(n_iterations),
            "-d" => first_day = value.parse::<u8>().unwrap_or(first_day),
            _ => {}
        }
    }

    let mut solutions = collect_solutions();
    for day in first_day..=25u8 {
        if let Some(a) = solutions.get_mut(&day) {
            run(a, n_iterations, test_mode);
        }
    }
}
