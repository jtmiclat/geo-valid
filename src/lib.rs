use geo_types::geometry::Geometry;
use geometry_collection::validate_geometry_collection;
use line_string::validate_line_string;
use multi_line_string::validate_multi_line_string;
use multi_point::validate_multi_point;
use multi_polygon::validate_multi_polygon;
use point::validate_point;
use polygon::validate_polygon;

pub mod coord;
pub mod geometry_collection;
pub mod line_string;
pub mod multi_line_string;
pub mod multi_point;
pub mod multi_polygon;
pub mod point;
pub mod polygon;
pub mod validation;

pub fn validate(geom: Geometry) -> validation::Validation {
    match geom {
        Geometry::Point(g) => validate_point(g),
        Geometry::LineString(g) => validate_line_string(g),
        Geometry::Polygon(g) => validate_polygon(g),
        Geometry::MultiPoint(g) => validate_multi_point(g),
        Geometry::MultiLineString(g) => validate_multi_line_string(g),
        Geometry::MultiPolygon(g) => validate_multi_polygon(g),
        Geometry::GeometryCollection(g) => validate_geometry_collection(g),
        _ => validation::Validation {
            is_valid: true,
            errors: vec![],
        },
    }
}
