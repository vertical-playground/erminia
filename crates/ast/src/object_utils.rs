#![allow(unused)]

enum CoordPrior {
    X,
    Y
}

impl std::string::ToString for CoordPrior {
    fn to_string(&self) -> String {
        match self {
            CoordPrior::X => "x".to_string(),
            CoordPrior::Y => "y".to_string()
        }
    }
}

impl std::str::FromStr for CoordPrior {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(CoordPrior::X),
            "y" => Ok(CoordPrior::Y),
            _   => Err("Coordinate Priors can either be x or y".to_string())
        }
    }
}

struct Point {
    x: u32,
    y: u32
}
 
impl Point {
    fn new(
        x: u32,
        y: u32
    ) -> Self {
        Point { x, y }
    }
}

struct Range {
    left: u32,
    right: u32,
    l_incl: bool,
    r_incl: bool,
    prior: CoordPrior,
}

impl Range {
    fn new(
        left: u32,
        right: u32,
        l_incl: bool,
        r_incl: bool,
        prior: CoordPrior
    ) -> Self {
        Range { left, right, l_incl, r_incl, prior }
    }
}

struct ObjectCall {
    id: String,
    offset_x: u32,
    offset_y: u32
}

impl ObjectCall {
    fn new(
        id: String,
        offset_x: u32,
        offset_y: u32
    ) -> Self {
        ObjectCall { id, offset_x, offset_y }
    }
}

enum CoordIter {
    Coord(u32),
    Range(Range)
}

impl CoordIter {
    fn new_cost(coord: u32) -> Self {
        CoordIter::Coord(coord)
    }

    fn new_range(
        left: u32,
        right: u32,
        l_incl: bool,
        r_incl: bool,
        prior: CoordPrior
    ) -> Self {
        CoordIter::Range(Range::new(left, right, l_incl, r_incl, prior))
    }
}

enum ObjectShape {
    Point(Point),
    CoordIter(CoordIter),
    ObjectCall(ObjectCall),
    List(Vec<ObjectShape>)
}

impl ObjectShape {
    fn new_point(x: u32, y: u32) -> Self {
        ObjectShape::Point(Point::new(x,y))
    }

    fn new_coord_iter_const(coord: u32) -> Self {
        ObjectShape::CoordIter(CoordIter::new_cost(coord))
    }

    fn new_coord_iter(
        left: u32,
        right: u32,
        l_incl: bool,
        r_incl: bool,
        prior: CoordPrior
    ) -> Self {
        ObjectShape::CoordIter(CoordIter::new_range(left, right, l_incl, r_incl, prior))
    } 

    fn new_object(
        id: String,
        offset_x: u32,
        offset_y: u32
    ) -> Self {
        ObjectShape::ObjectCall(ObjectCall { id, offset_x, offset_y })
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
