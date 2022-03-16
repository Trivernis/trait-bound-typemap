use crate::impl_typemap;
use std::any::Any;

impl_typemap!(
    /// A typemap that is Send and Sync
    SendSyncTypeMap,
    SendSyncTypeMapKey,
    Any,
    Send,
    Sync
);

unsafe impl Send for SendSyncTypeMap {}
unsafe impl Sync for SendSyncTypeMap {}
