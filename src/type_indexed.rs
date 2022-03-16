use std::any::TypeId;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

/// Base typemap used for implementation but not elsewhere
/// Each other typemap is just a newtype of this base type with
/// additional implementation.
#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct TypeIndexedMap<T>(pub(crate) HashMap<TypeId, T>);

impl<T> Default for TypeIndexedMap<T> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<T> TypeIndexedMap<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<K: 'static>(&mut self, value: T) {
        self.0.insert(TypeId::of::<K>(), value);
    }

    #[inline]
    pub fn get<K: 'static>(&self) -> Option<&T> {
        self.0.get(&TypeId::of::<K>())
    }

    #[inline]
    pub fn get_mut<K: 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<K>())
    }

    #[inline]
    pub fn remove<K: 'static>(&mut self) -> Option<T> {
        self.0.remove(&TypeId::of::<K>())
    }

    #[inline]
    pub fn contains_key<K: 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<K>())
    }
}

impl<T> IntoIterator for TypeIndexedMap<T> {
    type Item = (TypeId, T);
    type IntoIter = IntoIter<TypeId, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Extend<(TypeId, T)> for TypeIndexedMap<T> {
    fn extend<I: IntoIterator<Item = (TypeId, T)>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}
