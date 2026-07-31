//@ run-fail
//@ needs-unwind
//@ error-pattern:explicit panic
//@ needs-subprocess

// Test that we get the correct message for resuming a panicked coroutine.

#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::Coroutine;
use std::panic;
use std::pin::Pin;

fn main() {
    let mut g = #[coroutine]
    || {
        panic!();
        yield;
    };
    panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let x = Pin::new(&mut g).resume(());
    }));
    // Coroutine is optimimized to have no state. So the panic should trigger again
    Pin::new(&mut g).resume(());
}
