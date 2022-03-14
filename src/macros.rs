/// Macro to create a new trait bound typemap
#[doc(hidden)]
#[macro_export]
macro_rules! impl_typemap {
    ($( #[$outer:meta] )*
        $map:ident, $key:ident, $($trt:ident), +) => {
        $( #[$outer] )*
        pub struct $map($crate::base::TypeMapBase);
        $crate::impl_typekey!($key, $( $trt )+);

        impl $crate::TypeMapTrait for $map {
            type Key = $key;

            #[inline]
            fn new() -> Self {
                Self($crate::base::TypeMapBase::new())
            }

            #[inline]
            fn insert<T: $crate::TypedKeyTrait<Self::Key>>(
                &mut self,
                value: T::Value,
            ) {
                let mto = <T::Value as $crate::TypedKeyMto<Self::Key>>::into_mto(value);
                self.0.insert::<T>(mto);
            }

            #[inline]
            fn get<T: $crate::TypedKeyTrait<Self::Key>>(&self) -> Option<&T::Value> {
                self.0.get::<T>().and_then(|v| v.downcast_ref())
            }

            #[inline]
            fn get_mut<T: $crate::TypedKeyTrait<Self::Key>>(
                &mut self,
            ) -> Option<&mut T::Value> {
                self.0.get_mut::<T>().and_then(|v| v.downcast_mut())
            }

            #[inline]
            fn remove<T: $crate::TypedKeyTrait<Self::Key>>(
                &mut self,
            ) -> Option<T::Value> {
                self.0.remove::<T>().and_then(|v| v.downcast())
            }

            #[inline]
            fn contains_key<T: $crate::TypedKeyTrait<Self::Key>>(&self) -> bool {
                self.0.contains_key::<T>()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_typekey {
    ($key:ident, $($trt:ident), +) => {
        #[doc(hidden)]
        pub struct $key;

        impl<T> $crate::TypedKeyTrait<$key> for T
        where
            T: $crate::TypeMapKey,
            <T as $crate::TypeMapKey>::Value: $crate::TypedKeyMto<$key> $(+ $trt )+,
        {
            type Value = T::Value;
        }

        impl<T: 'static $(+ $trt )+> $crate::TypedKeyMto<$key> for T {
            fn into_mto(self) -> multi_trait_object::MultitraitObject {
                multi_trait_object::create_object!(self $(, dyn $trt )+)
            }
        }
    }
}
