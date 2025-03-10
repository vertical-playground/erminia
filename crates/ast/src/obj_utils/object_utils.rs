#![allow(unused)]

use crate::obj_utils::point::Point;
use crate::obj_utils::obj_call::{
    Offset,
    ObjectCall,
};
use crate::obj_utils::error::{
    OUError,
    OUResult
};
use crate::obj_utils::iter::{
    CoordPrior,
    Range,
    CoordIterPrior,
    CoordIter
};

enum ObjectShape {
    Point(Point),
    CoordIter(CoordIterPrior),
    ObjectCall(ObjectCall),
}

impl ObjectShape {
    fn new_point(x: u32, y: u32) -> Self {
        ObjectShape::Point(Point::new(x,y))
    }

    fn new_coord_iter_const(coord: u32) -> Self {
        ObjectShape::CoordIter(CoordIterPrior::new_const(coord))
    }

    fn new_coord_iter(
        left: u32,
        right: u32,
        l_incl: bool,
        r_incl: bool,
        prior: CoordPrior
    ) -> Self {
        ObjectShape::CoordIter(CoordIterPrior::new_range(left, right, prior))
    } 

    fn new_object(
        id: String,
        offset: Offset
    ) -> Self {
        ObjectShape::ObjectCall(ObjectCall::new(id, offset))
    }
}

struct ObjectShapeVec {
    vector: Vec<ObjectShape>
}

impl ObjectShapeVec {
    fn new() -> Self {
        ObjectShapeVec {
            vector: Vec::<ObjectShape>::new()
        }
    }
    
    fn add_obj(object: ObjectShape) {
        todo!()
    }

    fn len(&self) -> usize {
        self.vector.len()
    }

    fn remove_obj_at(&mut self, at: usize) -> Result<ObjectShape, OUError> {
        if at > self.len()-1 {
            return Err(OUError::Error1)
        }

        let o: ObjectShape = self.vector.remove(at);
        Ok(o)
    }

    fn remove_obj_last(&mut self) -> Result<ObjectShape, OUError> {

        let o = self.vector.pop();

        return match o {
            Some(o) => Ok(o),
            None => Err(OUError::Error1)
        };
    }
}

enum ObjectColor {
    Pink,    //#E53AA3
    DarkRed, //#921231
    Red,     //#F93C31
    Orange,  //#FF851B
    Yellow,  //#FFDC00
    Green,   //#4FCC30
    Blue,    //#1E93FF
    Cyan,    //#87D8F1
    Gray,    //#999999
    Black,   //#000000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_to_string() {
        let c = CoordPrior::X;
        assert_eq!("x".to_string(), c.to_string());
    }
}
