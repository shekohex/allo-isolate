//! chrono types
//!
//! based on Dart VM, microseconds unit is used

use crate::{ffi::DartCObject, IntoDart};

impl IntoDart for chrono::DateTime<chrono::Utc> {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [DateTime](https://api.flutter.dev/flutter/dart-core/DateTime/DateTime.fromMicrosecondsSinceEpoch.html)
    ///   ```dart,ignore
    ///   DateTime.fromMicrosecondsSinceEpoch(raw, isUtc: true);
    ///   ```
    ///
    /// - hydrate into Rust [DateTime](chrono::DateTime)::<[Utc](chrono::Utc)>
    ///   ```rust,ignore
    ///   chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(0, raw * 1_000), chrono::Utc);
    ///   ```
    fn into_dart(self) -> DartCObject { self.timestamp_micros().into_dart() }
}

impl IntoDart for chrono::DateTime<chrono::Local> {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [DateTime](https://api.flutter.dev/flutter/dart-core/DateTime/DateTime.fromMicrosecondsSinceEpoch.html)
    ///   ```dart,ignore
    ///   DateTime.fromMicrosecondsSinceEpoch(raw, isUtc: false);
    ///   ```
    ///
    /// - hydrate into Rust [DateTime](chrono::DateTime)::<[Local](chrono::Local)>
    ///   ```rust,ignore
    ///   chrono::DateTime::<chrono::Local>::from_local(chrono::NaiveDateTime::from_timestamp(0, raw * 1_000), chrono::Local);
    ///   ```
    fn into_dart(self) -> DartCObject { self.timestamp_micros().into_dart() }
}

impl IntoDart for chrono::NaiveDateTime {
    /// on the other side of FFI, value should be reconstructed like [DateTime](chrono::DateTime)::<[Local](chrono::Local)>
    fn into_dart(self) -> DartCObject { self.timestamp_micros().into_dart() }
}

impl IntoDart for chrono::Duration {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [Duration](https://api.flutter.dev/flutter/dart-core/Duration/Duration.html)
    ///   ```dart,ignore
    ///   Duration(microseconds: raw);
    ///   ```
    ///
    /// - hydrate into Rust [Duration](chrono::Duration)
    ///   ```rust,ignore
    ///   chrono::Duration::from_microseconds(raw);
    ///   ```
    fn into_dart(self) -> DartCObject { self.num_microseconds().into_dart() }
}
