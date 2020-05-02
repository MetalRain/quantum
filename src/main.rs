extern crate num;
extern crate rand;

use std::fmt;
use rand::{thread_rng, Rng};
use rand::distributions::{WeightedIndex, Distribution};

#[derive(Debug)]
struct QBit {
    // QBit (1, 0) is CBit 0
    // QBit (0, 1) is CBit 1
    a: num::complex::Complex64,
    b: num::complex::Complex64
}

impl QBit {
    fn collapse(self: &Self, rng: &mut rand::rngs::ThreadRng) -> CBit {
        // Intuition, measure QBit get beck CBit
        // P(QBit (a, b) == CBit 0) = ||a||^2
        let is_zero: bool = rng.gen_bool(self.a.norm().powi(2));
        if is_zero {
            return CBit::Zero
        }
        return CBit::One
    }
}

impl fmt::Display for QBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

#[derive(Debug)]
struct QBitVector {
    size: usize,
    components: Vec<num::complex::Complex64>
}

impl QBitVector {
    fn collapse(self: &Self, rng: &mut rand::rngs::ThreadRng) -> CBitVector {
        // Intuition, only one can be true
        let component_probabilities: Vec<f64> = self.components.iter()
            .map(|c| c.norm().powi(2))
            .collect();
        let distribution: WeightedIndex<f64> = WeightedIndex::new(&component_probabilities).unwrap();
        let selected_component_index = distribution.sample(rng);
        
        let mut selected_bit_index = selected_component_index / 2; // Even
        if selected_component_index % 2 == 1 { // Odd
            let selected_bit_index = (selected_component_index - 1) / 2;
        }

        let mut result = Vec::with_capacity(self.size);
        result.resize(self.size, CBit::Zero);
        result[selected_bit_index] = CBit::One;
        return CBitVector { size: self.size, bits: result }
    }
}

#[derive(Clone, Debug)]
enum CBit {
    Zero,
    One
}

impl fmt::Display for CBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBit::Zero => write!(f, "0"),
            CBit::One => write!(f, "1")
        }
    }
}

#[derive(Debug)]
struct CBitVector {
    size: usize,
    bits: Vec<CBit>
}

/*
fn cnot(control: &QBit, value: &QBit) -> QBit {
    if (control)
}
*/

fn main() {
    let mut rng = thread_rng();

    let coin = QBit {
        a: num::complex::Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
        b: num::complex::Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)
    };
    println!("QBit {}, collapses to CBit {}", coin, coin.collapse(&mut rng));

    let coins = QBitVector {
        size: 2,
        components: vec![
            num::complex::Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            num::complex::Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            num::complex::Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            num::complex::Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)
        ]
    };

    println!("QBitVector {:#?}, collapses to CBitVector {:#?}", coins, coins.collapse(&mut rng))
}
