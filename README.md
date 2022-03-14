# Trait bound Typemap [![](https://img.shields.io/crates/v/trait-bound-typemap)](https://crates.io/crates/trait-bound-typemap) [![](https://img.shields.io/docsrs/trait-bound-typemap)](https://docs.rs/trait-bound-typemap)

This crate offers typemaps that restrict a given type in their
trait and therefore offer additional trait implementations such as `Clone` and `PartialEq`.


## Safety

This crate relies on the [multi-trait-object](https://crates.io/crates/multi-trait-object) crate
which provides a workaround for storing a type erased object with all associated traits until
this feature is implemented in the language itself. This crate will likely break when the
fat pointer used by trait objects changes which it hasn't in a long time so far. 


## Usage

```rust
use trait_bound_typemap::{CloneTypeMap, TypeMapTrait, TypeMapKey};

#[derive(Clone)]
pub struct MyStruct {
    a: u8,
    b: String,
}

pub struct MyStructKey;

impl TypeMapKey for MyStructKey {
    type Value = MyStruct;
}

fn main() {
    let mut map = CloneTypeMap::new();
    let value = MyStruct {a: 5, b: String::from("Hello World")};
    map.insert::<MyStructKey>(value);
    assert!(map.contains_key::<MyStructKey>());
}
```

## License

Apache-2.0