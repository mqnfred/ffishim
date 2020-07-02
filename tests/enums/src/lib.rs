#[macro_use]
extern crate ffishim_derive;

#[ffishim_library]
mod ffishim_library {}

#[ffishim_function]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[derive(FFIShim)]
pub enum Location {
    GPS(f64, f64),
    Address {
        addr: String,
        city: String,
        zip: i64,
    },
    Unknown,
}

#[derive(FFIShim)]
pub enum Type {
    Assets,
    Trading,
}
