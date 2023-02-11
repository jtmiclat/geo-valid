use super::point::PointValidation;
use geo_types::LineString;
pub trait LineStringValidation {
    fn is_valid(&self) -> bool;
}

impl LineStringValidation for LineString {
    fn is_valid(&self) -> bool {
        if !self.points().all(|p| p.is_valid()) {
            return false;
        } else {
            // Line string must have 2 or more points
            // TODO: check if there is an intersection?
            return self.points().count() >= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::coord;
    use std::f64;
    #[test]
    fn all_points_are_valid() {
        let line_string = LineString::new(vec![coord! { x: 0., y: 0. }, coord! { x: 10., y: 0. }]);
        assert_eq!(line_string.is_valid(), true)
    }
    #[test]
    fn some_points_are_invalid() {
        let line_string = LineString::new(vec![
            coord! { x: 0., y: 0. },
            coord! { x: 10., y: f64::INFINITY},
        ]);
        assert_eq!(line_string.is_valid(), false)
    }
    #[test]
    fn enough_points() {
        let line_string = LineString::new(vec![
            coord! { x: 0., y: 0. },
            coord! { x: 10., y: 0. },
            coord! { x: 10., y: 10. },
        ]);
        assert_eq!(line_string.is_valid(), true)
    }
    #[test]
    fn not_enoug_points() {
        let line_string = LineString::new(vec![coord! { x: 0., y: 0. }]);
        assert_eq!(line_string.is_valid(), false)
    }
}
