#[macro_use]
extern crate ffishim_derive;

#[ffishim_library]
mod ffishim_library {}

#[derive(FFIShim)]
pub struct WorldRecord {
    time: ::chrono::Duration,
}

#[ffishim_function]
fn get_duration() -> ::chrono::Duration {
    ::chrono::Duration::seconds(5)
}
