#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub struct Player {
    name: String,
    age: Option<u64>,
    coordinates: GPS,
}

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

#[ffishim]
fn set_player_lat(mut p: Player, lat: f64) -> Player {
    p.coordinates.lat = lat;
    p
}

#[ffishim]
fn get_player_1() -> Player {
    Player{
        name: "lyz".to_owned(),
        age: Some(28),
        coordinates: GPS{
            lat: 37.405263,
            lon: -122.109926,
        },
    }
}
