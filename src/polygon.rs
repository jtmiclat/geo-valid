use geo_types::Polygon;

use super::linestring::LineStringValidationExt;

pub trait PolygonValidationExt {
    fn all_points_are_valid(&self) -> bool;
    fn rings_closed(&self) -> bool;
    fn rings_size(&self) -> bool;
    fn is_valid(&self) -> bool;
}
impl PolygonValidationExt for Polygon {
    fn all_points_are_valid(&self) -> bool {
        if !self.exterior().is_valid() {
            return false;
        }
        if !self.interiors().iter().all(|i| i.is_valid()) {
            return false;
        }
        return true;
    }
    fn rings_closed(&self) -> bool {
        if !self.exterior().is_closed() {
            return false;
        }
        for interior in self.interiors() {
            if !interior.is_closed() {
                return false;
            }
        }
        return true;
    }
    fn rings_size(&self) -> bool {
        if !self.exterior().check_point_ring_size() {
            return false;
        }
        for interior in self.interiors() {
            if !interior.check_point_ring_size() {
                return false;
            }
        }
        return true;
    }
    fn is_valid(&self) -> bool {
        if !self.all_points_are_valid() {
            return false;
        };
        if !self.rings_closed() {
            return false;
        };
        if !self.rings_size() {
            return false;
        };
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::LineString;
    #[test]
    fn all_points_are_valid() {
        let polygon = Polygon::new(LineString::from(vec![(0., 0.), (1., 1.), (1., 0.)]), vec![]);
        assert_eq!(polygon.is_valid(), true)
    }
}
