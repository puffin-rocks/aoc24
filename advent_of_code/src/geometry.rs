use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction{
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    None
}

impl Direction {
    #[allow(dead_code)]
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
            Direction::None => Direction::None
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
        }
    }

    pub(crate) fn to_point(&self) -> Point2D{
        let (dx, dy) = self.to_tuple();
        Point2D::new(dx,dy)
    }

    pub(crate) fn list_valid() -> [Direction; 8]{
        [Direction::Up, Direction::UpRight, Direction::Right, Direction::DownRight,
            Direction::Down, Direction::DownLeft, Direction::Left, Direction::UpLeft]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Point2D {
    x: isize,
    y: isize
}

impl Point2D {
    pub(crate) fn new(x: isize, y: isize)->Self{
        Self{
            x,
            y
        }
    }

    pub(crate) fn is_out_of_bounds(&self, width: usize, height: usize) -> bool{
        self.x < 0 || self.y < 0 || self.x > (width - 1) as isize || self.y > (height - 1) as isize
    }
    #[allow(dead_code)]
    pub(crate) fn x(&self) -> &isize{
        &self.x
    }
    #[allow(dead_code)]
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

}

impl Hash for Point2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state); // Include `x` in the hash
        self.y.hash(state); // Include `y` in the hash
    }
}

impl Eq for Point2D {}

impl Add<&Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point2D> for &Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Mul<T> for &Point2D
where
    T: Copy + TryInto<isize>,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    type Output = Point2D;

    fn mul(self, rhs: T) -> Point2D {
        let rhs = rhs.try_into().expect("Conversion failed");
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}


#[derive(Debug)]
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

    pub(crate) fn is_out_of_bounds(&self, length: usize, width: usize, height: usize) -> bool {
        for i in 0..length{
            if self.get_point(i).is_out_of_bounds(width, height) {return true}
        }
        false
    }
    pub(crate) fn get_point(&self, length: usize) -> Point2D{
        &self.direction.to_point() *length + &self.anchor
    }

    pub(crate) fn shift(&self, p: &Point2D) -> Vector {
        Vector::new(self.direction, p + &self.anchor)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Canvas{
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl Default for Canvas {
    fn default() -> Self {
        Self{
            rows:Vec::new(),
            width:0,
            height:0
        }
    }
}

impl Canvas {
    pub(crate) fn width(&self) -> &usize{
        &self.width
    }
    pub(crate) fn height(&self) -> &usize{
        &self.height
    }
    pub(crate) fn add_row(&mut self, row: Vec<char>){
        if self.width>0 {assert_eq!(row.len(), self.width)}
        else {self.width = row.len()}
        self.rows.push(row);
        self.height+=1;
    }
    pub(crate) fn get_element(&self, point: &Point2D) -> char{
        self.rows[point.y as usize][point.x as usize]
    }

    pub(crate) fn locate_element(&self, el: char) -> HashSet<Point2D>{
        let mut result = HashSet::new();
        for p in self.iter(){
            if self.get_element(&p)==el{
                result.insert(p);
            }
        }
        result
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = Point2D>+ '_  {
        (0..self.width).flat_map(move |i| {
            (0..self.height).map(move |j| Point2D::new(i as isize, j as isize))
        })
    }
    #[allow(dead_code)]
    pub(crate) fn transpose(&self) -> Self {
        let rows: Vec<Vec<char>> = (0..self.width).map( |i| {
                (0..self.height).map(|j| self.rows[j][i]).collect()
        }).collect();
        Self{
            rows,
            width: self.height,
            height: self.width
        }
    }
    #[allow(dead_code)]
    pub(crate) fn flip(&self) -> Self {
        let rows: Vec<Vec<char>> = (0..self.height).map( |i| {
            self.rows[self.height-i-1].clone()
        }).collect();
        Self{
            rows,
            width: self.width,
            height: self.height
        }
    }
    #[allow(dead_code)]
    pub(crate) fn transpose_flip(&self) -> Self {
        let rows: Vec<Vec<char>> = (0..self.width).map( |i| {
            (0..self.height).map(|j| self.rows[j][self.width-i-1]).collect()
        }).collect();
        Self{
            rows,
            width: self.height,
            height: self.width
        }
    }
}
