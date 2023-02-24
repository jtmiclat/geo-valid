use super::line_string::validate_line_string;
use super::multi_line_string::validate_multi_line_string;
use super::multi_point::validate_multi_point;
use super::multi_polygon::validate_multi_polygon;
use super::point::validate_point;
use super::polygon::validate_polygon;
use geo::{Geometry, GeometryCollection};

use crate::validation::{self, Validation};

pub fn validate_geometry_collection(geometry_collection: GeometryCollection) -> Validation {
    let mut errors: Vec<String> = vec![];
    for geometry in geometry_collection {
        let validation = validate_geometry(geometry);
        if validation.is_valid == false {
            errors.extend(validation.errors);
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}

// This is placed here because of the possible recursive nature of geometry collections
pub fn validate_geometry(geom: Geometry) -> validation::Validation {
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
