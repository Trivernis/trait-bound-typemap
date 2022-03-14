use crate::base::TypeMapBase;
use crate::{impl_typemap, TypeMapKey, TypedKeyMto, TypedKeyTrait};
use multi_trait_object::{create_object, MultitraitObject, RawClone, TryClone};

pub struct CloneTypeMapKey;

impl<T> TypedKeyTrait<CloneTypeMapKey> for T
where
    T: TypeMapKey,
    <T as TypeMapKey>::Value: TypedKeyMto<CloneTypeMapKey> + Clone,
{
    type Value = T::Value;
}

impl<T: 'static + Clone> TypedKeyMto<CloneTypeMapKey> for T {
    fn into_mto(self) -> MultitraitObject {
        create_object!(self, dyn RawClone)
    }
}

impl_typemap!(CloneTypeMap, CloneTypeMapKey);

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
