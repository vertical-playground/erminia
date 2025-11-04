#[derive(Default, Debug, PartialEq)]
pub enum ErminiaType {
    #[default]
    Object,
    Int,
    String,
    // Tuple(Box<ErminiaType>),
    // List(Box<ErminiaType>)
}
