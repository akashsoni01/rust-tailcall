# rust-tailcall

# TailCall Implementation in Rust

This document explains how to implement and use tail call optimization (TCO) using Rust's type system and functional style. The crate is a rust implementation of The Art of Simplicity_Venkat Subramaniam.

## 📖 What is Tail Call?

A **tail call** is a function call performed as the final action of a function. In languages without proper tail call optimization, deep recursion can cause a stack overflow.

Rust does not guarantee tail call optimization at the compiler level, but we can simulate it using enums and iteration.

---

## ✅ TailCall in Rust

We define a `TailCall` enum that represents two states:

1. `Continue`: A pending computation wrapped in a closure.
2. `Done`: A completed result.

```rust
enum TailCall<T> {
    Continue(Rc<dyn Fn() -> TailCall<T>>),
    Done(T),
}
```


## ✅ TailCall in basic example

```rust
use rust_tailcall::TailCall;

// Example usage: factorial function using TailCall
fn factorial(n: u64, acc: u64) -> TailCall<u64> {
    if n == 0 {
        TailCall::Done(acc)
    } else {
        TailCall::Continue(Box::new(move || factorial(n - 1, acc * n)))
    }
}

fn main() {
    let result = factorial(5, 1).invoke();
    println!("Factorial result: {}", result);
}
```

## ✅ TailCall in bigint example

```rust
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
```


## 🔗 Helpful Links & Resources

* 📘 [The Art of Simplicity_Venkat Subramaniam](https://www.youtube.com/watch?v=AFZMI4y7Cuk&list=LL&index=2)


## 📜 License

* Mozilla Public License 2.0