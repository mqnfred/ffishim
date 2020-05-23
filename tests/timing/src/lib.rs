#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[derive(FFIShim)]
pub struct WorldRecord {
    time: ::chrono::Duration,
}

#[ffishim_use_case]
fn get_duration() -> ::chrono::Duration {
    ::chrono::Duration::seconds(5)
}
