use crate::utils::Solve;
use std::collections::HashMap;
use std::env;

mod utils;
mod geometry;

mod advent01;
mod advent02;
mod advent03;
mod advent04;
mod advent05;
mod advent06;
mod advent07;
mod advent08;
mod advent09;
mod advent10;
mod advent11;
mod advent12;
mod advent13;
mod advent14;
mod advent15;
mod advent16;
mod advent17;
mod advent18;
mod advent19;

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

fn format_duration(duration: Duration) -> String{
    let total_seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
    if total_seconds >= 1.0 {
        format!("{:.1}s", total_seconds)
    } else if duration.as_millis() >= 1 {
        let total_millis = (duration.as_micros() as f64)/1_000.0;
        format!("{:.1}ms", total_millis)
    } else {
        let total_micros = (duration.as_nanos() as f64)/1_000.0;
        format!("{:.1}µs", total_micros)
    }
}

fn run(a: &mut Box<dyn Solve>, n_iterations: u32, test_mode: bool, bruteforce: bool) {
    println!("{}", "-".repeat(50));
    println!(":::Day {}:::", a.get_label().number);
    if bruteforce {
        a.apply_bruteforce();
    }

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
                    if let Ok(d) = d {
                        println!("Time taken {}: {}", part_name, format_duration(d));
                    }
                }
            }
            Err(msg) => {
                {println!("{}", msg);}
            }
        }
    }
    println!("{}", "\n");
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
    add_default_to_collection::<advent08::Advent>(&mut advents);
    add_default_to_collection::<advent09::Advent>(&mut advents);
    add_default_to_collection::<advent10::Advent>(&mut advents);
    add_default_to_collection::<advent11::Advent>(&mut advents);
    add_default_to_collection::<advent12::Advent>(&mut advents);
    add_default_to_collection::<advent13::Advent>(&mut advents);
    add_default_to_collection::<advent14::Advent>(&mut advents);
    add_default_to_collection::<advent15::Advent>(&mut advents);
    add_default_to_collection::<advent16::Advent>(&mut advents);
    add_default_to_collection::<advent17::Advent>(&mut advents);
    add_default_to_collection::<advent18::Advent>(&mut advents);
    add_default_to_collection::<advent19::Advent>(&mut advents);
    advents
}

fn main() {
    let mut n_iterations = 0;
    let mut test_mode = false;
    let mut first_day: u8 = 1;
    let mut last_day: u8 = 25;
    let mut bruteforce: bool = false;

    let args: Vec<String> = env::args().collect();
    let mut itr = args.iter().skip(1);

    while let (Some(key), Some(value)) = (itr.next(), itr.next()) {
        match key.as_str() {
            "-t" => test_mode = value.parse::<bool>().unwrap_or(test_mode),
            "-b" => bruteforce = value.parse::<bool>().unwrap_or(bruteforce),
            "-i" => n_iterations = value.parse::<u32>().unwrap_or(n_iterations),
            "-fd" => first_day = value.parse::<u8>().unwrap_or(first_day),
            "-ld" => last_day = value.parse::<u8>().unwrap_or(last_day),
            _ => {}
        }
    }
    last_day = [first_day, last_day].into_iter().max().unwrap();

    let mut solutions = collect_solutions();
    for day in first_day..=last_day {
        if let Some(a) = solutions.get_mut(&day) {
            run(a, n_iterations, test_mode, bruteforce);
        }
    }
}
