extern crate num;
extern crate rand;

use std::fmt;
use rand::{thread_rng, Rng};

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

/*
fn cnot(control: &QBit, value: &QBit) -> QBit {
    if (control)
}
*/

fn main() {
    let mut rng = thread_rng();
    let b = QBit {
        a: num::complex::Complex::new(0.5, 0.0),
        b: num::complex::Complex::new(0.5, 0.0)
    };
    println!("QBit {}, collapses to CBit {}", b, b.collapse(&mut rng))
}
