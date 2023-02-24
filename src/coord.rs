use super::validator::Validation;
use geo_types::Coord;

pub fn validate_coord(coord: Coord) -> Validation {
    let mut errors: Vec<String> = vec![];
    if coord.x.is_finite() && coord.y.is_finite() {
        return Validation {
            is_valid: true,
            errors: errors,
        };
    } else {
        errors.push(String::from("Coordinates are infinite"));
        return Validation {
            is_valid: false,
            errors: errors,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    #[test]
    fn valid_point() {
        let coord = Coord { x: 1., y: 1. };
        let validation = validate_coord(coord);
        assert_eq!(validation.is_valid, true);
        assert_eq!(validation.errors.len(), 0);
    }
    #[test]
    fn invalid_point_infinity() {
        let coord = Coord {
            x: f64::INFINITY,
            y: 1.,
        };
        let validation = validate_coord(coord);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn invalid_point_neg_infinity() {
        let coord = Coord {
            x: 1.,
            y: f64::NEG_INFINITY,
        };
        let validation = validate_coord(coord);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn invalid_point_nan() {
        let coord = Coord { x: 1., y: f64::NAN };
        let validation = validate_coord(coord);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
}
