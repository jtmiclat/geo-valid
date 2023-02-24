use geo_types::geometry::Geometry;
use point::validate_point;

pub mod coord;
pub mod linestring;
pub mod point;
pub mod polygon;
pub mod validator;

pub fn validate(geom: Geometry) -> validator::Validation {
    match geom {
        Geometry::Point(g) => validate_point(g),
        _ => validator::Validation {
            is_valid: false,
            errors: vec![],
        },
    }
}
