use geo_types::MultiPolygon;

use crate::{polygon::validate_polygon, validation::Validation};

pub fn validate_multi_polygon(multi_polygon: MultiPolygon) -> Validation {
    let mut errors: Vec<String> = vec![];
    for polygon in multi_polygon {
        let polygon_validation = validate_polygon(polygon);
        if polygon_validation.is_valid == false {
            errors.extend(polygon_validation.errors);
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
