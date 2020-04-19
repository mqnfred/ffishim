#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub enum Location {
    Address {
        line: Option<String>,
        city: String,
        zip: u64,
    },
    GPS(GPS),
    Unknown,
}

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
    planet: Option<u64>,
}

#[ffishim]
fn cheese_shop(gps: GPS) -> GPS {
    GPS {
        lon: -122.109926,
        ..gps
    }
}
