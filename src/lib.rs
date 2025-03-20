use sha2;

// Elliptic curve formula = y**2 = x**3 + ax + b
//
// Bitcoin's EC is more like a scatter plot
// It's an EC over a finite field of whole numbers (mod p)
// It restricts the numbers to within a certain range
//
// The curve used in bitcoin is secp256k1

// Building blocks of ECDSA is:
// Parameters of secp256k1 curve (bitcoin's EC)
// Modular inverse -
// Add -
// Double -
// Multiply -

// Secp256k1 curve parameters
// Using section 2.4.1 to set the parameters -> https://www.secg.org/sec2-v2.pdf
// y² = x³ + ax + b
pub const A: [u8; 32] = [0x00; 32];
pub const B: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07,
];

// Finite Field prime number (field size); size of the finite field
// In decimal = 115792089237316195423570985008687907853269984665640564039457584007908834671663
pub const P: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xfc, 0x2f,
];

// Order of the Secp256k1 curve; number of points on the curve
// In decimal = 115792089237316195423570985008687907852837564279074904382605163141518161494337
pub const N: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41,
];

// X coordinate of the generator
// In decimal = 55066263022277343669578718895168534326250603453777594175500187360389116729240
pub const GENERATOR_X: [u8; 32] = [
    0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87, 0x0b, 0x07,
    0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16, 0xf8, 0x17, 0x98,
];

// Y coordinate of the generator
// In decimal = 32670510020758816978083085130507043184471273380659243275938904335757337482424
pub const GENERATOR_Y: [u8; 32] = [
    0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d, 0xa4, 0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8,
    0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54, 0x19, 0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8,
];

// G constant (generator point) - all calculations start with this point
pub const G: (&[u8; 32], &[u8; 32]) = (&GENERATOR_X, &GENERATOR_Y);

pub enum EcdsaError {
    InvalidPointAddition,
}

pub enum ArithmeticError {
    InvalidAddition,
    InvalidSubstraction,
    InvalidMultiplicaiton,
    //InvalidAddition,
}

/// Helper function to compare arrays of [u8;32] .
/// TODO: Move to separate file later
fn is_greater_or_equal(x: &[u8; 32], y: &[u8; 32]) -> bool {
    for i in 0..32 {
        if x[i] > y[i] {
            return true;
        }
        if x[i] < y[i] {
            return false;
        }
    }
    true
}

// A point on the secp256k1 curve
#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: [u8; 32],
    y: [u8; 32],
}

impl Point {
    /// The generator point G
    pub fn generator() -> Self {
        Point {
            x: GENERATOR_X,
            y: GENERATOR_Y,
        }
    }

    // Scalar multiiplcation - doubles a point
    pub fn double() -> Self {
        unimplemented!("Need to implement point doubling")
    }

    // Point addition is a geometric operation which takes 2 points on the curve
    // P and Q and produces a third point R = P + Q.
    //
    // Considerations:
    //
    // P and Q are distinct points (P != Q)
    // 2 other considerations (will handle later)
    //  1. P = Q --> this is point doubling
    //  2. One of the points is the point at infinity or (0, 0)
    pub fn add(p: Point, q: Point) -> Result<Point, EcdsaError> {
        if p.x == q.x && p.y == q.y {
            panic!("Use double() for equal points");
        }

        // compute slope: s = (y2 - y1) / (x2 - x1) mod P

        unimplemented!()
    }
}

mod arithmetic_operations {
    use std::num::Wrapping;

    use super::*;

    // addition operation: c = (a + b) mod P
    //
    // a and b are inputs being added together
    // modulus is my P (finite field size)
    // Result C must be in range of [0, P-1]
    //
    pub fn addition(a: &[u8; 32], b: &[u8; 32], modulus: &[u8; 32], adjustment: bool) -> [u8; 32] {
        let mut result = [0; 32];
        let mut carry = 0;

        // iterate over 32 byte arrays from right to left
        for i in (0..32).rev() {
            // cast values as u16 to catch overflow
            let temp = a[i] as u16 + b[i] as u16 + carry as u16;
            println!("IN ADDITION: i:{}, temp:{},", i, temp);
            // Set the ith value of result as u8 (0-255) value
            result[i] = temp as u8;
            // If overflow occured (a+b was larger then 255) carry it over to the next iteration
            carry = (temp >> 8) as u8;
        }

        println!("ADDITION LOOP END: carry: {:?},", carry);
        // We need to adjust result if our value is > P or >= P
        //
        // Only adjust if not already 0 with carry (i.e., 2^256 mod P)
        let is_zero = result.iter().all(|&x| x == 0);
        // carry != 0: a + b >= 2**256
        // result >= modulus: a + b > P
        if !adjustment
            && (carry != 0 || is_greater_or_equal(&result, modulus))
            && !(is_zero && carry == 1)
        {
            println!("CARRY > 0, RESULT: {:?},", result);
            result = subtract(&result, modulus, modulus, true)
        }

        println!("EXITING ADDITION: result: {:?},", result);
        result
    }

    // substract operation: c = a - b mod P
    //
    // a and b: 256-bit integers as [u8;32] (big endian)
    // P: is modulus (the secp256k1 prime / fixed of my finite field)
    // c: must be in the range of [0, P-1]
    //
    //NOTE: addition has carries, substract has borrowing when a[i] < b[i]
    // This implies a negative result, so we add P to wrap it around
    // (back to postive) while ensuring it's < P.
    pub fn subtract(a: &[u8; 32], b: &[u8; 32], modulus: &[u8; 32], adjustment: bool) -> [u8; 32] {
        let mut result = [0; 32];
        let mut borrow = 0;

        // iterate over 32 byte arrays from right to left
        for i in (0..32).rev() {
            // cast values as u16 to catch overflow
            let mut temp = Wrapping(a[i] as u16) - Wrapping(b[i] as u16) - Wrapping(borrow as u16);
            println!("i={}, temp={}", i, temp.0);
            if temp > Wrapping(255) {
                temp += Wrapping(256);
                println!("temp was smaller then wrapping 255: {:?}", temp.0);
                borrow = 1;
            } else {
                borrow = 0;
            }
            result[i] = temp.0 as u8;
        }

        println!("SUBTRACT LOOP END: borrow: {:?},", borrow);
        // if borrow is not 0, then result is negative
        // run the result through addition to add P to it
        if !adjustment && borrow != 0 {
            result = addition(&result, modulus, modulus, true);
        }

        println!("EXITING SUBTRACT: result: {:?},", result);
        result
    }
}

#[cfg(test)]
mod tests {
    use arithmetic_operations::{addition, subtract};

    use super::*;

    #[test]
    fn test_simple_byte_array_addition() {
        let a: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03,
        ];
        let b: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x05,
        ];
        let correct_result: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x08,
        ];
        let result = addition(&a, &b, &P, false);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_byte_array_addition_with_carry() {
        let a: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xFF,
        ];
        let b: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ];
        let correct_result: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00,
        ];
        let result = addition(&a, &b, &P, false);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_addition_near_p_with_carry() {
        let mut a = P;
        let one = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ];
        a = subtract(&a, &one, &P, false); // P - 1
        let b = one;
        let correct_result = [0; 32]; // 0 mod P
        let result = addition(&a, &b, &P, false);
        println!("P-1 + 1 = {:?}", result);
        assert_eq!(result, correct_result);
    }
    #[test]
    fn test_addition_max_plus_one() {
        let a = [0xFF; 32]; // 2^256 - 1
        let b = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]; // 1
        let correct_result = [0; 32]; // 0 mod P (since 2^256 - 1 + 1 = 2^256 = 0 mod P)
        let result = addition(&a, &b, &P, false);
        println!("2^256 - 1 + 1 = {:?}", result);
        assert_eq!(result, correct_result);
    }
    //#[test]
    //fn test_is_greater_or_equal() {
    //    let x1 = [0x01, 0x00, 0x00]; // 256
    //    let y1 = [0x02, 0x00, 0x00]; // 512
    //    assert_eq!(is_greater_or_equal(&x1, &y1), false); // 256 < 512
    //
    //    let x2 = [0x02, 0x00, 0x00]; // 512
    //    let y2 = [0x01, 0x00, 0x00]; // 256
    //    assert_eq!(is_greater_or_equal(&x2, &y2), true); // 512 > 256
    //
    //    let x3 = [0x01, 0x00, 0x00]; // 256
    //    let y3 = [0x01, 0x00, 0x00]; // 256
    //    assert_eq!(is_greater_or_equal(&x3, &y3), true); // 256 = 256
    //}

    #[test]
    fn test_simple_byte_array_substraction_with_carry() {
        let a: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00,
        ];
        let b: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ];
        let correct_result: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xFF,
        ];
        let result = subtract(&a, &b, &P, true);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_negative_wrapping_byte_array_subtraction() {
        let a: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02,
        ];
        let b: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04,
        ];
        let two: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02,
        ];
        let mut correct_result = P;
        // correct result here is P - 2
        // We'll calculate it dynamically using the subtract function itself
        correct_result = subtract(&correct_result, &two, &P, false);
        let result = subtract(&a, &b, &P, false);
        assert_eq!(result, correct_result);
    }
}
