#[macro_use]
extern crate ffishim_derive;

#[ffishim]
fn give_option() -> Option<u32> {
    Some(28)
}

#[ffishim]
fn take_option(opt: Option<u32>) -> Option<u32> {
    if opt.is_none() {
        Some(32)
    } else {
        None
    }
}

#[ffishim]
fn string_option(opt: Option<String>) -> Option<String> {
    opt.map(|s| format!("Hello, {}!", s))
}

#[ffishim]
fn option_result(input: Option<u32>) -> Result<Option<u32>, ::ffishim::library::Error> {
    Ok(Some(input.ok_or(::ffishim::library::Error::msg("no input"))? + 1))
}
