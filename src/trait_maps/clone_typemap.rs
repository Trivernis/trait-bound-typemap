use crate::base::TypeMapBase;
use crate::impl_typemap;
use multi_trait_object::{RawClone, TryClone};

impl_typemap!(
    /// A typemap that can be cloned restricting all inner
    /// types to implement [std::clone::Clone] as well.
    CloneTypeMap,
    CloneTypeMapKey,
    RawClone
);

impl Clone for CloneTypeMap {
    fn clone(&self) -> Self {
        let map = self
            .0
             .0
            .iter()
            .map(|(t, o)| (t.clone(), o.try_clone().unwrap()))
            .collect();
        CloneTypeMap(TypeMapBase(map))
    }
}
