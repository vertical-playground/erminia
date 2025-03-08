#![allow(unused)]

type TypeError = String;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    IntType,
    ObjectType,
    SuperObjectType
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::IntType => write!(f, "IntType"),
            Type::ObjectType => write!(f, "ObjectType"),
            Type::SuperObjectType => write!(f, "SuperObjectType"),
        }
    }
}

fn type_equality(t1: & Type, t2: & Type) -> bool {
    if t1.eq(t2) {
        return true;
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_equal() {
        let t1: Type = Type::IntType;
        let t2: Type = Type::IntType;

        assert_eq!(true, type_equality(&t1,&t2));
    }

    #[test]
    fn check_neq() {
        let t1: Type = Type::ObjectType;
        let t2: Type = Type::SuperObjectType;

        assert_ne!(true, type_equality(&t1,&t2));
    }
}

