use std::ops::{Add, Mul};
use std::fmt;


// TODO: read up on research related to BigNum handling etc.
// TODO: implement Mul for `BigUInt`, probably after pondering the redesign.
// TODO: implement From trait for conversions from u8-u64.
// TODO: ponder whether specific errors are needed.
// TODO: maybe rethink design so that `BigUInt` can implement Copy, i.e. make different versions as wrappers around differing quantities of u64's, e.g. BigU128, BigU256 etc.


/// `BigUInt` is a wrapper around a vector of the digits of a number in reverse order
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BigUInt(Vec<u8>);

impl BigUInt {
    pub fn new() -> Self {
        BigUInt(Vec::new())
    }

    pub fn from_buf(buf: &[u8]) -> Self {
        BigUInt(Vec::from(buf))
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn trim_leading_zeroes(&mut self) {
        while let Some(&0u8) = self.0.iter().last() {
            self.0.pop();
        }
    }
}

/// implementing Add so we can use the + operator for `BigUInt`
impl Add for BigUInt {
    /// Adding the `BigUInt` to another `BigUInt` produces a `BigUInt`.
    type Output = Self;

    /// The method for the + operator
    fn add(self, other: Self) -> Self {
        let mut temp1 = self.0.clone();
        let mut temp2 = other.0.clone();
        let (tuple_list, longest_length) = match (self.size(), other.size()) {
            (i, j) if i < j => {
                temp1.resize(j, 0u8);
                (temp1.iter().zip(other.0), j)
            }
            (i, j) if i > j => {
                temp2.resize(i, 0u8);
                (temp2.iter().zip(self.0), i)
            }
            (i, _) => (self.0.iter().zip(other.0), i),
        };
        let mut result = Vec::with_capacity(longest_length);
        let mut carry = 0u8;

        for (i, j) in tuple_list {
            let temp = i + j + carry;
            result.push(temp % 10u8);
            carry = temp / 10u8;
        }

        BigUInt::from_buf(&result)
    }
}

/// implementing Mul so that we can actually use the `BigUInt` type for arithmetic
impl Mul for BigUInt {
    type Output = Self;

    #[allow(unused_variables)]
    fn mul(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for BigUInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let mut it = self.0.iter().rev().skip_while(|x| **x == 0).peekable();
        if it.peek().is_some() {
            for digit in it {
                result = format!("{}{}", result, digit);
            }
        } else {
            result = "0".to_string();
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        // create a new empty `BigUInt`
        let biguint1 = BigUInt::new();
        let biguint2 = BigUInt::from_buf(&vec![0u8;2]);

        assert_eq!(biguint1, BigUInt(Vec::new()));
        assert_eq!(biguint2, BigUInt(vec![0u8, 0u8]));
    }

    #[test]
    fn addition() {
        let biguint1 = BigUInt::new(); // 0
        let biguint2 = BigUInt::from_buf(&vec![2u8, 1]); // 0+12

        assert_eq!("0", format!("{}", biguint1));
        assert_eq!("12", format!("{}", biguint2));
        assert_eq!("12", format!("{}", biguint1.clone() + biguint2.clone()));
        assert_eq!("24", format!("{}", biguint2.clone() + biguint2.clone()));
    }
}
