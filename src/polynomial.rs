/// Represents a polynomial over integers, coefficients low-to-high degree.
#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<i64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<i64>) -> Self {
        Polynomial { coefficients }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len().saturating_sub(1)
    }

    /// Evaluate polynomial at x, mod prime.
    pub fn eval(&self, x: i64, prime: i64) -> i64 {
        let mut result: i64 = 0;
        let mut power: i64 = 1;
        for &coef in &self.coefficients {
            result = (result + coef * power).rem_euclid(prime);
            power = (power * x).rem_euclid(prime);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_poly_evals_to_itself() {
        let p = Polynomial::new(vec![7]);
        assert_eq!(p.eval(5, 101), 7);
    }
}