#![allow(unused)]

use crate::obj_utils::error::{
    OUError,
    OUResult
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    pub fn new_const(value: u32) -> Self {
        CoordIterPrior::Const(Const::new(value))
    }

    pub fn new_range(
        left: u32,
        right: u32,
        prior: CoordPrior
    ) -> Self {
        CoordIterPrior::Range(Range::new(left, right, prior))
    }

    pub fn get<T>(&self) -> OUResult<&T>
    where
        T: 'static,
    {
        match self {
            CoordIterPrior::Const(v) if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Const>() => {
                Ok(unsafe { &*(v as *const Const as *const T) }) // Safe cast since we checked the type
            }
            CoordIterPrior::Range(range) if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Range>() => {
                Ok(unsafe { &*(range as *const Range as *const T) })
            }
            _ => Err(OUError::Error1.into()), // Return an error if the type does not match
        }
    }

    pub fn get_value(&self) -> OUResult<&Const> {
        match self {
            CoordIterPrior::Const(v) => Ok(v),
            CoordIterPrior::Range(range) => Err(OUError::Error1)
        }
    }

    pub fn get_range(&self) -> OUResult<&Range> {
        match self {
            CoordIterPrior::Range(range) => Ok(range),
            CoordIterPrior::Const(v) => Err(OUError::Error1)
        }
    }
}

pub struct CoordIter {
    left: CoordIterPrior,
    right: CoordIterPrior,
}

impl CoordIter {
    pub fn new(left: CoordIterPrior, right: CoordIterPrior) -> Self {
        CoordIter {
            left,
            right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get() -> OUResult<()> {
        let val = CoordIterPrior::new_const(1);

        let con = val.get::<Const>()?;

        assert_eq!(*con.get_value(), 1);

        Ok(())
    }

    #[test]
    fn check_get2() -> OUResult<()> {
        let val = CoordIterPrior::new_range(1, 2, CoordPrior::X);

        let con = val.get::<Range>()?;

        assert_eq!(*con.get_prior(), CoordPrior::X);

        Ok(())
    }

    #[test]
    fn check_get_value() -> OUResult<()> {
        let left = CoordIterPrior::new_const(1);
        let right = CoordIterPrior::new_range(1,2,CoordPrior::X);

        let coorditer = CoordIter::new(left, right);

        let value = coorditer.left.get_value()?;

        assert_eq!(*value.get_value(), 1);

        Ok(())
    }

    #[test]
    fn check_get_range() -> OUResult<()> {
        let right = CoordIterPrior::new_range(1,2,CoordPrior::X);
        let value = &right.get_range()?; 

        assert_eq!(*value.get_prior(), CoordPrior::X);

        Ok(())
    }
}
