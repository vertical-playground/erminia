use std::fmt;

pub enum MessageKind {
    Note,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Note {
    ExpectedLeftInclusive(String),
    ExpectedRightInclusive(String),
    ExpectedDataType(String),
    ExpectedInteger(String),
    ExpectedIdentifier(String),
    ExpectedSomethingElse(String, String),
    ExpectedASTNode(String, String),
    ExpectedStatement(String),
    ExpectedIDorInteger(String),
    ExpectedShapeOrColor(String),
    ExpectedTypeofTuple(String),
    FileNotFound(String),
    ExpectedFileToHaveErmPostfix(String),
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

impl Note {
    pub fn stringify(&self) -> String {
        match self {
            Note::ExpectedLeftInclusive(str1) => format!(
                "Expected '(' or '[' for range inclusivity, but found '{}'.",
                str1
            ),
            Note::ExpectedRightInclusive(str1) => format!(
                "Expected ')' or ']' for range inclusivity, but found '{}'.",
                str1
            ),
            Note::ExpectedDataType(str1) => format!("Expected a data type, but found '{}'.", str1),
            Note::ExpectedInteger(str1) => {
                format!("Expected an integer constant, but found '{}'.", str1)
            }
            Note::ExpectedIdentifier(str1) => {
                format!("Expected an identifier, but found '{}'.", str1)
            }
            Note::ExpectedSomethingElse(str1, str2) => {
                format!("Expected '{}', but found '{}'.", str1, str2)
            }
            Note::ExpectedASTNode(str1, str2) => format!(
                "Expected '{}' AST Node, but failed to parse Node with id: '{}'.",
                str1, str2
            ),
            Note::ExpectedIDorInteger(str1) => format!(
                "Expected an identifier or integer constant, but found '{}'.",
                str1
            ),
            Note::ExpectedStatement(str1) => {
                format!("Expected a statement keyword, but found '{}'.", str1)
            }
            Note::ExpectedShapeOrColor(str1) => {
                format!("Expected a shape or color keyword, but found '{}'.", str1)
            }
            Note::ExpectedTypeofTuple(str1) => {
                format!("Expected a 'tuple' or 'object' type, but found '{}'.", str1)
            }
            Note::FileNotFound(str1) => {
                format!(
                    "File '{}' could not be retrieved, please try a different file.",
                    str1
                )
            }
            Note::ExpectedFileToHaveErmPostfix(str1) => {
                format!(
                    "Expected file to have '.erm' postfix but instead got '{}'.",
                    str1
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Help {
    ConsiderChangingToInclusive,
    DidYouMeanDataType,
    DidYouMeanStmtKeyword,
    DidYouMeanShapeOrColor,
    DidYouMeanTupleorObject,
    DidYouMeanToUseAnErmPostfix,
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
            Help::DidYouMeanToUseAnErmPostfix => {
                "Did you mean to use an '.erm' postfix on your input file.".to_string()
            }
        }
    }
}
