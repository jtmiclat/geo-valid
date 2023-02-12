use geo_types::Point;
pub trait PointValidationExt {
    fn is_valid(&self) -> bool;
}

impl PointValidationExt for Point {
    fn is_valid(&self) -> bool {
        if self.0.x.is_finite() && self.0.y.is_finite() {
            return true;
        } else {
            return false;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    #[test]
    fn valid_point() {
        let point = Point::new(1., 1.);
        assert_eq!(point.is_valid(), true)
    }
    #[test]
    fn invalid_point() {
        let point = Point::new(f64::INFINITY, 1.);
        assert_eq!(point.is_valid(), false);
        let point = Point::new(1., f64::INFINITY);
        assert_eq!(point.is_valid(), false);
        let point = Point::new(f64::INFINITY, f64::INFINITY);
        assert_eq!(point.is_valid(), false);
    }
}
