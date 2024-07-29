use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ptr::write;
use num::ToPrimitive;
use crate::structs::arrays::HeapArray;
use crate::structs::strings::HeapString;

#[derive(PartialEq, Debug)]
pub enum Associativity {
    Left,
    Right
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Negate,
    Factorial,
    Deref,
    Unknown
}

impl Operator {
    fn from(value: char, unary: bool) -> Operator {
        match value {
            '+' => Operator::Plus,
            '-' => {
                if unary {
                    Operator::Negate
                } else {
                    Operator::Minus
                }
            },
            '*' => {
                if unary {
                    Operator::Deref
                } else {
                    Operator::Multiply
                }
            },
            '/' => Operator::Divide,
            '^' => Operator::Exponent,
            '!' => Operator::Factorial,
            _ => Operator::Unknown
        }
    }

    fn charset() -> HashSet<char> {
        HashSet::from(['+', '-', '*', '/', '^', '!'])
    }

    pub fn to_char(&self) -> Option<char> {
        match self {
            Operator::Plus => Some('+'),
            Operator::Minus => Some('-'),
            Operator::Multiply => Some('*'),
            Operator::Deref => Some('*'),
            Operator::Divide => Some('/'),
            Operator::Exponent => Some('^'),
            Operator::Negate => Some('-'),
            Operator::Factorial => Some('!'),
            _ => None
        }
    }

    pub(crate) fn associativity(&self) -> Associativity {
        match self {
            Operator::Plus => Associativity::Left,
            Operator::Minus => Associativity::Left,
            Operator::Multiply => Associativity::Left,
            Operator::Divide => Associativity::Left,
            Operator::Exponent => Associativity::Right,
            Operator::Negate => Associativity::Right,
            Operator::Factorial => Associativity::Right,
            Operator::Deref => Associativity::Right,
            Operator::Unknown => Associativity::Left
        }
    }

    pub(crate) fn precedence(&self) -> usize {
        match self {
            Operator::Plus => 1,
            Operator::Minus => 1,
            Operator::Multiply => 2,
            Operator::Divide => 2,
            Operator::Exponent => 3,
            Operator::Negate => 4,
            Operator::Factorial => 4,
            Operator::Deref => 4,
            Operator::Unknown => 100,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Punctuation {
    // Semicolon,
    // Comma,
    // LeftBrace,
    // RightBrace,
    LeftParen,
    RightParen,
    Unknown
}

impl From<char> for Punctuation {
    fn from(value: char) -> Self {
        match value {
            '(' => Punctuation::LeftParen,
            ')' => Punctuation::RightParen,
            _ => Punctuation::Unknown
        }
    }
}

impl Punctuation {
    fn charset() -> HashSet<char> {
        HashSet::from(['(', ')'])
    }

    fn to_char(&self) -> Option<char> {
        match self {
            Punctuation::LeftParen => Some('('),
            Punctuation::RightParen => Some(')'),
            _ => None
        }
    }

    pub fn precedence(&self) -> usize {
        match self {
            Self::LeftParen => 0,
            Self::RightParen => 0,
            Self::Unknown => 100,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token {
    // Keyword(HeapString),
    // Identifier(HeapString),
    Letter(char),
    Digit(u8),
    Operator(Operator),
    // Number(u8),
    Punctuation(Punctuation),
    Whitespace,
    Unknown(char),
}

impl Token {
    pub fn tokenize(expression: &HeapString) -> HeapArray<Self> {
        let mut tokens: HeapArray<Self> = HeapArray::with_capacity(expression.len());
        for &c in expression.iter() {
            tokens.push(Token::from(c as char));
        }
        tokens
    }

    pub fn to_char(&self) -> Option<char> {
        // let foo = self.deref().;
        // self.deref().to_char()
        match self {
            Token::Letter(c) => Some(*c),
            Token::Digit(d) => Some((*d).into()),
            Token::Operator(op) => op.to_char(),
            Token::Punctuation(punc) => punc.to_char(),
            Token::Whitespace => Some(' '),
            // Operator::Plus => Some('+'),
            // Operator::Minus => Some('-'),
            // Operator::Multiply => Some('*'),
            // Operator::Deref => Some('*'),
            // Operator::Divide => Some('/'),
            // Operator::Exponent => Some('^'),
            // Operator::Negate => Some('-'),
            // Operator::Factorial => Some('!'),
            _ => None
        }
    }

    pub fn precedence(&self) -> usize {
        match self {
            Token::Letter(c) => 100,
            Token::Digit(d) => 100,
            Token::Operator(op) => op.precedence(),
            Token::Punctuation(punc) => punc.precedence(),
            Token::Whitespace => 100,
            _ => 100
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            c if Punctuation::charset().contains(&c) => Token::Punctuation(Punctuation::from(value)),
            c if Operator::charset().contains(&c) => Token::Operator(Operator::from(value, false)),
            ' ' => Token::Whitespace,
            'a'..='z' => Token::Letter(value),
            'A'..='Z' => Token::Letter(value),
            '0'..='9' => Token::Digit(value.to_digit(10).unwrap() as u8),
            _ => Token::Unknown(value)
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Punctuation(val) => write!(f, "{}", val.to_char().unwrap())?,
            _ => write!(f, "None")?
            //
            // c if Punctuation::charset().contains(&c) => write!(f, "{}", self::)?,
            // c if Punctuation::charset().contains(&c) => Token::Punctuation(Punctuation::from(value)),
            // c if Operator::charset().contains(&c) => Token::Operator(Operator::from(value, false)),
            // ' ' => Token::Whitespace,
            // 'a'..='z' => Token::Letter(value),
            // 'A'..='Z' => Token::Letter(value),
            // '0'..='9' => Token::Digit(value as u8),
            // _ => Token::Unknown(value)
        }
        Ok(())
    }
}

#[cfg(test)]
mod operator {
    use std::collections::HashSet;
    use crate::structs::tokens::{Associativity, Operator};

    #[test]
    fn test_from() {
        assert_eq!(Operator::from('+', false), Operator::Plus, "Operator for '+' is invalid!");
        assert_eq!(Operator::from('-', false), Operator::Minus, "Operator for '-' is invalid!");
        assert_eq!(Operator::from('*', false), Operator::Multiply, "Operator for '*' is invalid!");
        assert_eq!(Operator::from('*', true), Operator::Deref, "Operator for unary '*' is invalid!");
        assert_eq!(Operator::from('/', false), Operator::Divide, "Operator for '/' is invalid!");
        assert_eq!(Operator::from('^', false), Operator::Exponent, "Operator for '^' is invalid!");
        assert_eq!(Operator::from('-', true), Operator::Negate, "Operator for unary '-' is invalid!");
        assert_eq!(Operator::from('!', false), Operator::Factorial, "Operator for '!' is invalid!");
        assert_eq!(Operator::from('%', false), Operator::Unknown, "Operator for unknown character is invalid!");
    }

    #[test]
    fn test_charset() {
        assert_eq!(Operator::charset(), HashSet::from(['+', '-', '*', '/', '^', '!']), "Operator charset is invalid!");
    }

    #[test]
    fn test_to_char() {
        assert_eq!(Operator::Plus.to_char(), Some('+'), "Operator char for Plus is invalid!");
        assert_eq!(Operator::Minus.to_char(), Some('-'), "Operator char for Minus is invalid!");
        assert_eq!(Operator::Multiply.to_char(), Some('*'), "Operator char for Multiply is invalid!");
        assert_eq!(Operator::Deref.to_char(), Some('*'), "Operator char for Deref is invalid!");
        assert_eq!(Operator::Divide.to_char(), Some('/'), "Operator char for Divide is invalid!");
        assert_eq!(Operator::Exponent.to_char(), Some('^'), "Operator char for Exponent is invalid!");
        assert_eq!(Operator::Negate.to_char(), Some('-'), "Operator char for Negate is invalid!");
        assert_eq!(Operator::Factorial.to_char(), Some('!'), "Operator char for Factorial is invalid!");
        assert_eq!(Operator::Unknown.to_char(), None, "Operator char should be None!");
    }

    #[test]
    fn test_associativity() {
        assert_eq!(Operator::Plus.associativity(), Associativity::Left, "Operator associativity for Plus is invalid!");
        assert_eq!(Operator::Minus.associativity(), Associativity::Left, "Operator associativity for Minus is invalid!");
        assert_eq!(Operator::Multiply.associativity(), Associativity::Left, "Operator associativity for Multiply is invalid!");
        assert_eq!(Operator::Deref.associativity(), Associativity::Right, "Operator associativity for Deref is invalid!");
        assert_eq!(Operator::Divide.associativity(), Associativity::Left, "Operator associativity for Divide is invalid!");
        assert_eq!(Operator::Exponent.associativity(), Associativity::Right, "Operator associativity for Exponent is invalid!");
        assert_eq!(Operator::Negate.associativity(), Associativity::Right, "Operator associativity for Negate is invalid!");
        assert_eq!(Operator::Factorial.associativity(), Associativity::Right, "Operator associativity for Factorial is invalid!");
        assert_eq!(Operator::Unknown.associativity(), Associativity::Left, "Operator associativity for Unknown is invalid!");
    }
}

mod punctuation {
    use std::collections::HashSet;
    use crate::structs::tokens::{Punctuation};

    #[test]
    fn test_from() {
        assert_eq!(Punctuation::from('('), Punctuation::LeftParen, "Punctuation for '(' is invalid!");
        assert_eq!(Punctuation::from(')'), Punctuation::RightParen, "Punctuation for ')' is invalid!");
        assert_eq!(Punctuation::from('%'), Punctuation::Unknown, "Punctuation for invalid character is invalid!");
    }

    #[test]
    fn test_charset() {
        assert_eq!(Punctuation::charset(), HashSet::from(['(', ')']), "Punctuation charset is invalid!");
    }

    #[test]
    fn test_to_char() {
        assert_eq!(Punctuation::LeftParen.to_char(), Some('('), "Punctuation char '(' is invalid!");
        assert_eq!(Punctuation::RightParen.to_char(), Some(')'), "Punctuation char '(' is invalid!");
        assert_eq!(Punctuation::Unknown.to_char(), None, "Punctuation char should be None!");
    }
}

mod token {
    use crate::structs::strings::HeapString;
    use crate::structs::tokens::{Operator, Punctuation, Token};

    #[test]
    fn test_from() {
        assert_eq!(Token::from('a'), Token::Letter('a'), "Token for 'a' is invalid!");
        assert_eq!(Token::from('1'), Token::Digit(1), "Token for '1' is invalid!");
        assert_eq!(Token::from('+'), Token::Operator(Operator::Plus), "Token for '+' is invalid!");
        assert_eq!(Token::from('('), Token::Punctuation(Punctuation::LeftParen), "Token for '(' is invalid!");
        assert_eq!(Token::from(' '), Token::Whitespace, "Token for whitespace is invalid!");
        assert_eq!(Token::from('%'), Token::Unknown('%'), "Token for invalid character is invalid!");
    }

    #[test]
    fn test_tokenize() {
        let str: HeapString = HeapString::from("(a + b)^c");
        let tokens = Token::tokenize(&str);
        assert_eq!(tokens.get_len(), 9, "Tokens length is invalid!");
        assert_eq!(tokens[0], Token::Punctuation(Punctuation::LeftParen), "Token for '(' is invalid!");
        assert_eq!(tokens[1], Token::Letter('a'), "Token for 'a' is invalid!");
        assert_eq!(tokens[2], Token::Whitespace, "Token for ' ' is invalid!");
        assert_eq!(tokens[3], Token::Operator(Operator::Plus), "Token for '+' is invalid!");
        assert_eq!(tokens[4], Token::Whitespace, "Token for ' ' is invalid!");
        assert_eq!(tokens[5], Token::Letter('b'), "Token for 'a' is invalid!");
        assert_eq!(tokens[6], Token::Punctuation(Punctuation::RightParen), "Token for ')' is invalid!");
        assert_eq!(tokens[7], Token::Operator(Operator::Exponent), "Token for '^' is invalid!");
        assert_eq!(tokens[8], Token::Letter('c'), "Token for 'a' is invalid!");
    }
}