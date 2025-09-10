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
