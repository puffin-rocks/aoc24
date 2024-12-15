use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction{
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    None,
    ToPoint(Point2D)
}

impl Direction {
    pub(crate) fn mirror(self) -> Self{
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
            Direction::None => Direction::None,
            Direction::ToPoint(p) => Direction::ToPoint(Point2D::new(-p.x, -p.y))
        }
    }
    pub(crate) fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::UpRight => (1, 1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, -1),
            Direction::Down => (0, -1),
            Direction::DownLeft => (-1, -1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, 1),
            Direction::None => (0, 0),
            Direction::ToPoint(p) => (p.x, p.y)
        }
    }

    pub(crate) fn to_point(&self) -> Point2D{
        match self{
            Direction::ToPoint(p) => *p,
            _ => {
                let (dx, dy) = self.to_tuple();
                Point2D::new(dx,dy)
            }
        }
    }

    pub(crate) fn diagonal() -> [Direction; 4]{
        [Direction::UpRight, Direction::DownRight,
            Direction::DownLeft, Direction::UpLeft]
    }

    pub(crate) fn base() -> [Direction; 4]{
        [Direction::Up,  Direction::Right, Direction::Down, Direction::Left]
    }
}

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let p = self.to_point();
        p.hash(state);
    }
}

impl Eq for Direction {}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Point2D {
    x: isize,
    y: isize
}


impl Point2D {
    pub(crate) fn new<T>(x: T, y: T) -> Self
    where
        T: TryInto<isize>,
    {
        let x_converted = x.try_into().unwrap_or_else(|_| {
            eprintln!("Failed to convert x to isize, defaulting to 0");
            0
        });

        let y_converted = y.try_into().unwrap_or_else(|_| {
            eprintln!("Failed to convert y to isize, defaulting to 0");
            0
        });

        Self { x: x_converted, y: y_converted }
    }

    pub(crate) fn is_out_of_bounds(&self, width: usize, height: usize) -> bool{
        self.x < 0 || self.y < 0 || self.x > (width - 1) as isize || self.y > (height - 1) as isize
    }

    pub(crate) fn return_into_bounds(&mut self, width: usize, height: usize){
        fn coordinate_within_bounds(x:isize, bound: isize) -> isize{
            let x_rem = x.rem_euclid(bound);
            if x_rem<0{
                bound + x_rem
            }
            else{
                x_rem
            }
        }

        self.x = coordinate_within_bounds(self.x, width as isize);
        self.y = coordinate_within_bounds(self.y, height as isize);
    }

    pub(crate) fn x(&self) -> &isize{
        &self.x
    }

    pub(crate) fn y(&self) -> &isize{
        &self.y
    }

    pub(crate) fn rotate90(&self, n_rotations: u8, width: usize, height: usize) -> Point2D{
        let remainder = n_rotations % 4;
        match remainder {
            1 => Point2D::new(self.y, width as isize-self.x-1),
            2 => Point2D::new(width as isize-self.x-1, height as isize - self.y-1),
            3 => Point2D::new(height as isize - self.y-1, self.x),
            _ => {Point2D::new(self.x, self.y)}
        }
    }

    pub (crate) fn  get_point(&self, direction: Direction, length: usize) -> Point2D{
        self + &(&direction *length)
    }

}

impl Hash for Point2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state); // Include `x` in the hash
        self.y.hash(state); // Include `y` in the hash
    }
}

impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl Add<&Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Point2D> for &Point2D {
    type Output = Direction;

    fn sub(self, other: &Point2D) -> Direction {
        let p = Point2D::new(self.x - other.x, self.y - other.y);
        Direction::ToPoint(p)
    }
}

impl Add<&Direction> for &Point2D {
    type Output = Point2D;

    fn add(self, other: &Direction) -> Point2D {
        let p = other.to_point();
        Point2D {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl Add<&Direction> for &Direction {
    type Output = Direction;

    fn add(self, other: &Direction) -> Direction {
        let p1 = other.to_point();
        let p0 = self.to_point();
        Direction::ToPoint(Point2D::new(p0.x + p1.x, p0.y + p1.y))
    }
}

impl<T> Mul<T> for &Direction
where
    T: Copy + TryInto<isize>,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    type Output = Direction;

    fn mul(self, rhs: T) -> Direction {
        let rhs = rhs.try_into().expect("Conversion failed");
        let p = self.to_point();
        Direction::ToPoint(Point2D::new(p.x * rhs, p.y * rhs))
    }
}


#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Vector {
    direction: Direction,
    anchor: Point2D
}

impl Vector {
    pub(crate) fn new(direction: Direction, anchor: Point2D) -> Self{
        Self{
            direction,
            anchor
        }
    }
    pub(crate) fn null() -> Self{
        Self{
            direction: Direction::None,
            anchor: Point2D::new(0, 0)
        }
    }

    pub(crate) fn direction(&self) -> &Direction{
        &self.direction
    }
    pub(crate) fn anchor(&self) -> &Point2D{
        &self.anchor
    }

    pub(crate) fn is_out_of_bounds(&self, length: usize, width: usize, height: usize) -> bool {
        //either start or end of vector is out of bounds
        self.anchor.is_out_of_bounds(width, height) | self.get_point(length-1).is_out_of_bounds(width, height)
    }
    pub(crate) fn get_point(&self, length: usize) -> Point2D{
        &self.anchor + &(&self.direction *length)
    }

    pub(crate) fn shift(&self, d: &Direction) -> Vector {
        Vector::new(self.direction, &self.anchor + d)
    }

    #[allow(dead_code)]
    pub(crate) fn change_anchor(&mut self, anchor: Point2D){
        self.anchor = anchor;
    }

    pub(crate) fn change_direction(&mut self, direction: Direction){
        self.direction = direction;
    }
}

impl Eq for Vector {}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.direction.to_point().hash(state);
        self.anchor.hash(state);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Canvas {
    rows: Vec<Vec<Rc<char>>>,
    points: Vec<Rc<Point2D>>,
    elements: BTreeMap<Rc<char>, BTreeSet<Rc<Point2D>>>,
    width: usize,
    height: usize
}

impl Default for Canvas {
    fn default() -> Self {
        Self{
            rows:Vec::new(),
            points: Vec::new(),
            elements: BTreeMap::new(),
            width:0,
            height:0
        }
    }
}

impl Canvas {
    pub(crate) fn shape(&self) -> (&usize, &usize){
        (&self.width, &self.height)
    }

    pub(crate) fn add_row(&mut self, row: Vec<char>){
        if self.width>0 {assert_eq!(row.len(), self.width)}
        else {self.width = row.len()}
        self.height+=1;
        let mut rc_row: Vec<Rc<char>> = Vec::new();
        for (e, &r) in row.iter().enumerate(){
            let rc_char = Rc::new(r);
            let p = Rc::new(Point2D::new(e, self.height-1));

            self.elements.entry(Rc::clone(&rc_char))
                .or_insert_with(BTreeSet::new).insert(Rc::clone(&p));
            self.points.push(p);
            rc_row.push(rc_char);
        }
        self.rows.push(rc_row);
    }
    pub(crate) fn get_element(&self, point: &Point2D) -> Option<&char>{
        if point.is_out_of_bounds(self.width, self.height){
            None
        }else {
            Some(&self.rows[point.y as usize][point.x as usize])
        }
    }

    pub(crate) fn get_element_set(&self) -> BTreeSet<Rc<char>>{
        self.elements.keys().cloned().collect()
    }

    pub(crate) fn elements(&self) -> &BTreeMap<Rc<char>, BTreeSet<Rc<Point2D>>>{
        &self.elements
    }

    pub(crate) fn try_locate_element(&self, el: &char) -> Result<&BTreeSet<Rc<Point2D>>, String>{
        match self.elements.get(el){
            None => Err(format!("Cannot locate {}", el)),
            Some(locations) => {Ok(locations)}
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = Point2D>+ '_  {
        (0..self.width).flat_map(move |i| {
            (0..self.height).map(move |j| Point2D::new(i, j))
        })
    }
    #[allow(dead_code)]
    pub(crate) fn transpose(&self) -> Self {
        let rows: Vec<Vec<Rc<char>>> = (0..self.width).map( |i| {
                (0..self.height).map(|j| Rc::new(*self.rows[j][i])).collect()
        }).collect();

        let points: Vec<Rc<Point2D>> = (0..self.width)
            .flat_map(|i| {
                (0..self.height).map(move |j| Rc::new(Point2D::new(j, i)))
            })
            .collect();

        let elements: BTreeMap<Rc<char>, BTreeSet<Rc<Point2D>>> = self.elements
            .iter()
            .map(|(k, v)| {
                let transformed_set = v.iter()
                    .map(|p| Rc::new(Point2D::new(p.y, p.x)))
                    .collect();
                (Rc::new(**k), transformed_set)
            })
            .collect();
        Self{
            rows,
            points,
            elements,
            width: self.height,
            height: self.width
        }
    }
    #[allow(dead_code)]
    pub(crate) fn flip(&self) -> Self {
        let rows: Vec<Vec<Rc<char>>> = (0..self.height).map( |i| {
            self.rows[self.height-i-1].clone()
        }).collect();

        let points: Vec<Rc<Point2D>> = (0..self.width)
            .flat_map(|i| {
                (0..self.height).map(move |j| Rc::new(Point2D::new(i , self.height-j-1)))
            })
            .collect();

        let elements: BTreeMap<Rc<char>, BTreeSet<Rc<Point2D>>> = self.elements
            .iter()
            .map(|(k, v)| {
                let transformed_set = v.iter()
                    .map(|p| Rc::new(Point2D::new(p.x, self.height as isize - p.y - 1)))
                    .collect();
                (Rc::new(**k), transformed_set)
            })
            .collect();

        Self{
            rows,
            points,
            elements,
            width: self.width,
            height: self.height
        }
    }
    #[allow(dead_code)]
    pub(crate) fn transpose_flip(&self) -> Self {
        let rows: Vec<Vec<Rc<char>>> = (0..self.width).map( |i| {
            (0..self.height).map(|j| Rc::new(*self.rows[j][self.width-i-1])).collect()
        }).collect();

        let points: Vec<Rc<Point2D>> = (0..self.width)
            .flat_map(|i| {
                (0..self.height).map(move |j| Rc::new(Point2D::new(j, self.width-i-1)))
            })
            .collect();

        let elements: BTreeMap<Rc<char>, BTreeSet<Rc<Point2D>>> = self.elements
            .iter()
            .map(|(k, v)| {
                let transformed_set = v.iter()
                    .map(|p| Rc::new(Point2D::new(p.y, self.width as isize - p.x - 1) ))
                    .collect();
                (Rc::new(**k), transformed_set)
            })
            .collect();

        Self{
            rows,
            points,
            elements,
            width: self.height,
            height: self.width
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CanvasAsync {
    rows: Vec<Vec<Arc<char>>>,
    points: Vec<Arc<Point2D>>,
    elements: BTreeMap<Arc<char>, BTreeSet<Arc<Point2D>>>,
    width: usize,
    height: usize
}

impl Default for CanvasAsync {
    fn default() -> Self {
        Self{
            rows:Vec::new(),
            points: Vec::new(),
            elements: BTreeMap::new(),
            width:0,
            height:0
        }
    }
}
impl CanvasAsync {
    pub(crate) fn shape(&self) -> (&usize, &usize) {
        (&self.width, &self.height)
    }

    pub(crate) fn add_row(&mut self, row: Vec<char>) {
        if self.width > 0 { assert_eq!(row.len(), self.width) } else { self.width = row.len() }
        self.height += 1;
        let mut rc_row: Vec<Arc<char>> = Vec::new();
        for (e, &r) in row.iter().enumerate() {
            let rc_char = Arc::new(r);
            let p = Arc::new(Point2D::new(e, self.height - 1));

            self.elements.entry(Arc::clone(&rc_char))
                .or_insert_with(BTreeSet::new).insert(Arc::clone(&p));
            self.points.push(p);
            rc_row.push(rc_char);
        }
        self.rows.push(rc_row);
    }
    pub(crate) fn get_element(&self, point: &Point2D) -> Option<&char> {
        if point.is_out_of_bounds(self.width, self.height) {
            None
        } else {
            Some(&self.rows[point.y as usize][point.x as usize])
        }
    }

    // pub(crate) fn get_element_set(&self) -> BTreeSet<Arc<char>> {
    //     self.elements.keys().cloned().collect()
    // }

    pub(crate) fn elements(&self) -> &BTreeMap<Arc<char>, BTreeSet<Arc<Point2D>>> {
        &self.elements
    }

    // pub(crate) fn try_locate_element(&self, el: &char) -> Result<&BTreeSet<Arc<Point2D>>, String> {
    //     match self.elements.get(el) {
    //         None => Err(format!("Cannot locate {}", el)),
    //         Some(locations) => { Ok(locations) }
    //     }
    // }
    //
    // pub(crate) fn iter(&self) -> impl Iterator<Item=Point2D> + '_ {
    //     (0..self.width).flat_map(move |i| {
    //         (0..self.height).map(move |j| Point2D::new(i, j))
    //     })
    // }
}