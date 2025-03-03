use crate::lexer::SyntaxKind;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub(crate) enum ErminiaLanguage {}

impl rowan::Language for ErminiaLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }

}

pub(crate) type SyntaxNode = rowan::SyntaxNode<ErminiaLanguage>;
