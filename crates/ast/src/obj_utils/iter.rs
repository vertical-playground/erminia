pub enum CoordPrior {
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

pub struct Range {
    left: u32,
    right: u32,
    l_incl: bool,
    r_incl: bool,
    prior: CoordPrior,
}

impl Range {
    pub fn new(
        left: u32,
        right: u32,
        l_incl: bool,
        r_incl: bool,
        prior: CoordPrior
    ) -> Self {
        Range { left, right, l_incl, r_incl, prior }
    }
}

pub enum CoordIter {
    Coord(u32),
    Range(Range)
}

impl CoordIter {
    pub fn new_cost(coord: u32) -> Self {
        CoordIter::Coord(coord)
    }

    pub fn new_range(
        left: u32,
        right: u32,
        l_incl: bool,
        r_incl: bool,
        prior: CoordPrior
    ) -> Self {
        CoordIter::Range(Range::new(left, right, l_incl, r_incl, prior))
    }
}

pub enum CoordIterTuple {
    Left(CoordIter),
    Right(CoordIter)
}
