# ord_subset [![Build Status](https://travis-ci.org/Emerentius/ord_subset.svg?branch=master)](https://travis-ci.org/Emerentius/ord_subset)

Crate for working with the `Ord` subset of certain types like `f32`, `f64`.



Documentation is up at https://docs.rs/ord_subset/.

# Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
ord_subset = "2"
```

```rust
extern crate ord_subset;
use ord_subset::{OrdSubsetIterExt, OrdSubsetSliceExt};

fn main() {
  // Slices. Works on vector, too.
  let mut s = [5.0, std::f64::NAN, 3.0, 2.0];
  s.ord_subset_sort();
  assert_eq!(&s[0..3], &[2.0, 3.0, 5.0]);
  assert_eq!(s.ord_subset_binary_search(&5.0), Ok(2));

  // iterators
  assert_eq!( s.iter().ord_subset_max(), Some(&5.0) );
  assert_eq!( s.iter().ord_subset_min(), Some(&2.0) );
}
```

# License
Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed
except according to those terms.
