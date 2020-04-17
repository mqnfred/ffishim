#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub enum Location {
    GPS(f64, f64),
}

#[ffishim]
pub fn lol(l: u32) -> Result<u32, ::anyhow::Error> {
    Ok(l)
}

/*
#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}
*/
