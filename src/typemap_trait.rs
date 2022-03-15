use multi_trait_object::MultitraitObject;
use std::any::Any;

/// A trait that allows using the object implementing it
/// to be used as a type key.
pub trait TypeMapKey: 'static {
    type Value: Any;
}

/// A trait used for restricting values inserted in a type map
/// using type checking
#[doc(hidden)]
pub trait TypedKeyTrait<T>: 'static {
    type Value: TypedKeyMto<T>;
}

/// A trait used to create a multitrait-object from a given
/// value with the given guaranteed trait implementations
#[doc(hidden)]
pub trait TypedKeyMto<T> {
    fn into_mto(self) -> MultitraitObject;
}

/// A trait to map the key to the map it describes
#[doc(hidden)]
pub trait MapKey {
    type Map: TypeMapTrait<Key = Self>;
}

/// A trait implemented by all typemaps that provides
/// all basic typemap functions
pub trait TypeMapTrait {
    type Key;

    /// Creates a new typemap
    fn new() -> Self;

    /// Inserts a value into the typemap with the given key
    fn insert<T: TypedKeyTrait<Self::Key>>(&mut self, value: T::Value);

    /// Returns a reference to a value from the type map with the given provided key
    fn get<T: TypedKeyTrait<Self::Key>>(&self) -> Option<&T::Value>;

    /// Returns a mutable reference to a value from the type map with the given provided key
    fn get_mut<T: TypedKeyTrait<Self::Key>>(&mut self) -> Option<&mut T::Value>;

    /// Removes a value from the map for the given key
    fn remove<T: TypedKeyTrait<Self::Key>>(&mut self) -> Option<T::Value>;

    /// Returns if the map contains a given key
    fn contains_key<T: TypedKeyTrait<Self::Key>>(&self) -> bool;
}
