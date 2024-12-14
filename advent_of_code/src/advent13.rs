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
        let c: usize = 10_000_000_000_000;
        //let c: usize =  0;
        self.check_input(Some(2))?;
        let mut total_cost = 0;
        for (a, b, p) in izip!(&self.button_a, &self.button_b, &self.prize) {
            if c==0 {
                total_cost += linalg(a, b, p, c, Some(100));
            }
            else {
                total_cost += linalg(a, b, p, c, None);
            }
        }
        assert_display(total_cost, Some(875318608908), 74478585072604, "Total cost", test_mode)
    }
}

fn to_sign_val(value: isize) -> (isize, usize){
    (value.signum(), value.abs() as usize)
}

fn to_div_rem(nom: usize, den: usize) -> (usize, usize){
    (nom/den, nom%den)
}

fn calculate_press(c: usize, det: isize, mult_c: isize, nom: isize) -> Option<usize> {
    let (det_sign, det_val) = to_sign_val(det);
    let (mult_c_sign, mult_c_val) = to_sign_val(mult_c);
    let (nom_sign, nom_val) = to_sign_val(nom);

    let (nom_div, nom_rem) = to_div_rem(nom_val,det_val);
    let (c_div, c_rem) = to_div_rem(c, det_val);

    let sign1 = det_sign*mult_c_sign;
    let sign2 = det_sign*nom_sign;

    let rest = sign1*((c_rem*mult_c_val) as isize)+sign2*(nom_rem as isize);
    let tot_rem = rest%det_val as isize;
    if tot_rem!=0{
        None
    }
    else {
        let tot_div = rest/det_val as isize;
        let val = sign1 * ((c_div * mult_c_val) as isize) + sign2 * nom_div as isize + tot_div;
        if val<0{
            None
        }else{
            Some(val as usize)
        }
    }
}

fn linalg(a: &Direction, b: &Direction, p: &Direction, c: usize, cap: Option<usize>) -> usize{
    let price_a: usize = 3;
    let price_b: usize = 1;
    let pp = p.to_point();
    let pa = a.to_point();
    let pb = b.to_point();
    let ax = *pa.x();
    let ay = *pa.y();
    let bx = *pb.x();
    let by = *pb.y();
    let tx = *pp.x();
    let ty = *pp.y();
    let det = ax*by- bx*ay;

    if det.abs()==0{
        println!("{}", "Zero det");
    }

    let nom_a = by*tx-bx*ty;
    let nom_b = -ay*tx+ax*ty;
    let mult_c_a = by-bx;
    let mult_c_b = ax-ay;

    if let Some(v) = cap{
        let det_abs = det.abs() as usize;
        let det_sign = det.signum();

        let na_nom = ((by*tx-bx*ty)*det_sign) as usize;
        let nb_nom = ((-ay*tx+ax*ty)*det_sign) as usize;
        let na_div = na_nom/det_abs;
        let na_rem = na_nom%det_abs;
        let nb_div = nb_nom/det_abs;
        let nb_rem = nb_nom%det_abs;

        if na_div > v || nb_div> v  || na_rem!=0 || nb_rem!=0 {
            0
        }
        else{
            na_div*price_a+nb_div*price_b
        }
    }
    else{
        if let (Some(na_fin), Some(nb_fin)) =
            (calculate_press(c, det, mult_c_a, nom_a), calculate_press(c, det, mult_c_b, nom_b)){

            assert_eq!(na_fin * ax as usize + nb_fin * bx as usize, tx as usize+c);
            assert_eq!(na_fin * ay as usize + nb_fin * by as usize, ty as usize+c);
            na_fin*price_a+nb_fin*price_b
        }
        else{
            0
        }
    }
}

