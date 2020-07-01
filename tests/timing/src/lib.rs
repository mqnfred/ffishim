#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub struct WorldRecord {
    time: ::chrono::Duration,
}

#[ffishim_function]
fn get_duration() -> ::chrono::Duration {
    ::chrono::Duration::seconds(5)
}
