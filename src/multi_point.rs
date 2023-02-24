use geo_types::MultiPoint;

use crate::{point::validate_point, validation::Validation};

pub fn validate_multi_point(multie_point: MultiPoint) -> Validation {
    let mut errors: Vec<String> = vec![];
    for point in multie_point {
        let point_validation = validate_point(point);
        if point_validation.is_valid == false {
            errors.extend(point_validation.errors);
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
