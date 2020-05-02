extern crate num;
extern crate rand;

use std::fmt;
use rand::{thread_rng, Rng};

struct Qbit {
    a: num::complex::Complex64,
    b: num::complex::Complex64
}

impl Qbit {
    fn collapse(self: &Self, rng: &mut rand::rngs::ThreadRng) -> Qbit {
        // P(bit == (1, 0)) = ||a||^2
        let is_zero: bool = rng.gen_bool(self.a.norm().powi(2));
        if is_zero {
            return Qbit {
                a: num::complex::Complex::new(1.0, 0.0),
                b: num::complex::Complex::new(0.0, 0.0)
            }
        }
        return Qbit {
            a: num::complex::Complex::new(0.0, 0.0),
            b: num::complex::Complex::new(1.0, 0.0)
        }
    }
}

impl fmt::Display for Qbit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

/*
fn cnot(control: &Qbit, value: &Qbit) -> Qbit {
    if (control)
}
*/

fn main() {
    let mut rng = thread_rng();
    let b = Qbit {
        a: num::complex::Complex::new(0.5, 0.0),
        b: num::complex::Complex::new(0.5, 0.0)
    };
    println!("QBit {}, collapses to CBit {}", b, b.collapse(&mut rng))
}
