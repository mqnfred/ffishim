#[macro_use]
extern crate ffishim_derive;

#[ffishim_function]
fn give_string() -> String {
    "Hello, world!".to_owned()
}

#[ffishim_function]
fn take_string(s: String) -> String {
    format!("Hello, {}!", s)
}
