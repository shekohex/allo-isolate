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

impl IntoDart for Vec<uuid::Uuid> {
    /// delegate to `Vec<u8>` implementation but concatenates outputs in a single Vec<u8>
    ///
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart List<[UuidValue](https://pub.dev/documentation/uuid/latest/uuid/UuidValue-class.html)>
    ///   ```dart
    ///   final count = raw.lengthInBytes / 16;
    ///   var List<UuidValue> ids = List(growable: true);
    ///   for (var i = 0; i < count; i += 16) {
    ///     ids.add(UuidValue.fromByteList(Uint8List.view(raw.buffer, i * 16, 16)))
    ///   }
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
    fn into_dart(self) -> DartCObject {
        self.into_iter()
            .map(|x| x.as_bytes().to_vec())
            .flatten()
            .collect::<Vec<u8>>()
            .into_dart()
    }
}
