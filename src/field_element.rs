/// Finite field element modulo a prime
#[derive(Clone, Debug)]
pub struct FieldElement {
    pub value: u128,
    pub prime: u128,
}

impl FieldElement {
    pub fn new(value: u128, prime: u128) -> Self {
        Self {
            value: value % prime,
            prime,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.prime, other.prime, "Mismatched primes!");
        let sum = (self.value + other.value) % self.prime;
        FieldElement {
            value: sum,
            prime: self.prime,
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.prime, other.prime, "Mismatched primes!");
        let product = (self.value * other.value) % self.prime;
        FieldElement {
            value: product,
            prime: self.prime,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.prime, other.prime, "Mismatched primes!");
        let diff = (self.value + self.prime - other.value) % self.prime;
        FieldElement {
            value: diff,
            prime: self.prime,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        assert_eq!(self.prime, other.prime, "Mismatched primes!");
        self.value == other.value
    }
}
