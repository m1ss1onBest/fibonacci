use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::{env, mem};

type Matrix = [[BigUint; 2]; 2];

#[inline]
fn matrix_default() -> Matrix {
    [
        [BigUint::one(), BigUint::one()],
        [BigUint::one(), BigUint::zero()],
    ]
}

#[inline]
fn mul_matrices(a: Matrix, b: Matrix) -> Matrix {
    let mut result = [
        [BigUint::zero(), BigUint::zero()],
        [BigUint::zero(), BigUint::zero()],
    ];

    result[0][0] = &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0];
    result[0][1] = &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1];
    result[1][0] = &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0];
    result[1][1] = &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1];

    result
}

fn matrix_power(mut m: Matrix, mut n: u64) -> Matrix {
    let mut res = [
        [BigUint::one(), BigUint::zero()],
        [BigUint::zero(), BigUint::one()],
    ];

    while n > 0 {
        if n & 1 == 1 {
            res = mul_matrices(mem::replace(&mut res, matrix_default()), m.clone());
        }
        m = mul_matrices(m.clone(), m);
        n >>= 1;
    }

    res
}

#[inline]
fn fibonacci(n: u64) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        n => matrix_power(matrix_default(), n - 1)[0][0].clone(),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Computes the nth Fibonacci number. Takes 1 number as argument.");
        return Ok(())
    }

    let n: u64 = args[1]
        .parse()
        .map_err(|e| format!("Failed to parse number: {}", e))?;

    println!("{}", fibonacci(n));
    Ok(())
}
