use super::validation::Validation;
use geo_types::Point;

pub fn validate_point(point: Point) -> Validation {
    let mut errors: Vec<String> = vec![];
    if !(point.x().is_finite() && point.y().is_finite()) {
        let error_message = format!("Coordinates of {:?} are not finite", point);
        errors.push(error_message);
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    #[test]
    fn valid_point() {
        let point = Point::new(1., 1.);
        let validation = validate_point(point);
        assert_eq!(validation.is_valid, true);
        assert_eq!(validation.errors.len(), 0);
    }
    #[test]
    fn invalid_point_infinity() {
        let point = Point::new(f64::INFINITY, 1.);
        let validation = validate_point(point);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn invalid_point_neg_infinity() {
        let point = Point::new(1., f64::NEG_INFINITY);
        let validation = validate_point(point);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn invalid_point_nan() {
        let point = Point::new(1., f64::NAN);
        let validation = validate_point(point);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
}
