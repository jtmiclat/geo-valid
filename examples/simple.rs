use geo::{Geometry, LineString, Polygon};
use geo_valid::validate;

fn main() {
    let exterior = LineString::from(vec![(0., 0.), (2., 2.), (2., 0.), (0., 2.)]);
    let interiors = vec![];
    let polygon = Polygon::new(exterior, interiors);
    let geom: Geometry = polygon.into();
    let validation = validate(geom);
    println!("{:?}", validation);
}
