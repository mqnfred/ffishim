#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
#[ffishim(opaque)]
pub struct App {
    size: u64,
}

#[ffishim_use_case]
fn get_app() -> App {
    App{size: 5}
}

#[ffishim_use_case]
fn add_to_app_size(mut app: App, additional: u64) -> App {
    app.size += additional;
    app
}

#[ffishim_use_case]
fn get_app_size(app: App) -> u64 {
    app.size
}
