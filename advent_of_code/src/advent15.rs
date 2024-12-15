use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use crate::geometry::{Canvas, Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};

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
                        let mut moved_boxes_before: HashSet<Rc<Point2D>> = HashSet::new();
                        let mut moved_boxes_after: HashSet<Rc<Point2D>> = HashSet::new();
                        let mut p = test_position.clone();
                        let mut could_move: bool = true;
                        loop{
                            moved_boxes_before.insert(p.clone());
                            let p_moved = &p + &d;
                            moved_boxes_after.insert(p_moved.clone());
                            if obstacles.contains(&p_moved) {
                                could_move = false;
                                break;
                            }
                            if movable_boxes.contains(&p_moved){
                                p = p_moved;
                            }
                            else{
                                break;
                            }
                        }
                        if could_move{
                            movable_boxes.retain(|x| !moved_boxes_before.contains(x));
                            movable_boxes.extend(moved_boxes_after);
                            robot_position = test_position;
                        }
                    }else{
                        robot_position = test_position;
                    }
                }
            }
            let gps_sum: isize = movable_boxes
                .iter()
                .map(|p| p.x() + p.y() * 100)
                .sum();
            assert_display(gps_sum as usize, Some(10092), 1552879, "Sum of boxes GPS coordinates", test_mode)
        }else{
            Err(String::from("Multiple robot locations"))
        }
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut canvas_streched = Canvas::default();
        let (width, height) = self.canvas.shape();
        for j in 0..*height{
            let mut row: Vec<char> = Vec::new();
            for i in 0..*width{
                match *self.canvas.get_element(&Point2D::new(i,j)).unwrap(){
                    '#' => {row.push('#'); row.push('#');},
                    '.' => {row.push('.'); row.push('.');},
                    'O' => {row.push('['); row.push(']');},
                    '@' => {row.push('@'); row.push('.');},
                    _ => unreachable!()
                }
            }
            canvas_streched.add_row(row);
        }
        let points = canvas_streched.try_locate_element(&'@')?;
        let obstacles = canvas_streched.try_locate_element(&'#')?;
        let boxes_left = canvas_streched.try_locate_element(&'[')?;
        let boxes_right = canvas_streched.try_locate_element(&']')?;
        if points.len()==1{
            let mut robot_position = points.first().unwrap().clone();
            type LabeledPoint = (Rc<Point2D>, char);
            fn reverse_label(label: &char)->char{
                if *label == ']'{
                    '['
                }
                else{
                    ']'
                }
            }
            let mut movable_boxes = boxes_left
                .iter()
                .map(|e| (e.clone(), '['))
                .chain(boxes_right.iter().map(|e| (e.clone(), ']')))
                .collect::<BTreeSet<_>>();
            
            for c in self.commands.chars(){
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
                    if movable_boxes.contains( &(test_position.clone(),']')) || movable_boxes.contains( &(test_position.clone(),'[')){
                        let mut moved_boxes_before: HashSet<LabeledPoint> = HashSet::new();
                        let mut moved_boxes_after: HashSet<LabeledPoint> = HashSet::new();
                        let mut could_move: bool = true;
                        match d {
                            Direction::Right | Direction::Left => {
                                let label= if movable_boxes.contains( &(test_position.clone(),']')){
                                    ']'
                                }
                                else{
                                    '['
                                };
                                let mut p= (test_position.clone(), label);
                                loop{
                                    let start_moved = &p.0 + &d;
                                    let end_moved = &p.0 + &(&d*2);
                                    moved_boxes_before.extend([p, (start_moved.clone(), reverse_label(&label))]);
                                    moved_boxes_after.extend([(start_moved.clone(), label), (end_moved.clone(), reverse_label(&label))]);
                                    if obstacles.contains(&end_moved){
                                        could_move = false;
                                        break;
                                    }
                                    if movable_boxes.contains( &(end_moved.clone(), label)){
                                        p = (end_moved, label);
                                    }
                                    else{
                                        break;
                                    }
                                }
                            },
                            Direction::Up | Direction::Down =>{
                                let mut stack: Vec<LabeledPoint> = Vec::new();

                                if movable_boxes.contains( &(test_position.clone(),']')){
                                    stack.push((test_position.clone(),']'));
                                    stack.push((&test_position+&Direction::Left,'['));
                                }
                                else{
                                    stack.push((test_position.clone(),'['));
                                    stack.push((&test_position+&Direction::Right,']'));
                                }
                                loop{
                                    let mut stack_next: Vec<LabeledPoint> = Vec::new();
                                    for (p, label) in stack.iter(){
                                        let p_moved = p + &d;
                                        moved_boxes_after.insert((p_moved.clone(), *label));
                                        if obstacles.contains(&p_moved){
                                            could_move = false;
                                            break;
                                        }
                                        if movable_boxes.contains( &(p_moved.clone(), *label)){
                                            stack_next.push((p_moved.clone(), *label));
                                        }
                                        let rev_label = reverse_label(label);
                                        if movable_boxes.contains( &(p_moved.clone(), rev_label)){
                                            stack_next.push((p_moved.clone(), rev_label));
                                            if rev_label == '[' {
                                                stack_next.push((&p_moved + &Direction::Right, ']'));
                                            }
                                            else{
                                                stack_next.push((&p_moved + &Direction::Left, '['));
                                            }
                                        }
                                    }
                                    moved_boxes_before.extend(stack);
                                    stack=stack_next;
                                    if stack.is_empty(){
                                        break;
                                    }
                                }

                            },
                            _ => unreachable!()
                        };
                        if could_move{
                            movable_boxes.retain(|x| !moved_boxes_before.contains(x));
                            movable_boxes.extend(moved_boxes_after);
                            robot_position = test_position;
                        }
                    }else{
                        robot_position = test_position;
                    }
                }
            }

            let gps_sum: isize = movable_boxes
                .iter()
                .filter(|(_, label)| *label == '[')
                .map(|(p, _)| p.x() + p.y() * 100)
                .sum();
            assert_display(gps_sum as usize, Some(9021), 1561175, "Sum of boxes GPS coordinates", test_mode)
        }else{
            Err(String::from("Multiple robot locations"))
        }
    }
}