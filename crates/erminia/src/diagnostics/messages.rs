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
    ExpectedIDorInteger,
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
            Note::None => false,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Help {
    ConsiderChangingToInclusive,
    DidYouMeanDataType,
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
            Help::None => "".to_string(),
        }
    }
}
