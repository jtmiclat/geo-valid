use geo_types::MultiLineString;

use crate::{line_string::validate_line_string, validation::Validation};

pub fn validate_multi_line_string(multi_line_string: MultiLineString) -> Validation {
    let mut errors: Vec<String> = vec![];
    for line_string in multi_line_string {
        let line_string_validation = validate_line_string(line_string);
        if line_string_validation.is_valid == false {
            errors.extend(line_string_validation.errors);
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
