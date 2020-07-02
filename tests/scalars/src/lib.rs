#[macro_use]
extern crate ffishim_derive;

#[ffishim_library]
mod ffishim_library {}

#[ffishim_function]
fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[ffishim_function]
fn add_f64(a: f64, b: f32) -> f64 {
    a + b as f64
}

#[ffishim_function]
fn fails(a: i64) -> Result<i64, ::ffishim::library::Error> {
    if a == 5 {
        Ok(a + 1)
    } else {
        Err(::ffishim::library::Error::msg("only 5 accepted"))
    }
}

#[ffishim_function]
fn not(a: bool) -> bool {
    !a
}
