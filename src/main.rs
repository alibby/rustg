extern crate geo;
extern crate serde;
extern crate wkt;

use geo::Coordinate;
use geo::Geometry;
use geo::LineString;

use std::env;
use std::fs::File;
use wkt::ToWkt;

fn make_wkt_string(ls : LineString<f32>) -> String {
    let g = Geometry::from(ls);
    format!("{}", g.to_wkt().items[0])
}

fn read_points(f: File) -> LineString<f32> {
    let coordinator = |result| -> Option<Coordinate<f32>> {
        match result {
            Ok(coord) => Some(coord),
            _ => None,
        }
    };

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(f);

    let points = rdr
        .deserialize()
        .map(coordinator)
        .filter(|option| option.is_some())
        .map(|option| option.unwrap())
        .collect();

    LineString(points)
}

fn main() -> std::io::Result<()> {
    env::args().skip(1).for_each(|filename: String| {
        match File::open(&filename[..]) {
            Ok(f) => println!("{}", make_wkt_string(read_points(f))),
            Err(e) => eprintln!("Error '{}': {}", filename, e),
        };
    });

    Ok(())
}
