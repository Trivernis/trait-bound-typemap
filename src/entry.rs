use multi_trait_object::MultitraitObject;
use std::any::TypeId;
use std::marker::PhantomData;

pub struct TypeMapEntry<T> {
    pub type_id: TypeId,
    pub mto: MultitraitObject,
    _marker: PhantomData<T>,
}

impl<T> TypeMapEntry<T> {
    pub fn new(type_id: TypeId, mto: MultitraitObject) -> Self {
        Self {
            type_id,
            mto,
            _marker: PhantomData,
        }
    }
}
