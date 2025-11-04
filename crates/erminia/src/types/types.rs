#[derive(Debug, PartialEq)]
pub enum ErminiaType {
    Object,
    Int,
    String,
    // Tuple(Box<ErminiaType>),
    // List(Box<ErminiaType>)
}

impl Default for ErminiaType {
    fn default() -> Self {
        ErminiaType::Object
    }
}