extern crate num;
extern crate rand;
extern crate nalgebra as na;

use std::fmt;
use rand::{thread_rng, Rng};
use rand::distributions::{WeightedIndex, Distribution};
use na::{U1, U2, Dynamic, Matrix, Vector2, VecStorage};

macro_rules! one_sqrt_two {
    () => {
        1.0 / 2.0_f64.sqrt()
    }
}

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

type QBitVectorComponents = Matrix<num::complex::Complex64, U2, Dynamic, VecStorage<num::complex::Complex64, U2, Dynamic>>;

#[derive(Debug)]
struct QBitVector {
    components: QBitVectorComponents
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
            selected_bit_index = (selected_component_index - 1) / 2;
        }

        let size = self.components.nrows();
        let mut result: Vec<CBit> = Vec::with_capacity(size);
        result.resize(self.components.nrows(), CBit::Zero);
        result[selected_bit_index] = CBit::One;
        return CBitVector {
            bits: CBitVectorBits::from_iterator(size, result.into_iter()) }
    }

    
    fn hadamard(self: &Self) -> QBitVector {
        let hadamard_matrix = QBitVectorComponents::from_columns(&[
            Vector2::new(cplx(one_sqrt_two!()), cplx(one_sqrt_two!())),
            Vector2::new(cplx(one_sqrt_two!()), cplx(-one_sqrt_two!()))
        ]);
        return QBitVector {
            components: hadamard_matrix * self.components.clone()
        }
    }
}

#[derive(Copy, Clone, Debug, Eq)]
enum CBit {
    Zero,
    One
}

impl PartialEq for CBit {
    fn eq(self: &Self, other: &Self) -> bool{
        match (self, other) {
            (CBit::Zero, CBit::Zero) => true,
            (CBit::One, CBit::One) => true,
            _ => false
        }
    }
}

impl fmt::Display for CBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBit::Zero => write!(f, "0"),
            CBit::One => write!(f, "1")
        }
    }
}

type CBitVectorBits = Matrix::<CBit, U1, Dynamic, VecStorage<CBit, U1, Dynamic>>;

#[derive(Debug)]
struct CBitVector {
    bits: CBitVectorBits
}

fn cplx(real: f64) -> num::complex::Complex64 {
    num::complex::Complex::new(real, 0.0)
} 

/*
fn cnot(control: &QBit, value: &QBit) -> QBit {
    if (control)
}
*/

fn main() {
    let mut rng = thread_rng();

    let coin = QBit {
        a: cplx(one_sqrt_two!()),
        b: cplx(one_sqrt_two!())
    };
    println!("QBit {}, collapses to CBit {}", coin, coin.collapse(&mut rng));

    let pair = QBitVector {
        components: QBitVectorComponents::from_columns(&[
            Vector2::new(cplx(one_sqrt_two!()), cplx(one_sqrt_two!())),
            Vector2::new(cplx(one_sqrt_two!()), cplx(one_sqrt_two!()))
        ])
    };

    println!("QBitVector {:#?}, collapses to CBitVector {:#?}", pair, pair.collapse(&mut rng));

    let classical = QBitVector {
        components: QBitVectorComponents::from_columns(&[
            Vector2::new(cplx(1.0), cplx(0.0)),
            Vector2::new(cplx(0.0), cplx(1.0)),
            Vector2::new(cplx(one_sqrt_two!()), cplx(one_sqrt_two!())),
        ])
    };

    println!("QBitVector {:#?}, after hadamard {:#?}", classical, classical.hadamard());
}
