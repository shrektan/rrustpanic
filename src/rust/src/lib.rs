use extendr_api::prelude::*;
use std::panic;

struct Test {
    value: i32,
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Executing the destructor...")
    }
}

/// @export
#[extendr]
fn run(x : Robj) -> i32 {
    let out = Test {value: 123};
    match x.rtype() {
        Rtype::Integers => {
            out.value
        },
        _ => {
            panic!("x is not an integer");
        }
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rrustpanic;
    fn run;
}
