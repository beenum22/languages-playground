use std::path::Prefix;
use crate::structs::arrays::HeapArray;
use crate::structs::stacks::Stack;
use crate::structs::strings::HeapString;
use crate::structs::tokens::{Token, Associativity, Punctuation};

struct Expression {
    prefix: Option<HeapString>,
    postfix: Option<HeapString>,
    infix: HeapString,
}

impl Expression {
    fn new(string: &str) -> Self {
        Self {
            prefix: None,
            postfix: None,
            infix: HeapString::from(string),
        }
    }

    fn infix_to_postfix(&mut self) {
        let mut stack: Stack<Token> = Stack::new(self.infix.len());
        let tokens: HeapArray<Token> = Token::tokenize(&self.infix);
        let mut postfix: HeapString = HeapString::with_capacity(self.infix.len());
        for &token in &tokens {
            match token {
                Token::Punctuation(punc) => {
                    match punc {
                        Punctuation::LeftParen => stack.push(token),
                        Punctuation::RightParen => {
                            while *stack.peek().unwrap() != Token::Punctuation(Punctuation::LeftParen) {
                                postfix.push(stack.pop().unwrap().to_char().unwrap());
                            }
                            stack.pop();
                        }
                        _ => {}
                    }
                }
                (Token::Letter(_)) => postfix.push(token.to_char().unwrap()),
                (Token::Digit(_)) => postfix.push(token.to_char().unwrap()),
                Token::Whitespace => postfix.push(token.to_char().unwrap()),
                Token::Operator(operator) => {
                    while !stack.is_empty() && operator.precedence() <= stack.peek().unwrap().precedence() {
                        if operator.precedence() == stack.peek().unwrap().precedence() &&
                            operator.associativity() == Associativity::Right {
                            postfix.push(operator.to_char().unwrap());
                            break
                        } else {
                            postfix.push(stack.pop().unwrap().to_char().unwrap());
                        }
                    }
                    match stack.peek() {
                        Some(val) => {
                            match operator.precedence() == val.precedence() && operator.associativity() == Associativity::Right {
                                false => stack.push(token),
                                true => (),
                            }
                        },
                        None => stack.push(token)
                    }
                }
                _ => {}
            }
        }
        while !stack.is_empty() {
            postfix.push(stack.pop().unwrap().to_char().unwrap());
        }
        self.postfix = Some(postfix);
    }

    fn get_prefix(&mut self) -> &str {
        if self.prefix.is_none() {
            self.prefix = Some(HeapString::new());
        }
        self.prefix.as_ref().unwrap().as_str()
    }

    fn get_postfix(&mut self) -> &str {
        if self.postfix.is_none() {
            self.infix_to_postfix()
        }
        self.postfix.as_ref().unwrap().as_str()
    }
}

#[cfg(test)]
mod expression {
    use crate::structs::expressions::Expression;
    use crate::structs::strings::HeapString;

    #[test]
    fn test_new() {
        let exp = Expression::new("a + b");
        assert_eq!(exp.prefix, None, "Prefix representation must be None initially!");
        assert_eq!(exp.postfix, None, "Postfix representation must be None initially!");
        assert_eq!(exp.infix, HeapString::from("a + b"), "Infix representation is invalid!");
    }

    // #[test]
    fn test_get_prefix() {
        let exp = Expression::new("a + b");
        assert_eq!(Expression::new("a + b").get_prefix(), "+ab".to_string(), "Prefix representation is invalid for the first expression!");
        assert_eq!(Expression::new("(a + b) * (a - b)").get_prefix(), "*+ab-ab".to_string(), "Postfix representation is invalid for the second expression!");
    }

    #[test]
    fn test_get_postfix() {
        assert_eq!(Expression::new("a+b").get_postfix(), "ab+".to_string(), "Postfix representation is invalid for the 'a+b' expression!");
        assert_eq!(Expression::new("a + b").get_postfix(), "a  b+".to_string(), "Postfix representation is invalid for the 'a + b' expression!");
        assert_eq!(Expression::new("a+b*a-b").get_postfix(), "aba*+b-".to_string(), "Postfix representation is invalid for the 'a+b*a-b' expression!");
        assert_eq!(Expression::new("a + b * a - b").get_postfix(), "a  b  a *+ b-".to_string(), "Postfix representation is invalid for the 'a + b * a - b' expression!");
        assert_eq!(Expression::new("a+b^c^d+e").get_postfix(), "abc^d^+e+".to_string(), "Postfix representation is invalid for the 'a+b^c^d+e' expression!");
        assert_eq!(Expression::new("(a+b)*(a-b)").get_postfix(), "ab+ab-*".to_string(), "Postfix representation is invalid for the '(a+b)*(a-b)' expression!");
        assert_eq!(Expression::new("(a + b) * (a - b)").get_postfix(), "a  b+  a  b-*".to_string(), "Postfix representation is invalid for the '(a + b) * (a - b)' expression!");
        assert_eq!(Expression::new("a^x+b^x+x*a*b").get_postfix(), "ax^bx^+xa*b*+".to_string(), "Postfix representation is invalid for the 'a^x+b^x+x*a*b' expression!");
        // assert_eq!(Expression::new("a^2+b^2+2*a*b").get_postfix(), "a2^b2^+2a*b*+".to_string(), "Postfix representation is invalid for the 'a^2+b^2+2*a*b' expression!");
        // assert_eq!(Expression::new("(a + b) * (a - b)").get_postfix(), "a  b+  a  b-*".to_string(), "Postfix representation is invalid for the '(a + b) * (a - b)' expression!");
    }
}
