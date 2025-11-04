use crate::lexer::token::Position;

// ==================================================================================== //
// Location Struct                                                                      //
// ==================================================================================== //

#[derive(Debug)]
pub struct Location {
    _position: Position,
}

impl Location {
    pub fn new(position: Position) -> Self {
        Location {
            _position: position,
        }
    }
}
