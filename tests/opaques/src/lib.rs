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

#[derive(FFIShim)]
pub struct HideAndSeek {
    lookatme: u64,
    #[ffishim(opaque)]
    hideme: u64,
}

#[ffishim_use_case]
fn get_hideandseek() -> HideAndSeek {
    HideAndSeek{lookatme: 2, hideme: 7}
}

#[ffishim_use_case]
fn set_hidden_field(mut hs: HideAndSeek, to: u64) -> HideAndSeek {
    hs.hideme = to;
    hs
}

#[ffishim_use_case]
fn get_hidden_field(hs: HideAndSeek) -> u64 {
    hs.hideme
}
