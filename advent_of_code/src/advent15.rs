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
                        let (condition, reversed): (Box<dyn Fn(&Rc<Point2D>, &Rc<Point2D>) -> bool> , bool)= match d {
                            Direction::Right => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).y() == (**r).y() && (**b).x()>(**r).x()), false),
                            Direction::Left => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).y() == (**r).y() && (**b).x()<(**r).x()), true),
                            Direction::Up => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).x() == (**r).x() && (**b).y()>(**r).y()), false),
                            Direction::Down => (Box::new(|b: &Rc<Point2D>, r: &Rc<Point2D>| (**b).x() == (**r).x() && (**b).y()<(**r).y()), true),
                            _ => unreachable!()
                        };
                        let mut removed: Vec<_> = movable_boxes.iter().filter(|&x| condition(x, &robot_position)).cloned().collect();
                        match d{
                            Direction::Right|Direction::Left =>removed.sort_by_key(|point| *point.x()),
                            Direction::Up|Direction::Down =>removed.sort_by_key(|point| *point.y()),
                            _ => unreachable!()
                        }
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
                        if has_impulse == Some(false){
                            movable_boxes.retain(|x| !condition(x, &robot_position));
                            movable_boxes.extend(moved_boxes);
                            robot_position = test_position;
                        }
                        if has_impulse == Some(true){
                            unreachable!();
                        }
                    }else{
                        robot_position = test_position;
                    }
                }
            }
            let mut gps_sum: isize = 0;
            for b in movable_boxes.iter(){
                gps_sum+=(**b).x() + (**b).y()*100;
            }
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
            let mut movable_boxes = boxes_left.iter().map(|e| (e.clone(), '[')).collect::<BTreeSet<_>>();
            movable_boxes.extend(boxes_right.iter().map(|e| (e.clone(), ']')));
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
                        match d {
                            Direction::Right | Direction::Left => {
                                let (condition, reversed): (Box<dyn Fn(&LabeledPoint, &Rc<Point2D >) -> bool> , bool)=
                                if d == Direction::Right {
                                    (Box::new(|b: &LabeledPoint, r: &Rc<Point2D>| (*(*b).0).y() == (**r).y() && (*(*b).0).x() > (**r).x()), false)
                                }
                                else{
                                    (Box::new(|b: &LabeledPoint, r: &Rc<Point2D>| (*(*b).0).y() == (**r).y() && (*(*b).0).x() < (**r).x()), true)
                                };
                                let mut removed: Vec<_> = movable_boxes.iter().filter(|&x| condition(x, &robot_position)).cloned().collect();
                                removed.sort_by_key(|point| *point.0.x());
                                if reversed {
                                    removed.reverse();
                                }

                                let mut has_impulse: Option<bool> = Some(true);
                                let mut moved_boxes: HashSet<LabeledPoint> = HashSet::new();
                                for (p, label) in removed.iter().step_by(2){
                                    if let Some(impulse) = has_impulse{
                                        if impulse {
                                            let start_moved = p + &d;
                                            let end_moved = p + &(&d*2);
                                            if obstacles.contains(&end_moved) {
                                                has_impulse = None;
                                                break;
                                            }
                                            has_impulse = Some(movable_boxes.contains(&(end_moved.clone(),*label)));
                                            moved_boxes.insert((start_moved, *label));
                                            moved_boxes.insert( (end_moved, reverse_label(label)));
                                        }
                                        else{
                                            moved_boxes.insert( (p.clone(), *label));
                                            moved_boxes.insert( ((p + &d).clone(), reverse_label(label)));
                                        }
                                    }
                                }
                                if has_impulse == Some(false){
                                    movable_boxes.retain(|x| !condition(x, &robot_position));
                                    movable_boxes.extend(moved_boxes);
                                    robot_position = test_position;
                                }
                                if has_impulse == Some(true){
                                    unreachable!();
                                }
                            },
                            Direction::Up | Direction::Down =>{
                                let mut moved_boxes_before: HashSet<LabeledPoint> = HashSet::new();

                                let mut stack: Vec<LabeledPoint> = Vec::new();
                                let mut could_move: bool = true;
                                let mut moved_boxes_after: HashSet<LabeledPoint> = HashSet::new();
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
                                if could_move{
                                    movable_boxes.retain(|x| !moved_boxes_before.contains(x));
                                    movable_boxes.extend(moved_boxes_after);
                                    robot_position = test_position;
                                }
                            },
                            _ => unreachable!()
                        };
                    }else{
                        robot_position = test_position;
                    }
                }
            }

            let mut gps_sum: isize = 0;
            for (p, label) in movable_boxes.iter(){
                if *label == '[' {
                    gps_sum += (**p).x() + (**p).y() * 100;
                }
            }
            assert_display(gps_sum as usize, Some(9021), 1561175, "Sum of boxes GPS coordinates", test_mode)
        }else{
            Err(String::from("Multiple robot locations"))
        }
    }
}