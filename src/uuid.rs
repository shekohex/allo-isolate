//! uuid type

use crate::{ffi::DartCObject, IntoDart};

impl IntoDart for uuid::Uuid {
    /// delegate to `Vec<u8>` implementation
    /// 
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [UuidValue](https://pub.dev/documentation/uuid/latest/uuid/UuidValue-class.html)
    ///   `UuidValue.fromByteList(raw);`
    ///
    /// - hydrate into Rust [Uuid](uuid::Uuid)
    ///   ```rust,ignore
    ///   uuid::Uuid::from_bytes(*<&[u8] as std::convert::TryInto<&[u8;16]>>::try_into(raw.as_slice()).expect("invalid uuid slice"));
    ///   ```
    fn into_dart(self) -> DartCObject {
        self.as_bytes().to_vec().into_dart()
    }
}
