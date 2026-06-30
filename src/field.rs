/// A finite field element mod a large prime, with proper operator overloads.
/// This replaces raw i64 modular arithmetic scattered across the codebase.
use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    pub value: i128,
    pub modulus: i128,
}

impl FieldElement {
    pub fn new(value: i128, modulus: i128) -> Self {
        FieldElement { value: value.rem_euclid(modulus), modulus }
    }

    pub fn zero(modulus: i128) -> Self {
        FieldElement::new(0, modulus)
    }

    pub fn one(modulus: i128) -> Self {
        FieldElement::new(1, modulus)
    }

    pub fn pow(&self, mut exp: u64) -> FieldElement {
        let mut base = *self;
        let mut result = FieldElement::one(self.modulus);
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    /// Fermat's little theorem inverse (requires prime modulus).
    pub fn inverse(&self) -> FieldElement {
        self.pow((self.modulus - 2) as u64)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> FieldElement {
        FieldElement::new(self.value + rhs.value, self.modulus)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> FieldElement {
        FieldElement::new(self.value - rhs.value, self.modulus)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> FieldElement {
        FieldElement::new(self.value * rhs.value, self.modulus)
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        FieldElement::new(-self.value, self.modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_multiplies_to_one() {
        let p = 2147483647; // Mersenne prime
        let a = FieldElement::new(12345, p);
        let inv = a.inverse();
        assert_eq!((a * inv).value, 1);
    }
}