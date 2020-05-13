#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

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

#[ffishim_use_case]
fn add(a: i64, b: i64) -> i64 {
    a + b
}
