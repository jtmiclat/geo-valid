use geo_types::Coord;

pub trait CoordValidationExt {
    fn is_valid(&self) -> bool;
}

impl CoordValidationExt for Coord {
    fn is_valid(&self) -> bool {
        if self.x.is_finite() && self.y.is_finite() {
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
    fn valid_coord() {
        let coord = Coord::from((1., 1.));
        assert_eq!(coord.is_valid(), true)
    }
    #[test]
    fn invalid_coord() {
        let coord = Coord::from((f64::INFINITY, 1.));
        assert_eq!(coord.is_valid(), false);
        let coord = Coord::from((1., f64::INFINITY));
        assert_eq!(coord.is_valid(), false);
        let coord = Coord::from((f64::INFINITY, f64::INFINITY));
        assert_eq!(coord.is_valid(), false);
    }
}
