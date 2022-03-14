#[doc(hidden)]
#[macro_export]
macro_rules! impl_typemap {
    ($map:ident, $key:ty) => {
        pub struct $map($crate::base::TypeMapBase);

        impl $crate::typemap_trait::TypeMapTrait for $map {
            type Key = $key;

            #[inline]
            fn new() -> Self {
                Self($crate::base::TypeMapBase::new())
            }

            #[inline]
            fn insert<T: $crate::typemap_trait::TypedKeyTrait<Self::Key>>(
                &mut self,
                value: T::Value,
            ) {
                let mto = value.into_mto();
                self.0.insert::<T>(mto);
            }

            #[inline]
            fn get<T: $crate::typemap_trait::TypedKeyTrait<Self::Key>>(&self) -> Option<&T::Value> {
                self.0.get::<T>().and_then(|v| v.downcast_ref())
            }

            #[inline]
            fn get_mut<T: $crate::typemap_trait::TypedKeyTrait<Self::Key>>(
                &mut self,
            ) -> Option<&mut T::Value> {
                self.0.get_mut::<T>().and_then(|v| v.downcast_mut())
            }

            #[inline]
            fn remove<T: $crate::typemap_trait::TypedKeyTrait<Self::Key>>(
                &mut self,
            ) -> Option<T::Value> {
                self.0.remove::<T>().and_then(|v| v.downcast())
            }

            #[inline]
            fn contains_key<T: $crate::typemap_trait::TypedKeyTrait<Self::Key>>(&self) -> bool {
                self.0.contains_key::<T>()
            }
        }
    };
}
