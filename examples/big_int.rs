use num_bigint::BigUint;
use num_traits::{One, Zero};
use rust_tailcall::TailCall;

/// Computes the factorial of `n` using tail recursion.
///
/// # Arguments
/// * `n` - The number to compute the factorial for.
/// * `acc` - The accumulator holding the running result.
///
/// # Returns
/// A `TailCall` that will eventually produce the factorial of `n`.
pub fn factorial(n: BigUint, acc: BigUint) -> TailCall<BigUint> {
    if n.is_zero() {
        TailCall::Done(acc)
    } else {
        let next_n = &n - BigUint::one();
        let next_acc = &acc * &n;
        TailCall::Continue(Box::new(move || factorial(next_n.clone(), next_acc.clone())))
    }
}

fn main() {
    // Example 1: Normal case
    let result = factorial(BigUint::from(5u32), BigUint::one()).invoke();
    println!("Factorial of 5 is: {}", result);

    // Example 2: Edge case where n = 0
    let result_zero = factorial(BigUint::zero(), BigUint::one()).invoke();
    println!("Factorial of 0 is: {}", result_zero);

    // Example 3: Larger number to test stack safety and arbitrary precision
    let large = BigUint::from(1000u32); // Reduced from 10000 for practicality
    let result_large = factorial(large.clone(), BigUint::one()).invoke();
    println!("Factorial of 1000 has {} digits", result_large.to_str_radix(10).len());
}
