#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

#[ffishim]
fn milk_pail_coordinates() -> GPS {
    GPS {
        lat: 37.405263,
        lon: -122.109926,
    }
}
