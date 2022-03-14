use crate::impl_typemap;
use std::any::Any;

impl_typemap!(
    /// A typemap that can store any type (implementing [std::any::Any]).
    TypeMap,
    AnyTypeMapKey,
    Any
);
