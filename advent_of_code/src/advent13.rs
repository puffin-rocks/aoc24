use itertools::izip;
use crate::geometry::{Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};
use rayon::prelude::*;

pub(crate) struct Advent {
    label: Label,
    button_a: Vec<Direction>,
    button_b: Vec<Direction>,
    prize: Vec<Direction>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(13),
            button_a: Vec::new(),
            button_b: Vec::new(),
            prize: Vec::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if line.is_empty() { return Ok(()); }
        if let Some((_, rhs)) = line.split_once(": "){
            if let Some((x, y)) = rhs.split_once(", "){
                if line.contains("Button A") | line.contains("Button B"){
                    if let (Some((_,x_val)), Some((_,y_val))) =
                        (x.split_once("+"), y.split_once("+")){
                        let x = x_val.parse::<usize>()?;
                        let y = y_val.parse::<usize>()?;
                        let p = Direction::ToPoint(Point2D::new(x,y));
                        if line.contains("Button A"){
                            self.button_a.push(p);
                        }
                        if line.contains("Button B"){
                            self.button_b.push(p);
                        }
                    }
                }else{
                    if let (Some((_,x_val)), Some((_,y_val))) =
                        (x.split_once("="), y.split_once("=")){
                        let x = x_val.parse::<usize>()?;
                        let y = y_val.parse::<usize>()?;
                        let p = Direction::ToPoint(Point2D::new(x,y));
                        self.prize.push(p);
                    }
                }
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of prices: {}", self.prize.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let max_press: usize = 100;
        let price_a = 3;
        let price_b = 1;
        let mut total_cost = 0;
        for (a, b, p) in izip!(&self.button_a, &self.button_b, &self.prize) {
            let result = (0..=max_press)
                .into_par_iter()
                .flat_map(|n_a| {
                    (0..=max_press).into_par_iter().map(move |n_b| (n_a, n_b))
                })
                .filter_map(|(n_a, n_b)| {
                    if &(a * n_a) + &(b * n_b) == *p {
                        let cost = price_a * n_a + price_b * n_b;
                        Some((n_a, n_b, cost))
                    } else {
                        None
                    }
                })
                .reduce(
                    || (0, 0, (price_a + price_b) * 100), // Initial minimum values
                    |acc, val| if val.2 < acc.2 { val } else { acc }, // Keep the tuple with the lowest cost
                );

            let (opt_a, opt_b, min_cost) = result;
            if opt_a>0 || opt_b>0 {
                total_cost+=min_cost;
            }
        }
        assert_display(total_cost, Some(480), 39748, "Total cost", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        let c: isize = 10_000_000_000_000;
        self.check_input(Some(2))?;
        let price_a = 3;
        let price_b = 1;
        let mut total_cost = 0;
        for (a, b, p) in izip!(&self.button_a, &self.button_b, &self.prize) {
            let pp = p.to_point();
            let pa = a.to_point();
            let pb = b.to_point();
            let max_press: isize = *vec!((c+pp.x())/pa.x(), (c+pp.x())/pb.x(), (c+pp.y())/pa.y(), (c+pp.y())/pb.y()).iter().max().unwrap();
            println!("{:?}", max_press);
            let result = (0..=max_press)
                .into_par_iter()
                .flat_map(|n_a| {
                    (0..=max_press).into_par_iter().map(move |n_b| (n_a, n_b))
                })
                .filter_map(|(n_a, n_b)| {
                    if &(a * n_a) + &(b * n_b) == *p {
                        let cost = price_a * n_a + price_b * n_b;
                        Some((n_a, n_b, cost))
                    } else {
                        None
                    }
                })
                .reduce(
                    || (0, 0, (price_a + price_b) * 100), // Initial minimum values
                    |acc, val| if val.2 < acc.2 { val } else { acc }, // Keep the tuple with the lowest cost
                );

            let (opt_a, opt_b, min_cost) = result;
            println!("{:?}", result);
            if opt_a>0 || opt_b>0 {
                total_cost+=min_cost;
            }
        }
        println!("{}", total_cost);
        Err(String::from("Not solved yet"))
    }
}
