#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

#[ffishim]
fn push_gps(mut coordinates: Vec<GPS>, gps: GPS) -> Vec<GPS> {
    coordinates.push(gps);
    coordinates
}

#[derive(FFIShim)]
pub struct Player {
    name: String,
    points: Vec<u64>,
}

#[ffishim]
fn player_sum_points(p: Player) -> u64 {
    let a = p.points.iter().sum();
    a
}
