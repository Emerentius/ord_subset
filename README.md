# almost_ord

Crate for working with the `Ord` subset of certain types like `f32`, `f64`.

Documentation is up at https://emerentius.github.io/almost_ord.

# Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
almost_ord = "~0.1.0"
```

```rust
extern crate almost_ord;
use almost_ord::{AlmostOrdIterExt, AlmostOrdSliceExt};

fn main() {
  // Slices. Works on vector, too.
  let mut s = [5.0, std::f64::NAN, 3.0, 2.0];
  s.partial_sort();
  assert_eq!(&s[0..3], &[2.0, 3.0, 5.0]);
  assert_eq!(s.partial_binary_search(&5.0), Ok(2));

  // iterators
  assert_eq!( s.iter().partial_max(), Some(&5.0) );
  assert_eq!( s.iter().partial_min(), Some(&2.0) );
}
```

# License
Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed
except according to those terms.
