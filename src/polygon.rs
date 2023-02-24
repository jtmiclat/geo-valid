use geo::coords_iter::CoordsIter;
use geo::line_intersection::line_intersection;
use geo::{Line, LineIntersection, Polygon};

use crate::linestring::validate_linestring_points;

use super::validator::Validation;

fn validate_polygon_points(polygon: &Polygon) -> Validation {
    let mut errors: Vec<String> = vec![];
    let exterior_validation = validate_linestring_points(polygon.exterior());
    if exterior_validation.is_valid == false {
        errors.extend(exterior_validation.errors)
    }
    for interior in polygon.interiors() {
        let interior_validation = validate_linestring_points(interior);
        if interior_validation.is_valid == false {
            errors.extend(interior_validation.errors)
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
fn validate_polygon_rings_length(polygon: &Polygon) -> Validation {
    let mut errors: Vec<String> = vec![];

    let exterior = polygon.exterior();
    if exterior.coords_count() < 3 {
        let error_message = format!("Exterior Ring {:?} must contain 3 or more coords", exterior);
        errors.push(error_message);
    }
    for interior in polygon.interiors() {
        if interior.coords_count() < 3 {
            let error_message =
                format!("Interior Ring {:?} must contain 3 or more coords", interior);
            errors.push(error_message);
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}

fn validate_polygon_rings_closed(polygon: &Polygon) -> Validation {
    let mut errors: Vec<String> = vec![];
    let exterior = polygon.exterior();
    if !exterior.is_closed() {
        let error_message = format!("Exterior ring {:?} is not closed", exterior);
        errors.push(error_message);
    }
    for interior in polygon.interiors() {
        if !interior.is_closed() {
            let error_message = format!("Interior ring {:?} is not closed", interior);
            errors.push(error_message);
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
fn validate_self_intersection(polygon: &Polygon) -> Validation {
    let mut errors: Vec<String> = vec![];
    let exterior = polygon.exterior();
    let mut lines: Vec<Line> = vec![];

    lines.extend(exterior.lines());
    for interior in polygon.interiors() {
        lines.extend(interior.lines())
    }
    // Use index of the line to determine which parts we havent compared to yet
    for (index, line) in lines.clone().iter().enumerate() {
        for line2 in &lines.clone()[index + 1..] {
            if let Some(intersection) = line_intersection(*line, *line2) {
                let intersection_message = match intersection {
                    LineIntersection::Collinear { intersection } => {
                        Some(format!("Found collinear at {:?}", intersection))
                    }

                    LineIntersection::SinglePoint {
                        intersection,
                        is_proper: true,
                    } => Some(format!("Found self intersection at {:?}", intersection)),
                    _ => None,
                };
                if let Some(error_message) = intersection_message {
                    errors.push(error_message);
                }
            }
        }
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}
pub fn validate_polygon(polygon: Polygon) -> Validation {
    let mut errors: Vec<String> = vec![];

    let points_validation = validate_polygon_points(&polygon);
    if points_validation.is_valid == false {
        errors.extend(points_validation.errors)
    }
    // This validation might not be needed because generating polygons
    // with the geo library automatically closes linestrings if not empty
    let ring_closed_validation = validate_polygon_rings_closed(&polygon);
    if ring_closed_validation.is_valid == false {
        errors.extend(ring_closed_validation.errors)
    }
    let ring_length_validation = validate_polygon_rings_length(&polygon);
    if ring_length_validation.is_valid == false {
        errors.extend(ring_length_validation.errors)
    }
    let self_intersection_validation = validate_self_intersection(&polygon);
    if self_intersection_validation.is_valid == false {
        errors.extend(self_intersection_validation.errors)
    }
    return Validation {
        is_valid: errors.len() == 0,
        errors: errors,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    // use geo::coord;
    use geo_types::LineString;
    use std::f64;
    #[test]
    fn simple_polygon() {
        let exterior = LineString::from(vec![(0., 0.), (1., 1.), (1., 0.)]);
        let interiors = vec![];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, true);
        assert_eq!(validation.errors.len(), 0);
    }
    #[test]
    fn invalid_points() {
        let exterior = LineString::from(vec![(0., f64::NAN), (1., 1.), (1., 0.)]);
        let interiors = vec![];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 3);
    }
    #[test]
    fn invalid_exterior_ring_length() {
        let exterior = LineString::from(vec![(0., 0.)]);
        let interiors = vec![];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn invalid_interior_length() {
        let exterior = LineString::from(vec![(-10., -10.), (10., -10.), (10., 10.), (-10., 10.)]);
        let interior_1 = LineString::from(vec![(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)]);
        let interior_2 = LineString::from(vec![(5., -1.)]);
        let interiors = vec![interior_1, interior_2];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn self_intersecting_simple_polygon() {
        let exterior = LineString::from(vec![(0., 0.), (2., 2.), (2., 0.), (0., 2.)]);
        let interiors = vec![];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn self_intersecting_simple_polygon_spike() {
        let exterior = LineString::from(vec![(0., 0.), (2., 2.), (3., 3.), (2., 2.), (2., 0.)]);
        let interiors = vec![];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 1);
    }
    #[test]
    fn self_intersecting_with_interrrior() {
        let exterior = LineString::from(vec![(0., 0.), (2., 0.), (2., 2.), (0., 2.)]);
        let interior_1 = LineString::from(vec![(1., 1.), (2.5, 1.5), (1.5, 1.75)]);
        let interiors = vec![interior_1];
        let polygon = Polygon::new(exterior, interiors);
        let validation = validate_polygon(polygon);
        assert_eq!(validation.is_valid, false);
        assert_eq!(validation.errors.len(), 2);
    }
}
