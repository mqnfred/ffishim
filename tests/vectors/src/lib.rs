#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

#[ffishim_use_case]
fn push_gps(mut coordinates: Vec<GPS>, gps: GPS) -> Vec<GPS> {
    coordinates.push(gps);
    coordinates
}

#[derive(FFIShim)]
pub struct Player {
    name: String,
    points: Vec<u64>,
}

#[ffishim_use_case]
fn player_sum_points(p: Player) -> u64 {
    let a = p.points.iter().sum();
    a
}
