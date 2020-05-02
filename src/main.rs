extern crate num;
extern crate rand;
extern crate nalgebra as na;

use std::fmt;
use rand::{thread_rng, Rng};
use na::{Vector2, Matrix2, Vector4, Matrix4};
use na::linalg::{SVD};

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
        // Intuition, measure QBit get back CBit
        let probability = self.a.norm().powi(2);
        let is_zero: bool = rng.gen_bool(probability);
        if is_zero {
            return CBit::Zero
        }
        return CBit::One
    }

    fn bit_flip(self: &Self) -> QBit {
        return QBit { a: self.b, b: self.a }
    }
    
    fn hadamard(self: &Self) -> QBit {
        let c = cplx(one_sqrt_two!());
        return QBit {
            a: c * self.a + c * self.b,
            b: c * self.a - c * self.b
        }
    }
}

impl fmt::Display for QBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
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
            CBit::Zero => write!(f, "|0>"),
            CBit::One => write!(f, "|1>")
        }
    }
}

fn tensor_product(a: &QBit, b: &QBit) -> Vector4<num::complex::Complex64> {
    // Kronecker product (a.a, a.b) * (b.a, b.b)
    Vector4::new(
        a.a * b.a,
        a.a * b.b,
        a.b * b.a,
        a.b * b.b
    ) 
}

fn cnot(inputs: Vector4<num::complex::Complex64>) -> Vector4<num::complex::Complex64> {
    let matrix = Matrix4::new(
        cplx(1.0), cplx(0.0), cplx(0.0), cplx(0.0),
        cplx(0.0), cplx(1.0), cplx(0.0), cplx(0.0),
        cplx(0.0), cplx(0.0), cplx(0.0), cplx(1.0),
        cplx(0.0), cplx(0.0), cplx(1.0), cplx(0.0),
    );
    matrix * inputs
}

fn tensor_deproduct(inputs: Vector4<num::complex::Complex64>) -> (QBit, QBit) {
    // FIXME: Figure out how to reverse vector product
    let m = Matrix2::new(inputs[0], inputs[1], inputs[2], inputs[3]);
    // a * b = M = u * S * v_t =>
    // a = u
    // b = S * v_t
    //println!("M = {:#?}", m);
    let eps = 1e-9_f64;
    let svd = SVD::try_new(m, true, true, eps, 10).unwrap();
    //println!("SVD = {:#?}", svd);
    let c = Vector2::new(cplx(0.0), cplx(1.0));
    //println!("c = {:#?}", c);
    let result = svd.solve(&c, eps);
    let a = svd.u.unwrap();
    let b = Matrix2::from_diagonal(
        &Vector2::new(cplx(svd.singular_values[0]), cplx(svd.singular_values[1]))
    ) * svd.v_t.unwrap();
    (
        QBit {
            a: a[0],
            b: a[1]
        },
        QBit {
            a: b[0],
            b: b[1]
        }
    )
}


fn main() {
    test_op(op_constant_0, CBit::One, CBit::One, "op_constant_0");
    test_op(op_constant_1, CBit::One, CBit::One, "op_constant_1");
    test_op(op_identity, CBit::Zero, CBit::One, "op_identity");
    test_op(op_negation, CBit::Zero, CBit::One, "op_negation");
}

fn test_op(op: fn(QBit, QBit) -> (QBit, QBit), expect_output: CBit, expect_input: CBit, op_name: &str) -> () {
    let mut rng = thread_rng();

    let output = qbit_0()
        .bit_flip()
        .hadamard();
    let input = qbit_0()
        .bit_flip()
        .hadamard();

    let (output_after, input_after) = op(output, input);

    let output_normalized = output_after.hadamard();
    let output_result = output_normalized.collapse(&mut rng);

    let input_normalized = input_after.hadamard();
    let input_result = input_normalized.collapse(&mut rng);
    if output_result != expect_output {
        panic!("{}: output {} (normalized {}) did not match expectation {}", op_name, output_result, output_normalized, expect_output);
    }
    if input_result != expect_input {
        panic!("{}: input {} (normalized {}) did not match expectation {}", op_name, input_result, input_normalized, expect_input);
    }
}


fn op_constant_0(output: QBit, input: QBit) -> (QBit, QBit) {
    (output, input)
}

fn op_constant_1(output: QBit, input: QBit) -> (QBit, QBit) {
    (output.bit_flip(), input)
}

fn op_identity(output: QBit, input: QBit) -> (QBit, QBit) {
    let v = tensor_product(&input, &output);
    let v_result = cnot(v);
    let (input_result, output_result) = tensor_deproduct(v_result);
    println!("in: {}, out: {}", input_result, output_result);
    (output_result, input_result)
}

fn op_negation(output: QBit, input: QBit) -> (QBit, QBit) {
    let v = tensor_product(&input, &output);
    let v_result = cnot(v);
    let (input_result, output_result) = tensor_deproduct(v_result);
    println!("in: {}, out: {}", input_result, output_result);
    (output_result.bit_flip(), input_result)
}


/* Helper functions */


// TODO: Use na::Complex instead
fn cplx(real: f64) -> num::complex::Complex64 {
    num::complex::Complex::new(real, 0.0)
}

fn qbit_0() -> QBit {
    QBit {a: cplx(1.0), b: cplx(0.0) }
}

fn qbit_1() -> QBit {
    QBit {a: cplx(0.0), b: cplx(1.0) }
}

fn qbit_super() -> QBit{
    QBit {a: cplx(one_sqrt_two!()), b: cplx(one_sqrt_two!())}
}
