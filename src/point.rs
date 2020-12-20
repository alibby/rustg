extern crate geo;
extern crate wkt;

use geo::Coordinate;
use geo::Geometry;
use geo::Point;
use wkt::ToWkt;

// fn to_wkt(pt: Geometry) -> String {
//     "xxxxxx".to_string()
// }

fn main() -> std::io::Result<()> {
    let pt = Point(Coordinate { x: 1.0f64, y: 1.0f64 });
    let g = Geometry::from(pt);
    println!("{}", g.to_wkt().items[0]);
    println!("{:?}", pt);

    // let wkt = Wkt::new();
    // let x = wkt.try_from(pt)
    // println!("{:?}", x);
    Ok(())
}
