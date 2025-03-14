#![allow(unused)]

use crate::obj_utils::error::{
    OUError,
    OUResult
};

#[derive(Debug, Eq, PartialEq)]
pub enum CoordPrior {
    X,
    Y,
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

#[derive(Debug, Eq, PartialEq)]
pub struct Const {
    value: u32
}

impl Const {
    pub fn new(value: u32) -> Self {
        Const { value }
    }

    pub fn get_value(&self) -> &u32 {
        &self.value
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Range {
    left: u32,
    right: u32,
    prior: CoordPrior,
}

impl Range {
    pub fn new(
        left: u32,
        right: u32,
        prior: CoordPrior
    ) -> Self {
        Range { left, right, prior }
    }

    pub fn get_left(&self) -> &u32 {
        &self.left
    }

    pub fn get_right(&self) -> &u32 {
        &self.right
    }

    pub fn get_prior(&self) -> &CoordPrior {
        &self.prior
    }

    pub fn get_range(&self) -> (&u32, &u32, &CoordPrior) {
        (&self.left, &self.right, &self.prior)
    }
}

#[derive(Debug, PartialEq)]
pub enum CoordIterPrior {
    Const(Const),
    Range(Range)
}

impl CoordIterPrior {
    pub fn new_const(value: u32) -> CoordIterPrior {
        CoordIterPrior::Const(Const::new(value))
    }

    pub fn is_const(&self) -> bool {
        match self {
            CoordIterPrior::Const(v) => true,
            CoordIterPrior::Range(range) => false
        }
    }

    pub fn get_const(&self) -> OUResult<&Const> {
        if self.is_const() {
            match self {
                CoordIterPrior::Const(v) => Ok(v),
                _ => unreachable!()
            }
        } else {
            Err(OUError::Error1)
        }
    }

    pub fn new_range(
        left: u32,
        right: u32,
        prior: CoordPrior
    ) -> CoordIterPrior {
        CoordIterPrior::Range(Range::new(left, right, prior))
    }

    pub fn is_range(&self) -> bool {
        match self {
            CoordIterPrior::Const(v) => false,
            CoordIterPrior::Range(range) => true
        }
    }

    pub fn get_range(&self) -> OUResult<&Range> {
        if self.is_range() {
            match self {
                CoordIterPrior::Range(r) => Ok(r),
                _ => unreachable!()
            }
        } else {
            Err(OUError::Error1)
        }
    }
}

#[derive(Debug)]
pub struct CoordIter {
    left: CoordIterPrior,
    right: CoordIterPrior,
}

impl CoordIter {
    pub fn new(
        left: CoordIterPrior,
        right: CoordIterPrior
    ) -> Self {
        CoordIter { left, right }
    }

    pub fn get_left(&self) -> &CoordIterPrior {
        &self.left
    }

    pub fn get_right(&self) -> &CoordIterPrior {
        &self.right
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get() -> OUResult<()> {
        let val = Const::new(1);

        let con = val.get_value();

        assert_eq!(*con, 1);

        Ok(())
    }

    #[test]
    fn check_get2() {
        let val = Range::new(1,2,CoordPrior::X);

        let con = val.get_prior();

        assert_eq!(*con, CoordPrior::X);
    }

    #[test]
    fn check_get_value() -> OUResult<()> {
        let iter = CoordIter::new(
            CoordIterPrior::new_const(1),
            CoordIterPrior::new_range(1,2,CoordPrior::X)
        );

        let left = iter.get_left();

        if left.is_const() {
            let left = left.get_const()?;

            assert_eq!(*left.get_value(), 1);
        }

        Ok(())
    }

    #[test]
    fn check_get_range() -> OUResult<()> {
        let right = CoordIterPrior::new_range(1,2,CoordPrior::X);

        if right.is_range() {
            let value = right.get_range()?; 
            assert_eq!(*value.get_prior(), CoordPrior::X);
        }

        Ok(())
    }
}
