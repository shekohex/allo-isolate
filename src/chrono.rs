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
    ///   let s = (raw / 1_000_000) as i64;
    ///   let ns = (raw.rem_euclid(1_000_000) * 1_000) as u32;
    ///   chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Utc);
    ///   ```
    ///
    ///   note that it could overflow under the same conditions as of [chrono::NaiveDateTime::from_timestamp](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp)
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
    ///   let s = (raw / 1_000_000) as i64;
    ///   let ns = (raw.rem_euclid(1_000_000) * 1_000) as u32;
    ///   chrono::DateTime::<chrono::Local>::from_local(chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Local);
    ///   ```
    ///
    ///   note that it could overflow under the same conditions as of [chrono::NaiveDateTime::from_timestamp](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp)
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
