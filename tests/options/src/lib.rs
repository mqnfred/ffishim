#[macro_use]
extern crate ffishim_derive;

#[ffishim]
fn add(a: i64, b: i64) -> i64 {
    a + b
}
