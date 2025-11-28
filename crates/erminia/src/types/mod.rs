use derive_more::Display;

#[derive(Display, Default, Debug, PartialEq)]
pub enum ErminiaType {
    #[default]
    Object,
    Ident(String),
    Integer(i32),
    Int,
    Bool(bool),
    String,
    Poisoned,
    Void,
}

impl ErminiaType {
    pub fn is_poisoned(&self) -> bool {
        matches!(self, ErminiaType::Poisoned)
    }

    pub fn to_id(&self) -> String {
        match self {
            ErminiaType::Object => "Object".into(),
            ErminiaType::Ident(id) => id.to_string(),
            ErminiaType::Int => "Int".into(),
            ErminiaType::Integer(i) => i.to_string(),
            ErminiaType::Bool(b) => b.to_string(),
            ErminiaType::String => "String".into(),
            ErminiaType::Poisoned => "Poisoned".into(),
            ErminiaType::Void => "Void".into(),
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            ErminiaType::Integer(i) => *i,
            _ => 0,
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            ErminiaType::Bool(b) => *b,
            _ => false,
        }
    }
}
