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



## 🔗 Helpful Links & Resources

* 📘 [The Art of Simplicity_Venkat Subramaniam](https://www.youtube.com/watch?v=AFZMI4y7Cuk&list=LL&index=2)


## 📜 License

* Mozilla Public License 2.0