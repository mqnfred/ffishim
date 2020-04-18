#[macro_use]
extern crate ffishim_derive;

#[ffishim]
fn give_string() -> String {
    "Hello, world!".to_owned()
}

#[ffishim]
fn take_string(s: String) -> String {
    format!("Hello, {}!", s)
}
