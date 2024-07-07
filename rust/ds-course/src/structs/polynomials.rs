use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use crate::structs::arrays::HeapArray;

#[derive(Debug)]
struct Term {
    coefficient: i32,
    exponent: u32
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}x^{}", self.coefficient, self.exponent)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Polynomial {
    count: usize,
    terms: HeapArray<Term>
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.count {
            if i > 0 && self.terms[i].coefficient.is_positive() == true {
                write!(f, "+")?;
            }
            write!(f, "{}", self.terms[i])?;
        }
        Ok(())
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            return false
        }
        for i in 0..self.count {
            if self.terms[i].coefficient != other.terms[i].coefficient || self.terms[i].exponent != other.terms[i].exponent {
                return false
            }
        }
        return true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Polynomial {
    pub fn new(count: usize) -> Self {
        let terms = HeapArray::with_capacity(count);
        Polynomial {
            count,
            terms
        }
    }

    pub fn set_term(&mut self, coefficient: i32, exponent: u32) {
        if self.terms.get_len() == self.terms.get_size() {
            self.terms.resize(self.terms.get_len() + 1).expect("Failed to increase the Polynomial terms count!");
            self.count += 1
        }
        self.terms.push(
            Term {
                coefficient,
                exponent
            }
        );
    }

    pub fn evaluate(&self, x: i32) -> i32 {
        let mut res = 0;
        for i in 0..self.count {
            res += self.terms[i].coefficient * x.pow(self.terms[i].exponent)
        }
        return res
    }

    pub fn add(&mut self, other: Self) {
        let mut res = Self {
            count: self.count + other.count,
            terms: HeapArray::with_capacity(self.count + other.count),
        };

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;

        while i < self.count && j < other.count {
            if self.terms[i].exponent == other.terms[j].exponent {
                res.set_term(self.terms[i].coefficient + other.terms[j].coefficient, self.terms[i].exponent);
                i += 1;
                j += 1;
                k += 1;
            } else if self.terms[i].exponent > other.terms[i].exponent {
                res.set_term(self.terms[i].coefficient, self.terms[i].exponent);
                i += 1;
                k += 1;
            } else {
                res.set_term(other.terms[j].coefficient, other.terms[j].exponent);
                j += 1;
                k += 1;
            }
        }

        self.count = res.terms.get_len();
        self.terms = res.terms;
        self.terms.resize(self.count).expect("Failed to resize the Polynomial terms array!");
    }
}

mod polynomial {
    use super::*;

    #[test]
    fn test_new() {
        let count: usize = 5;
        let poly: Polynomial = Polynomial::new(count);
        assert_eq!(poly.count, 5, "Invalid polynomial terms count!");
        assert_eq!(poly.terms.get_len(), 0, "Invalid Polynomial terms array length. It should be 0!");
        assert_eq!(poly.terms.get_size(), count, "Invalid Polynomial terms array size. It should be set to the terms count!");
    }

    #[test]
    fn test_set_term() {
        let mut poly: Polynomial = Polynomial::new(2);
        poly.set_term(4, 2);
        poly.set_term(1, 1);
        assert_eq!(poly.terms.get_len(), 2, "Invalid Polynomial terms array length. It should be 2!");
        poly.set_term(2, 0);
        assert_eq!(poly.terms.get_len(), 3, "Failed to extend the Polynomial length!");
        assert_eq!(poly.count, 3, "Failed to update the Polynomial count!");
    }

    #[test]
    fn test_evaluate() {
        // p(x) = 3x^2 + 4x^1 + 3x^0
        let x = 2;
        let mut poly: Polynomial = Polynomial::new(3);
        poly.set_term(3, 2);
        poly.set_term(4, 1);
        poly.set_term(3, 0);
        assert_eq!(poly.evaluate(x), 23, "Invalid Polynomial evaluation. Outcome should be 2!");
    }

    #[test]
    fn test_add() {
        // p1(x) = 3x^2 + 4x^1 + 3x^0
        // p2(x) = 4x^3 + 2x^2 + 5x^1 + 2x^0
        // res(x) = 3x^2 + 4x^1 + 3x^0 + 4x^3 + 2x^2 + 5x^1 + 2x^0 = 4x^3 + 5x^2 + 9x^1 + 5
        let x = 2;
        let mut p1: Polynomial = Polynomial::new(3);
        let mut p2: Polynomial = Polynomial::new(4);
        let mut res: Polynomial = Polynomial::new(4);

        p1.set_term(3, 2);
        p1.set_term(4, 1);
        p1.set_term(3, 0);

        p2.set_term(4, 3);
        p2.set_term(2, 2);
        p2.set_term(5, 1);
        p2.set_term(2, 0);

        res.set_term(4, 3);
        res.set_term(5, 2);
        res.set_term(9, 1);
        res.set_term(5, 0);

        p1.add(p2);

        assert_eq!(p1.count, 4, "Invalid Polynomial count after addition. Outcome should be 4!");
        assert_eq!(p1.terms.get_len(), 4, "Invalid Polynomial terms array length after addition. Outcome should be 4!");
        assert_eq!(p1.terms.get_size(), 4, "Invalid Polynomial terms array size after addition. Outcome should be 4!");
        assert_eq!(p1, res, "Invalid Polynomial addition!");
    }
}
