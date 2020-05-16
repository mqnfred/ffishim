#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

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

#[ffishim_use_case]
fn set_player_lat(mut p: Player, lat: f64) -> Player {
    p.coordinates.lat = lat;
    p
}

#[ffishim_use_case]
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

#[derive(FFIShim)]
pub struct Coordinates(i64, i64);

#[ffishim_use_case]
fn set_x(mut coord: Coordinates, x: i64) -> Coordinates {
    coord.0 = x;
    coord
}
