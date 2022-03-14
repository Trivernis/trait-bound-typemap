use crate::{impl_typemap, TypeMapKey, TypedKeyMto, TypedKeyTrait};
use multi_trait_object::{create_object, MultitraitObject};
use std::any::Any;

#[derive(Eq, PartialEq, Hash)]
pub struct AnyTypeMapKey;

impl<T> TypedKeyTrait<AnyTypeMapKey> for T
where
    T: TypeMapKey,
    <T as TypeMapKey>::Value: Any,
{
    type Value = T::Value;
}

impl<T: 'static> TypedKeyMto<AnyTypeMapKey> for T {
    fn into_mto(self) -> MultitraitObject {
        create_object!(self, dyn Any)
    }
}

impl_typemap!(TypeMap, AnyTypeMapKey);
