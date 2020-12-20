extern crate geo;
extern crate serde;
extern crate wkt;

use geo::Coordinate;
use geo::Geometry;
use geo::LineString;

use std::env;
use std::fs::File;
use serde::Deserialize;
use wkt::ToWkt;

#[derive(Debug, Deserialize)]
struct Coord {
    x: f32,
    y: f32,
}

fn make_wkt_string(ls : LineString<f32>) -> String {
    let g = Geometry::from(ls);
    format!("{}", g.to_wkt().items[0])
}

fn read_points(filename: &str) -> std::io::Result<LineString<f32>> {
    let f = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(f);

    let points = rdr.deserialize()
        .map(|result| -> Option<Coordinate<f32>> {
            match result {
                Ok(v) => {
                    let c: Coord = v;
                    Some(Coordinate { x: c.x, y: c.y })
                },

                _ => None,
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let ls = LineString(points);
    Ok(ls)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let ls = read_points(filename)?;

    println!("{}", make_wkt_string(ls));

    Ok(())
}
