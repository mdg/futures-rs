#![feature(proc_macro, generators, pin)]

#[async]
fn foo() -> Result<i32, i32> {
    let a: i32 = "a"; //~ ERROR: mismatched types
    Ok(1)
}

fn main() {}
