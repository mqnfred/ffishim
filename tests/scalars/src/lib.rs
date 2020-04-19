#[macro_use]
extern crate ffishim_derive;

#[ffishim]
fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[ffishim]
fn add_f64(a: f64, b: f32) -> f64 {
    a + b as f64
}

#[ffishim]
fn fails(a: i64) -> Result<i64, ::ffishim::library::Error> {
    if a == 5 {
        Ok(a + 1)
    } else {
        Err(::ffishim::library::Error::msg("only 5 accepted"))
    }
}

#[derive(FFIShim)]
pub enum Location {
    GPS(GPS),
    Address {
        address: String,
        city: String,
    }
}

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
    planet: Option<u64>,
}

#[ffishim]
fn where_to_buy_cheese(gps: GPS) -> GPS {
    GPS {
        lat: 37.405263,
        lon: -122.109926,
        planet: None,
    }
}
