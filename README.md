# Trait bound Typemap

This crate offers typemaps that restrict a given type in their
trait and therefore offer additional trait implementations such as `Clone` and `PartialEq`.

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