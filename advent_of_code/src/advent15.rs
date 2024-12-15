use std::collections::HashSet;
use std::rc::Rc;
use crate::geometry::{Canvas, Direction, Point2D};
use crate::utils::{Solve, Label, write_vec_to_file, assert_display};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
    read_canvas: bool,
    commands: String
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(15),
            canvas: Canvas::default(),
            read_canvas: true,
            commands: String::from("")
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self,  line: String) -> Result<(), std::num::ParseIntError> {
        if line.is_empty(){
            self.read_canvas=false;
        }
        else {
            if self.read_canvas {
                self.canvas.add_row(line.chars().collect());
            } else {
                self.commands.push_str(&line);
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        println!("Number of commands: {}", self.commands.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let points = self.canvas.try_locate_element(&'@')?;
        let obstacles = self.canvas.try_locate_element(&'#')?;
        let boxes = self.canvas.try_locate_element(&'O')?;
        if points.len()==1{
            let mut robot_position = points.first().unwrap().clone();
            let mut movable_boxes = boxes.clone();
            for c in self.commands.chars(){
               // println!("test {:?}", (c, *robot_position));
                let d = match c{
                            '>' => Direction::Right,
                            'v' => Direction::Up,
                            '<' => Direction::Left,
                            '^' => Direction::Down,
                            _ => unreachable!()
                };
                let test_position = &robot_position+&d;
                if obstacles.contains(&test_position){
                    continue;
                }else{
                    if movable_boxes.contains(&test_position){
                        let (condition, reversed): (Box<dyn Fn(&Rc<Point2D>, &Rc<Point2D>) -> bool> , bool)= match d {
                            Direction::Right => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).y() == (**r).y() && (**b).x()>(**r).x()), false),
                            Direction::Left => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).y() == (**r).y() && (**b).x()<(**r).x()), true),
                            Direction::Up => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).x() == (**r).x() && (**b).y()>(**r).y()), false),
                            Direction::Down => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).x() == (**r).x() && (**b).y()<(**r).y()), true),
                            _ => unreachable!()
                        };
                        let mut removed: Vec<_> = movable_boxes.iter().filter(|&x| condition(x, &robot_position)).cloned().collect();
                       // movable_boxes.retain(|x| !condition(x));
                        removed.sort_by_key(|point| *point.x());
                        if reversed {
                            removed.reverse();
                        }
                        let mut has_impulse: Option<bool> = Some(true);
                        let mut moved_boxes: HashSet<Rc<Point2D>> = HashSet::new();
                        for p in removed.iter(){
                            if let Some(impulse) = has_impulse{
                                if impulse {
                                    let p_moved = p + &d;
                                    if obstacles.contains(&p_moved) {
                                        has_impulse = None;
                                        break;
                                    }
                                    has_impulse = Some(movable_boxes.contains(&p_moved));
                                    moved_boxes.insert(p_moved);
                                }
                                else{
                                    moved_boxes.insert(p.clone());
                                }
                            }
                        }
                        //println!("impulse: {:?}", has_impulse);
                        if has_impulse == Some(false){
                            movable_boxes.retain(|x| !condition(x, &robot_position));
                            movable_boxes.extend(moved_boxes);
                            robot_position = test_position;
                        }
                        if has_impulse == Some(true){
                            unreachable!();
                        }
                       // println!("New elements: {:?}", movable_boxes);

                    }else{
                        robot_position = test_position;
                    }
                }
                //println!("res {:?}", (c, &robot_position));
            }
            let mut gps_sum: isize = 0;
            for b in movable_boxes.iter(){
                gps_sum+=(**b).x() + (**b).y()*100;
            }
            println!("gps {:?}", gps_sum);
            // let (width, height) = self.canvas.shape();
            // let mut rows: Vec<Vec<char>> = Vec::new();
            // for j in 0..*height{
            //     let mut row: Vec<char> = Vec::new();
            //     for i in 0..*width{
            //         let p =Rc::new(Point2D::new(i, j));
            //         if obstacles.contains(&p){
            //             row.push('#');
            //         }
            //         else if movable_boxes.contains(&p){
            //             row.push('O');
            //         }
            //         else if robot_position == p{
            //             row.push('@');
            //         }
            //         else{
            //             row.push('.');
            //         }
            //     }
            //     rows.push(row);
            // }
            // write_vec_to_file(rows, "boxes.txt").expect(&format!("Cannot draw boxes"));
            assert_display(gps_sum as usize, Some(10092), 1552879, "Sum of boxes GPS coordinates", test_mode)
        }else{
            Err(String::from("Multiple robot locations"))
        }
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}