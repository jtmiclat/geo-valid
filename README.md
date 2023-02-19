# Geometry Validation for geo/geo-types

A pure rust module for validating geometries. Inspired by [Geos Geometry Validation Module](https://github.com/libgeos/geos/tree/0a55b3af552470754893485b51407cc0219dbaae/src/operation/valid)

## Usage
```rust
use geo_types::Point;
use geo_valid::PointValidationExt;

let is_valid = Point(1., 2.).is_valid();
```
## Notes:
### API Design
Currently, I've decided to use Trait Extensions for implementing the validation. This is was to mimic the shapley's validation api.

In shapely
```python
from shapely import Point

Point(1, 2).is_valid == True
```
In geo-valid
```rust
use geo_types::Point;
use geo_valid::PointValidationExt;

let is_valid: bool = Point(1, 2).is_valid();
```
but it might be better to use custom structs instead to be able to more robust error messages. For example
```rust
use geo_types::Point;
use geo_valid::{validate, Validation};
let validation: Validation = validate(Point(1., 2.));

validation.is_valid() == true;
validation.errors(); /// Returns a vector of string errors
```