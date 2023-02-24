use geo_types::geometry::Geometry;
use linestring::validate_linestring;
use point::validate_point;
use polygon::validate_polygon;

pub mod coord;
pub mod linestring;
pub mod point;
pub mod polygon;
pub mod validation;

pub fn validate(geom: Geometry) -> validation::Validation {
    match geom {
        Geometry::Point(g) => validate_point(g),
        Geometry::LineString(g) => validate_linestring(g),
        Geometry::Polygon(g) => validate_polygon(g),
        _ => validation::Validation {
            is_valid: true,
            errors: vec![],
        },
    }
}
