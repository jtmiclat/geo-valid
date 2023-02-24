use geo_types::geometry::Geometry;
use linestring::validate_linestring;
use point::validate_point;

pub mod coord;
pub mod linestring;
pub mod point;
pub mod polygon;
pub mod validator;

pub fn validate(geom: Geometry) -> validator::Validation {
    match geom {
        Geometry::Point(g) => validate_point(g),
        Geometry::LineString(g) => validate_linestring(g),
        _ => validator::Validation {
            is_valid: true,
            errors: vec![],
        },
    }
}
