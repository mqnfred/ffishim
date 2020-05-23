#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[ffishim_use_case]
fn get_duration() -> ::chrono::Duration {
    ::chrono::Duration::seconds(5)
}
