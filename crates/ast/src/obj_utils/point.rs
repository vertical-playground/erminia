#![allow(unused)]

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32
}
 
impl Point {
    pub fn new(
        x: u32,
        y: u32
    ) -> Self {
        Point { x, y }
    }

    pub fn get_left(&self) -> &u32 {
        &self.x
    }

    pub fn get_right(&self) -> &u32 {
        &self.y
    }
}
