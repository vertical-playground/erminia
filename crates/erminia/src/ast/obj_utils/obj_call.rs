#![allow(unused)]

use crate::obj_utils::point::Point;

#[derive(Debug)]
pub enum Offset {
    Offset(Point),
    None
}

impl Offset {
    pub fn new_offset(offset_x: u32, offset_y: u32) -> Self {
        Offset::Offset(Point::new(offset_x, offset_y))
    }

    pub fn new_none_offset() -> Self {
        Offset::None
    }
}

#[derive(Debug)]
pub struct ObjectCall {
    id: String,
    offset: Offset
}

impl ObjectCall {
    pub fn new(
        id: String,
        offset: Offset
    ) -> Self {
        ObjectCall { id, offset }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_offset(&self) -> &Offset {
        &self.offset
    }
}
