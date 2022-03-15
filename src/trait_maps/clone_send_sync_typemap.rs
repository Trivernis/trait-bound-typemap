use crate::impl_typemap;
use multi_trait_object::{MultitraitObject, RawClone, TryClone};
use std::any::TypeId;
use std::collections::HashMap;

impl_typemap!(
    /// A typemap that implements Clone and is Send + Sync
    CloneSendSyncTypeMap,
    CloneSendSyncTypeMapKey,
    RawClone,
    Send,
    Sync
);

impl Clone for CloneSendSyncTypeMap {
    fn clone(&self) -> Self {
        let map = self
            .0
             .0
            .iter()
            .map(|(tid, mto)| (tid.clone(), mto.try_clone().unwrap()))
            .collect::<HashMap<TypeId, MultitraitObject>>();

        Self(crate::type_indexed::TypeIndexedMap(map))
    }
}

unsafe impl Send for CloneSendSyncTypeMap {}
unsafe impl Sync for CloneSendSyncTypeMap {}
