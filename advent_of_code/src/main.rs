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
mod advent07;

use std::time::{Duration, Instant};

fn timeit<F>(mut func: F, n_iterations: u32) -> Result<Duration, String>
where
    F: FnMut() -> Result<String, String>,
{
    let start = Instant::now();
    for _ in 0..n_iterations {
        func()?;
    }
    Ok(start.elapsed()/n_iterations)
}

fn run(a: &mut Box<dyn Solve>, n_iterations: u32, test_mode: bool){
    match a.read_input(test_mode){
        Ok(_) => {
            if let Err(msg) = a.info() {println!("{}", msg)};
            match a.compute_part1_answer(test_mode) {
                Ok(result) => {
                    println!("{}", result);
                    if n_iterations > 0 {
                        let d = timeit(|| { a.compute_part1_answer(test_mode) }, n_iterations);
                        println!("Time taken Part 1: {:?}", d);
                    }
                }
                Err(msg) => {println!("{}", msg);}
            }
            match a.compute_part2_answer(test_mode) {
                Ok(result) => {
                    println!("{}", result);
                    if n_iterations > 0 {
                        let d = timeit(|| { a.compute_part2_answer(test_mode) }, n_iterations);
                        println!("Time taken Part 2: {:?}", d);
                    }
                }
                Err(msg) => {println!("{}", msg);}
            }
        }
        Err(_) => {println!("{}", "Cannot read puzzle input")}
    }
}

fn run_gpt(a: &mut Box<dyn Solve>, n_iterations: u32, test_mode: bool) {
    if let Err(_) = a.read_input(test_mode) {
        println!("Cannot read puzzle input");
        return;
    }

    if let Err(msg) = a.info() {println!("{}", msg)};

    // Define the method closures with explicit types
    let methods: [(&str, Box<dyn Fn(&mut Box<dyn Solve>, bool) -> Result<String, String>>); 2] = [
        (
            "Part 1",
            Box::new(|a: &mut Box<dyn Solve>, test_mode| a.compute_part1_answer(test_mode))
        ),
        (
            "Part 2",
            Box::new(|a: &mut Box<dyn Solve>, test_mode| a.compute_part2_answer(test_mode))
        ),
    ];

    // Iterate over the methods
    for (part_name, method) in methods.iter() {
        match method(a, test_mode) {
            Ok(result) => {
                println!("{}", result);
                if n_iterations > 0 {
                    let d = timeit(|| { method(a, test_mode) }, n_iterations);
                    println!("Time taken {}: {:?}", part_name, d);
                }
            }
            Err(msg) => {
                {println!("{}", msg);}
            }
        }
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
    add_default_to_collection::<advent07::Advent>(&mut advents);
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
            run_gpt(a, n_iterations, test_mode);
        }
    }
}
