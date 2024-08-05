use crate::structs::arrays::HeapArray;
use crate::structs::strings::HeapString;
use num::ToPrimitive;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ptr::write;

#[derive(PartialEq, Debug)]
pub enum Associativity {
    Left,
    Right,
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
    Unknown,
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
            }
            '*' => {
                if unary {
                    Operator::Deref
                } else {
                    Operator::Multiply
                }
            }
            '/' => Operator::Divide,
            '^' => Operator::Exponent,
            '!' => Operator::Factorial,
            _ => Operator::Unknown,
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
            _ => None,
        }
    }

    pub fn associativity(&self) -> Associativity {
        match self {
            Operator::Plus => Associativity::Left,
            Operator::Minus => Associativity::Left,
            Operator::Multiply => Associativity::Left,
            Operator::Divide => Associativity::Left,
            Operator::Exponent => Associativity::Right,
            Operator::Negate => Associativity::Right,
            Operator::Factorial => Associativity::Right,
            Operator::Deref => Associativity::Right,
            Operator::Unknown => Associativity::Left,
        }
    }

    pub fn precedence(&self) -> usize {
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

    pub fn evaluate(&self, x1: u32, x2: u32) -> u32 {
        // NOTE: Maybe later you would have to do byte operations to simulate arithmetic operations from scratch
        match self {
            Self::Plus => x1 + x2,
            Self::Minus => x1 - x2,
            Self::Multiply => x1 * x2,
            Self::Divide => x1 / x2,
            Self::Exponent => x1.pow(x2),
            _ => 0u32,
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
    Unknown,
}

impl From<char> for Punctuation {
    fn from(value: char) -> Self {
        match value {
            '(' => Punctuation::LeftParen,
            ')' => Punctuation::RightParen,
            _ => Punctuation::Unknown,
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
            _ => None,
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
    Number(u32), //TODO: Figure out if it should be i32 and how to handle negative values?
    Operator(Operator),
    Punctuation(Punctuation),
    Whitespace,
    Unknown(char),
}

impl Token {
    pub fn tokenize(expression: &HeapString) -> HeapArray<Self> {
        let mut tokens: HeapArray<Self> = HeapArray::with_capacity(expression.len());
        for &c in expression.iter() {
            tokens.push(Self::from(c as char));
        }
        tokens
    }

    pub fn to_char(&self) -> Option<char> {
        match self {
            Self::Letter(c) => Some(*c),
            Self::Digit(d) => Some((d + b'0') as char),
            Self::Operator(op) => op.to_char(),
            Self::Punctuation(punc) => punc.to_char(),
            Self::Whitespace => Some(' '),
            _ => None,
        }
    }

    pub fn precedence(&self) -> usize {
        match self {
            Self::Letter(c) => 100,
            Self::Digit(d) => 100,
            Self::Operator(op) => op.precedence(),
            Self::Punctuation(punc) => punc.precedence(),
            Self::Whitespace => 100,
            _ => 100,
        }
    }

    pub fn evaluate(x1: Token, x2: Token, operator: Token) -> Token {
        let out = match operator {
            Self::Operator(op) => {
                let x1_val: u32 = match x1 {
                    Self::Digit(val) => val as u32,
                    Self::Number(val) => val,
                    _ => 0u32,
                };
                let x2_val: u32 = match x2 {
                    Self::Digit(val) => val as u32,
                    Self::Number(val) => val,
                    _ => 0u32,
                };
                op.evaluate(x1_val, x2_val)
            }
            _ => 0u32,
        };
        Self::Number(out)
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            c if Punctuation::charset().contains(&c) => Self::Punctuation(Punctuation::from(value)),
            c if Operator::charset().contains(&c) => Self::Operator(Operator::from(value, false)),
            ' ' => Self::Whitespace,
            'a'..='z' => Self::Letter(value),
            'A'..='Z' => Self::Letter(value),
            '0'..='9' => Self::Digit(value.to_digit(10).unwrap() as u8),
            c if c.is_digit(10) => Self::Number(c as u32),
            _ => Self::Unknown(value),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Punctuation(val) => write!(f, "{}", val.to_char().unwrap())?,
            _ => write!(f, "None")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod operator {
    use crate::structs::tokens::{Associativity, Operator};
    use std::collections::HashSet;

    #[test]
    fn test_from() {
        assert_eq!(
            Operator::from('+', false),
            Operator::Plus,
            "Operator for '+' is invalid!"
        );
        assert_eq!(
            Operator::from('-', false),
            Operator::Minus,
            "Operator for '-' is invalid!"
        );
        assert_eq!(
            Operator::from('*', false),
            Operator::Multiply,
            "Operator for '*' is invalid!"
        );
        assert_eq!(
            Operator::from('*', true),
            Operator::Deref,
            "Operator for unary '*' is invalid!"
        );
        assert_eq!(
            Operator::from('/', false),
            Operator::Divide,
            "Operator for '/' is invalid!"
        );
        assert_eq!(
            Operator::from('^', false),
            Operator::Exponent,
            "Operator for '^' is invalid!"
        );
        assert_eq!(
            Operator::from('-', true),
            Operator::Negate,
            "Operator for unary '-' is invalid!"
        );
        assert_eq!(
            Operator::from('!', false),
            Operator::Factorial,
            "Operator for '!' is invalid!"
        );
        assert_eq!(
            Operator::from('%', false),
            Operator::Unknown,
            "Operator for unknown character is invalid!"
        );
    }

    #[test]
    fn test_charset() {
        assert_eq!(
            Operator::charset(),
            HashSet::from(['+', '-', '*', '/', '^', '!']),
            "Operator charset is invalid!"
        );
    }

    #[test]
    fn test_to_char() {
        assert_eq!(
            Operator::Plus.to_char(),
            Some('+'),
            "Operator char for Plus is invalid!"
        );
        assert_eq!(
            Operator::Minus.to_char(),
            Some('-'),
            "Operator char for Minus is invalid!"
        );
        assert_eq!(
            Operator::Multiply.to_char(),
            Some('*'),
            "Operator char for Multiply is invalid!"
        );
        assert_eq!(
            Operator::Deref.to_char(),
            Some('*'),
            "Operator char for Deref is invalid!"
        );
        assert_eq!(
            Operator::Divide.to_char(),
            Some('/'),
            "Operator char for Divide is invalid!"
        );
        assert_eq!(
            Operator::Exponent.to_char(),
            Some('^'),
            "Operator char for Exponent is invalid!"
        );
        assert_eq!(
            Operator::Negate.to_char(),
            Some('-'),
            "Operator char for Negate is invalid!"
        );
        assert_eq!(
            Operator::Factorial.to_char(),
            Some('!'),
            "Operator char for Factorial is invalid!"
        );
        assert_eq!(
            Operator::Unknown.to_char(),
            None,
            "Operator char should be None!"
        );
    }

    #[test]
    fn test_associativity() {
        assert_eq!(
            Operator::Plus.associativity(),
            Associativity::Left,
            "Operator associativity for Plus is invalid!"
        );
        assert_eq!(
            Operator::Minus.associativity(),
            Associativity::Left,
            "Operator associativity for Minus is invalid!"
        );
        assert_eq!(
            Operator::Multiply.associativity(),
            Associativity::Left,
            "Operator associativity for Multiply is invalid!"
        );
        assert_eq!(
            Operator::Deref.associativity(),
            Associativity::Right,
            "Operator associativity for Deref is invalid!"
        );
        assert_eq!(
            Operator::Divide.associativity(),
            Associativity::Left,
            "Operator associativity for Divide is invalid!"
        );
        assert_eq!(
            Operator::Exponent.associativity(),
            Associativity::Right,
            "Operator associativity for Exponent is invalid!"
        );
        assert_eq!(
            Operator::Negate.associativity(),
            Associativity::Right,
            "Operator associativity for Negate is invalid!"
        );
        assert_eq!(
            Operator::Factorial.associativity(),
            Associativity::Right,
            "Operator associativity for Factorial is invalid!"
        );
        assert_eq!(
            Operator::Unknown.associativity(),
            Associativity::Left,
            "Operator associativity for Unknown is invalid!"
        );
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(
            Operator::Plus.evaluate(10, 2),
            12,
            "Operator operation for Plus is invalid!"
        );
        assert_eq!(
            Operator::Minus.evaluate(10, 2),
            8,
            "Operator operation for Minus is invalid!"
        );
        assert_eq!(
            Operator::Multiply.evaluate(10, 2),
            20,
            "Operator operation for Multiply is invalid!"
        );
        assert_eq!(
            Operator::Divide.evaluate(10, 2),
            5,
            "Operator operation for Divide is invalid!"
        );
        assert_eq!(
            Operator::Exponent.evaluate(10, 2),
            100,
            "Operator operation for Exponent is invalid!"
        );
    }
}

mod punctuation {
    use crate::structs::tokens::Punctuation;
    use std::collections::HashSet;

    #[test]
    fn test_from() {
        assert_eq!(
            Punctuation::from('('),
            Punctuation::LeftParen,
            "Punctuation for '(' is invalid!"
        );
        assert_eq!(
            Punctuation::from(')'),
            Punctuation::RightParen,
            "Punctuation for ')' is invalid!"
        );
        assert_eq!(
            Punctuation::from('%'),
            Punctuation::Unknown,
            "Punctuation for invalid character is invalid!"
        );
    }

    #[test]
    fn test_charset() {
        assert_eq!(
            Punctuation::charset(),
            HashSet::from(['(', ')']),
            "Punctuation charset is invalid!"
        );
    }

    #[test]
    fn test_to_char() {
        assert_eq!(
            Punctuation::LeftParen.to_char(),
            Some('('),
            "Punctuation char '(' is invalid!"
        );
        assert_eq!(
            Punctuation::RightParen.to_char(),
            Some(')'),
            "Punctuation char '(' is invalid!"
        );
        assert_eq!(
            Punctuation::Unknown.to_char(),
            None,
            "Punctuation char should be None!"
        );
    }
}

mod token {
    use crate::structs::strings::HeapString;
    use crate::structs::tokens::{Operator, Punctuation, Token};

    #[test]
    fn test_from() {
        assert_eq!(
            Token::from('a'),
            Token::Letter('a'),
            "Token for 'a' is invalid!"
        );
        assert_eq!(
            Token::from('1'),
            Token::Digit(1),
            "Token for '1' is invalid!"
        );
        assert_eq!(
            Token::from('+'),
            Token::Operator(Operator::Plus),
            "Token for '+' is invalid!"
        );
        assert_eq!(
            Token::from('('),
            Token::Punctuation(Punctuation::LeftParen),
            "Token for '(' is invalid!"
        );
        assert_eq!(
            Token::from(' '),
            Token::Whitespace,
            "Token for whitespace is invalid!"
        );
        assert_eq!(
            Token::from('%'),
            Token::Unknown('%'),
            "Token for invalid character is invalid!"
        );
    }

    #[test]
    fn test_tokenize() {
        let str: HeapString = HeapString::from("(a + b)^c");
        let tokens = Token::tokenize(&str);
        assert_eq!(tokens.get_len(), 9, "Tokens length is invalid!");
        assert_eq!(
            tokens[0],
            Token::Punctuation(Punctuation::LeftParen),
            "Token for '(' is invalid!"
        );
        assert_eq!(tokens[1], Token::Letter('a'), "Token for 'a' is invalid!");
        assert_eq!(tokens[2], Token::Whitespace, "Token for ' ' is invalid!");
        assert_eq!(
            tokens[3],
            Token::Operator(Operator::Plus),
            "Token for '+' is invalid!"
        );
        assert_eq!(tokens[4], Token::Whitespace, "Token for ' ' is invalid!");
        assert_eq!(tokens[5], Token::Letter('b'), "Token for 'a' is invalid!");
        assert_eq!(
            tokens[6],
            Token::Punctuation(Punctuation::RightParen),
            "Token for ')' is invalid!"
        );
        assert_eq!(
            tokens[7],
            Token::Operator(Operator::Exponent),
            "Token for '^' is invalid!"
        );
        assert_eq!(tokens[8], Token::Letter('c'), "Token for 'a' is invalid!");
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(
            Token::evaluate(
                Token::Digit(5),
                Token::Digit(1),
                Token::Operator(Operator::Plus)
            ),
            Token::Number(6),
            "Token addition evaluation for Digit is invalid"
        );
        assert_eq!(
            Token::evaluate(
                Token::Number(55),
                Token::Number(10),
                Token::Operator(Operator::Plus)
            ),
            Token::Number(65),
            "Token addition evaluation for Number is invalid"
        );
        assert_eq!(
            Token::evaluate(
                Token::Letter('a'),
                Token::Letter('b'),
                Token::Operator(Operator::Plus)
            ),
            Token::Number(0),
            "Token addition evaluation for unknown is invalid"
        );
    }
}
