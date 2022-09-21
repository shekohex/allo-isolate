//! uuid type

use crate::IntoDart;

impl IntoDart for uuid::Uuid {
    /// delegate to `Vec<u8>` implementation
    fn into_dart(self) -> crate::ffi::DartCObject {
        self.as_bytes().to_vec().into_dart()
    }
}
