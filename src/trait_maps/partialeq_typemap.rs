use crate::impl_typemap;
use multi_trait_object::{PartialEqAny, TryPartialEq};
impl_typemap!(
    /// A typemap that provides a PartialEq implementation
    PartialEqTypeMap,
    PartialEqTypeMapKey,
    PartialEqAny
);

impl PartialEq for PartialEqTypeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0
             .0
            .iter()
            .zip(other.0 .0.iter())
            .all(|(a, b)| a.0 == b.0 && a.1.try_eq(b.1).unwrap())
    }
}
