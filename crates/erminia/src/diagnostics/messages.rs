use std::fmt;

pub enum MessageKind {
    Note,
    Help,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Note {
    ExpectedLeftInclusive,
    ExpectedRightInclusive,
    ExpectedDataType,
    ExpectedInteger,
    ExpectedIdentifier,
    ExpectedSomethingElse,
    ExpectedASTNode,
    ExpectedStatement,
    ExpectedIDorInteger,
    ExpectedShapeOrColor,
    ExpectedTypeofTuple,
    #[default]
    None,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stringify(vec![]))
    }
}

impl Note {
    pub fn stringify(&self, args: Vec<String>) -> String {
        match self {
            Note::ExpectedLeftInclusive => format!(
                "Expected '(' or '[' for range inclusivity, but found '{}'.",
                args[0]
            ),
            Note::ExpectedRightInclusive => format!(
                "Expected ')' or ']' for range inclusivity, but found '{}'.",
                args[0]
            ),
            Note::ExpectedDataType => format!("Expected a data type, but found '{}'.", args[0]),
            Note::ExpectedInteger => {
                format!("Expected an integer constant, but found '{}'.", args[0])
            }
            Note::ExpectedIdentifier => format!("Expected an identifier, but found '{}'.", args[0]),
            Note::ExpectedSomethingElse => {
                format!("Expected '{}', but found '{}'.", args[0], args[1])
            }
            Note::ExpectedASTNode => format!(
                "Expected '{}' AST Node, but failed to parse Node with id: '{}'.",
                args[0], args[1]
            ),
            Note::ExpectedIDorInteger => format!(
                "Expected an identifier or integer constant, but found '{}'.",
                args[0]
            ),
            Note::ExpectedStatement => {
                format!("Expected a statement keyword, but found '{}'.", args[0])
            }
            Note::ExpectedShapeOrColor => format!(
                "Expected a shape or color keyword, but found '{}'.",
                args[0]
            ),
            Note::ExpectedTypeofTuple => format!(
                "Expected a 'tuple' or 'object' type, but found '{}'.",
                args[0]
            ),
            Note::None => "".to_string(),
        }
    }

    pub fn args_required(&self) -> bool {
        match self {
            Note::ExpectedLeftInclusive => true,
            Note::ExpectedRightInclusive => true,
            Note::ExpectedDataType => true,
            Note::ExpectedInteger => true,
            Note::ExpectedIdentifier => true,
            Note::ExpectedSomethingElse => true,
            Note::ExpectedASTNode => true,
            Note::ExpectedIDorInteger => true,
            Note::ExpectedStatement => true,
            Note::ExpectedShapeOrColor => true,
            Note::ExpectedTypeofTuple => true,
            Note::None => false,
        }
    }

    pub fn args_count(&self) -> usize {
        match self {
            Note::ExpectedLeftInclusive => 1,
            Note::ExpectedRightInclusive => 1,
            Note::ExpectedDataType => 1,
            Note::ExpectedInteger => 1,
            Note::ExpectedIdentifier => 1,
            Note::ExpectedSomethingElse => 2,
            Note::ExpectedASTNode => 2,
            Note::ExpectedIDorInteger => 1,
            Note::ExpectedStatement => 1,
            Note::ExpectedShapeOrColor => 1,
            Note::ExpectedTypeofTuple => 1,
            Note::None => 0,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Help {
    ConsiderChangingToInclusive,
    DidYouMeanDataType,
    DidYouMeanStmtKeyword,
    DidYouMeanShapeOrColor,
    DidYouMeanTupleorObject,
    #[default]
    None,
}

impl fmt::Display for Help {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

impl Help {
    pub fn stringify(&self) -> String {
        match self {
            Help::ConsiderChangingToInclusive => {
                "Consider changing to an inclusive range by using '[' or ']'.".to_string()
            }
            Help::DidYouMeanDataType => {
                "Did you mean to use 'int', 'string', or 'object' as data type?".to_string()
            }
            Help::DidYouMeanStmtKeyword => {
                "Did you mean to use a statement keyword like 'let', 'input', 'output', 'example', or 'solution'?".to_string()
            }
            Help::DidYouMeanShapeOrColor => {
                "Did you mean to use the 'shape' or 'color' keywords?".to_string()
            }
            Help::DidYouMeanTupleorObject => {
                "Did you mean to use a 'tuple' or an 'object' type? You can create tuples by using this syntax: <tuple> ::= '(' <int> ',' <int> ')' or use a declared object instead.".to_string()
            }
            Help::None => "".to_string(),
        }
    }
}
