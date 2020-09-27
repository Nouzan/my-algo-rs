use std::cmp::{Ordering, PartialOrd};
use std::fmt;

pub enum Operator {
    End,
    LeftParenthese,
    RightParenthese,
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::End => write!(f, "\0"),
            Self::LeftParenthese => write!(f, "("),
            Self::RightParenthese => write!(f, ")"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::End, Self::End) => true,
            (Self::LeftParenthese, Self::RightParenthese) => true,
            (Self::RightParenthese, Self::LeftParenthese) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            match (self, other) {
                (_, Self::End) => Some(Ordering::Greater),
                (Self::End, _) => Some(Ordering::Less),
                (Self::LeftParenthese, _) => Some(Ordering::Less),
                (_, Self::RightParenthese) => Some(Ordering::Greater),
                (_, Self::LeftParenthese) => Some(Ordering::Less),
                (Self::Mul, Self::Mul)
                | (Self::Mul, Self::Div)
                | (Self::Div, Self::Div)
                | (Self::Div, Self::Mul) => Some(Ordering::Greater),
                (Self::Add, Self::Add)
                | (Self::Add, Self::Sub)
                | (Self::Sub, Self::Sub)
                | (Self::Sub, Self::Add) => Some(Ordering::Greater),
                (Self::Mul, Self::Add)
                | (Self::Mul, Self::Sub)
                | (Self::Div, Self::Add)
                | (Self::Div, Self::Sub) => Some(Ordering::Greater),
                (Self::Add, Self::Mul)
                | (Self::Sub, Self::Mul)
                | (Self::Add, Self::Div)
                | (Self::Sub, Self::Div) => Some(Ordering::Less),
                _ => None,
            }
        }
    }
}
