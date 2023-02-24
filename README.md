# Geometry Validation for geo/geo-types

A pure rust module for validating geometries. Inspired by [Geos Geometry Validation Module](https://github.com/libgeos/geos/tree/0a55b3af552470754893485b51407cc0219dbaae/src/operation/valid)

## Usage
### Basic usage
```rust
use geo_types::{Point, Polygon};
use geo_valid::{validate, Validation};

let validation: Validation = validate(Point(1., 2.));
validation.is_valid;
// true

let self_intersecting_polygon = Polygon::new(
    LineString::from(vec![(0., 0.), (2., 2.), (2., 0.), (0., 2.)]),
    vec![],
);

let validaiton: Validaiton = validate(self_intersecting_polygon);
validation.is_valid;
// false
```