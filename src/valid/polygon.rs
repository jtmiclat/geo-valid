use geo_types::{LineString, Polygon};

pub trait PolygonValidation {
    fn is_valid(&self) -> bool;
}
impl PolygonValidation for Polygon {
    fn is_valid(&self) -> bool {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_points_are_valid() {
        let polygon = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        assert_eq!(polygon.is_valid(), true)
    }
}
