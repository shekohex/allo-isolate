//! uuid type

use crate::{
    ffi::{DartCObject, DartHandleFinalizer, DartTypedDataType},
    into_dart::{free_zero_copy_buffer_u8, DartTypedDataTypeTrait},
    IntoDart,
};

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

impl IntoDart for Vec<uuid::Uuid> {
    /// ⚠️ concatenated in a single `Vec<u8>` for performance optimization
    ///
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart List<[UuidValue](https://pub.dev/documentation/uuid/latest/uuid/UuidValue-class.html)>
    ///   ```dart
    ///   return List<UuidValue>.generate(
    ///     raw.lengthInBytes / 16,
    ///     (int i) => UuidValue.fromByteList(Uint8List.view(raw.buffer, i * 16, 16)),
    ///     growable: false);
    ///   ```
    ///
    /// - hydrate into Rust Vec<[Uuid](uuid::Uuid)>
    ///   ```rust,ignore
    ///   raw
    ///   .as_slice()
    ///   .chunks(16)
    ///   .map(|x: &[u8]| uuid::Uuid::from_bytes(*<&[u8] as std::convert::TryInto<&[u8;16]>>::try_into(x).expect("invalid uuid slice")))
    ///   .collect();
    ///   ```
    ///
    /// note that buffer could end up being incomplete under the same conditions as of [std::io::Write::write](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write).
    fn into_dart(self) -> DartCObject {
        use std::io::Write;
        let mut buffer = Vec::<u8>::with_capacity(self.len() * 16);
        for id in self {
            let _ = buffer.write(id.as_bytes());
        }
        buffer.into_dart()
    }
}

impl<const N: usize> IntoDart for [uuid::Uuid; N] {
    fn into_dart(self) -> DartCObject {
        let vec: Vec<_> = self.into();
        vec.into_dart()
    }
}

impl DartTypedDataTypeTrait for uuid::Uuid {
    fn dart_typed_data_type() -> DartTypedDataType {
        DartTypedDataType::Uint8
    }

    fn function_pointer_of_free_zero_copy_buffer() -> DartHandleFinalizer {
        free_zero_copy_buffer_u8
    }
}
