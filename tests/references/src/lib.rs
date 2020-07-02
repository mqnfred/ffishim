#[macro_use]
extern crate ffishim_derive;

#[ffishim_library]
mod ffishim_library {}

#[derive(FFIShim)]
#[ffishim(opaque)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

#[ffishim_function]
fn new_gps() -> GPS {
    GPS{lat: 1.0, lon: 2.0}
}

#[ffishim_function]
fn set_lat(gps: &mut GPS, lat: f64) -> f64 {
    gps.lat = lat;
    gps.lon = lat + 1.0;
    lat
}

#[ffishim_function]
fn get_lon(gps: &mut GPS) -> f64 {
    gps.lon
}
