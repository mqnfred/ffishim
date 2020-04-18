#[macro_use]
extern crate ffishim_derive;

#[ffishim]
fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[ffishim]
fn add_f64(a: f64, b: f32) -> f64 {
    a + b as f64
}
