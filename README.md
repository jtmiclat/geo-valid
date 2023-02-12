# Geometry Validation based on GEOS

A pure rust module for validating geometryies. Inspired by [Geos Geometry Validation Module](https://github.com/libgeos/geos/tree/0a55b3af552470754893485b51407cc0219dbaae/src/operation/valid)

## Usage
```rust
use geotypes::Point;
use geo_valid::PointValidationExt;

let is_valid = Point(.1, .2).is_valid();
```