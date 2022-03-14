use multi_trait_object::MultitraitObject;
use std::any::TypeId;
use std::collections::HashMap;

/// Base typemap used for implementation but not elsewhere
/// Each other typemap is just a newtype of this base type with
/// additional implementation.
#[doc(hidden)]
#[derive(Debug, Default)]
pub(crate) struct TypeMapBase(pub(crate) HashMap<TypeId, MultitraitObject>);

impl TypeMapBase {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<K: 'static>(&mut self, value: MultitraitObject) {
        self.0.insert(TypeId::of::<K>(), value);
    }

    #[inline]
    pub fn get<K: 'static>(&self) -> Option<&MultitraitObject> {
        self.0.get(&TypeId::of::<K>())
    }

    #[inline]
    pub fn get_mut<K: 'static>(&mut self) -> Option<&mut MultitraitObject> {
        self.0.get_mut(&TypeId::of::<K>())
    }

    #[inline]
    pub fn remove<K: 'static>(&mut self) -> Option<MultitraitObject> {
        self.0.remove(&TypeId::of::<K>())
    }

    #[inline]
    pub fn contains_key<K: 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<K>())
    }
}
