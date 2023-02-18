use geo::{
    algorithm::line_intersection::line_intersection, Coord, GeoFloat, Line, LineIntersection,
};
use geo_types::Polygon;

use super::linestring::LineStringValidationExt;

pub trait LineIntersectionExt {
    fn single_point(&self) -> Option<Coord>;
}
impl<F: GeoFloat> LineIntersectionExt for LineIntersection<F> {
    fn single_point(&self) -> Option<Coord> {
        match self {
            Self::Collinear { .. } => None,
            Self::SinglePoint { intersection, .. } => Some(Coord::from((
                intersection.x.to_f64()?,
                intersection.y.to_f64()?,
            ))),
        }
    }
}

pub trait PolygonValidationExt {
    fn all_points_are_valid(&self) -> bool;
    fn rings_closed(&self) -> bool;
    fn rings_size(&self) -> bool;
    fn is_valid(&self) -> bool;
    fn get_self_intersection(&self) -> Option<Coord>;
}
impl PolygonValidationExt for Polygon {
    fn get_self_intersection(&self) -> Option<Coord> {
        let exterior = self.exterior();
        let lines: Vec<Line> = exterior.lines().collect();
        for line in lines.clone() {
            for line2 in lines.clone() {
                //skip if comparing the same part
                if line == line2 {
                    continue;
                }
                if let Some(intersection) = line_intersection(line, line2) {
                    if intersection.is_proper() {
                        if let Some(point) = intersection.single_point() {
                            return Some(point);
                        }
                    }
                }
            }
        }

        None
    }
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
    use geo::coord;
    use geo_types::LineString;
    #[test]
    fn all_points_are_valid() {
        let polygon = Polygon::new(LineString::from(vec![(0., 0.), (1., 1.), (1., 0.)]), vec![]);
        assert_eq!(polygon.is_valid(), true)
    }
    #[test]
    fn self_intersecting_polygon() {
        let polygon = Polygon::new(
            LineString::from(vec![(0., 0.), (2., 2.), (2., 0.), (0., 2.)]),
            vec![],
        );
        assert_eq!(polygon.get_self_intersection(), Some(coord! {x:1.0, y:1.0}))
    }
}
