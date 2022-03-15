use crate::{AnyTypeMap, CloneTypeMap, PartialEqTypeMap, TypeMapKey, TypeMapTrait};

pub struct TestStructKey;
#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    a: u64,
    b: String,
}

impl Default for TestStruct {
    fn default() -> Self {
        Self {
            a: 46,
            b: String::from("Hello World"),
        }
    }
}

impl TypeMapKey for TestStructKey {
    type Value = TestStruct;
}

pub struct TestStruct2Key;
#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct2 {
    c: i64,
    d: bool,
}

impl Default for TestStruct2 {
    fn default() -> Self {
        Self { c: 12, d: true }
    }
}

impl TypeMapKey for TestStruct2Key {
    type Value = TestStruct2;
}

pub struct NoCloneStruct;

impl TypeMapKey for NoCloneStruct {
    type Value = NoCloneStruct;
}

pub struct NotInsertedKey;

impl TypeMapKey for NotInsertedKey {
    type Value = ();
}

#[test]
fn it_creates_any_maps() {
    let mut map = AnyTypeMap::new();
    map.insert::<NoCloneStruct>(NoCloneStruct);
    map.insert::<TestStructKey>(TestStruct::default());
    map.insert::<TestStruct2Key>(TestStruct2::default());
    assert!(map.contains_key::<NoCloneStruct>());
    assert!(map.contains_key::<TestStructKey>());
    assert!(map.contains_key::<TestStruct2Key>());
    assert_eq!(map.contains_key::<NotInsertedKey>(), false);
}

#[test]
fn it_creates_clonable_maps() {
    let mut map = CloneTypeMap::new();
    map.insert::<TestStructKey>(TestStruct::default());
    map.insert::<TestStruct2Key>(TestStruct2::default());
    assert!(map.contains_key::<TestStructKey>());
    assert!(map.contains_key::<TestStruct2Key>());
    assert_eq!(map.contains_key::<NotInsertedKey>(), false);
}

#[test]
fn it_creates_partial_eq_maps() {
    let mut map = PartialEqTypeMap::new();
    map.insert::<TestStructKey>(TestStruct::default());
    let mut map2 = PartialEqTypeMap::new();
    map2.insert::<TestStructKey>(TestStruct::default());
    assert_eq!(map, map2)
}

#[test]
fn it_converts() {
    let mut clone_map = CloneTypeMap::new();
    clone_map.insert::<TestStructKey>(TestStruct::default());
    assert!(clone_map.contains_key::<TestStructKey>());
    let any_map = AnyTypeMap::from_iter(clone_map);
    assert!(any_map.contains_key::<TestStructKey>());
}
