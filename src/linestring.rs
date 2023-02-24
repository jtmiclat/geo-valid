use super::point::validate_point;
use geo::coords_iter::CoordsIter;
use geo_types::LineString;

use super::validation::Validation;

pub fn validate_linestring_points(line: &LineString) -> Validation {
    let mut errors: Vec<String> = vec![];
    for point in line.points() {
        let validation = validate_point(point);
        if validation.is_valid == false {
            errors.extend(validation.errors)
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}

fn validate_linestring_length(line: &LineString) -> Validation {
    let mut errors: Vec<String> = vec![];
    let length = line.coords_count();
    if length == 1 {
        let error_message = format!(
            "Linestring {:?} should be empty or contain 2 or more coords",
            line
        );
        errors.push(error_message)
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
pub fn validate_linestring(line: LineString) -> Validation {
    let mut errors: Vec<String> = vec![];
    let points_validation = validate_linestring_points(&line);
    if points_validation.is_valid == false {
        errors.extend(points_validation.errors)
    }
    let length_validation = validate_linestring_length(&line);
    if length_validation.is_valid == false {
        errors.extend(length_validation.errors)
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::coord;
    use std::f64;
    #[test]
    fn all_points_are_valid() {
        let line_string = LineString::new(vec![coord! { x: 0., y: 0. }, coord! { x: 10., y: 0. }]);
        let validation = validate_linestring(line_string);
        assert_eq!(validation.is_valid, true);
        assert_eq!(validation.errors.len(), 0);
    }
    #[test]
    fn one_point_is_invalid() {
        let line_string = LineString::new(vec![
            coord! { x: 0., y: 0. },
            coord! { x: 10., y: f64::INFINITY},
        ]);
        let validation = validate_linestring(line_string);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }

    #[test]
    fn some_points_are_invalid() {
        let line_string = LineString::new(vec![
            coord! { x: 0., y: 0. },
            coord! { x: 10., y: f64::INFINITY},
            coord! { x:f64::NEG_INFINITY, y: 0.},
            coord! { x:f64::NAN, y: 0.},
        ]);
        let validation = validate_linestring(line_string);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 3);
    }
    #[test]
    fn zero_points() {
        let line_string = LineString::new(vec![]);
        let validation = validate_linestring(line_string);
        assert_eq!(validation.is_valid, true);
        assert_eq!(validation.errors.len(), 0);
    }
    #[test]
    fn enough_points() {
        let line_string = LineString::new(vec![
            coord! { x: 0., y: 0. },
            coord! { x: 10., y: 0. },
            coord! { x: 10., y: 10. },
        ]);
        let validation = validate_linestring(line_string);
        assert_eq!(validation.is_valid, true);
        assert_eq!(validation.errors.len(), 0);
    }
    #[test]
    fn not_enough_points() {
        let line_string = LineString::new(vec![coord! { x: 0., y: 0. }]);
        let validation = validate_linestring(line_string);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
}
