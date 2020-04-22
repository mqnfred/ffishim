#[macro_use]
extern crate ffishim_derive;

#[ffishim_use_case]
fn give_string() -> String {
    "Hello, world!".to_owned()
}

#[ffishim_use_case]
fn take_string(s: String) -> String {
    format!("Hello, {}!", s)
}
