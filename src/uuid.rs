//! uuid type

use crate::IntoDart;

impl IntoDart for uuid::Uuid {
    /// delegate to `[u8;16]` implementation
    fn into_dart(self) -> crate::ffi::DartCObject {
        (*self.as_bytes()).into_dart()
    }
}
