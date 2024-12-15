use std::collections::HashSet;
use std::rc::Rc;
use itertools::Itertools;
use crate::geometry::{Canvas, Direction, Point2D};
use crate::utils::{Solve, Label};

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
        println!("Symbols: {:?}", self.commands.chars().collect::<HashSet<char>>());
        //println!("{:?}", self.canvas.elements());
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
                let d = match c{
                            '>' => Direction::Right,
                            'v' => Direction::Up,
                            '<' => Direction::Left,
                            '^' => Direction::Down,
                            x => {panic!("Unexpected direction {}", x)}
                };
                let test_position = &robot_position+&d;
                //println!("test {:?}", (c, &test_position));
                if obstacles.contains(&test_position){
                    continue;
                }else{
                    if movable_boxes.contains(&test_position){
                        let (condition, reversed): (Box<dyn Fn(&Rc<Point2D>) -> bool> , bool)= match d {
                            Direction::Left | Direction::Right => (Box::new(|b: &Rc<Point2D>| (**b).y() == test_position.y()), d==Direction::Right),
                            Direction::Up | Direction::Down => (Box::new(|b: &Rc<Point2D>| (**b).x() == test_position.x()), d==Direction::Up),
                            _ => (Box::new(|_: &Rc<Point2D>| false), false),
                        };
                        let mut removed: Vec<_> = movable_boxes.iter().filter(|&x| condition(x)).cloned().collect();
                        movable_boxes.retain(|x| !condition(x));
                        removed.sort_by_key(|point| *point.x());
                        if reversed {
                            removed.reverse();
                        }
                        println!("Removed elements: {:?}", removed);
                        println!("Remaining elements: {:?}", movable_boxes);

                    }else{
                        robot_position = test_position;
                    }
                }
                //println!("res {:?}", (c, &robot_position));
            }
            Err(String::from("Not solved yet"))
        }else{
            Err(String::from("Multiple robot locations"))
        }
    }
    // fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Not solved yet"))
    // }
}