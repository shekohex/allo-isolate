//! chrono types
//!
//! based on Dart VM, microseconds unit is used
//!
//! recommendations below implies UTC based conversions,
//! as these are generally easier to work with.
//! > see [timestamp_micros](https://docs.rs/chrono/0.4.20/chrono/naive/struct.NaiveDateTime.html?search=timestamp_micros#method.timestamp_micros)

use crate::{ffi::DartCObject, IntoDart};

impl IntoDart for chrono::DateTime<chrono::Utc> {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [DateTime](https://api.dart.dev/stable/2.18.0/dart-core/DateTime/DateTime.fromMicrosecondsSinceEpoch.html)
    ///   `DateTime.fromMicrosecondsSinceEpoch(raw, isUtc: true);`
    ///
    /// - hydrate into Rust [DateTime](chrono::DateTime)::<[Utc](chrono::Utc)>
    ///   ```rust,ignore
    ///   let s = (raw / 1_000_000) as i64;
    ///   let ns = (raw.rem_euclid(1_000_000) * 1_000) as u32;
    ///   chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Utc);
    ///   ```
    ///
    ///   note that it could overflow under the same conditions as of [chrono::NaiveDateTime::from_timestamp](https://docs.rs/chrono/0.4.20/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp)
    fn into_dart(self) -> DartCObject {
        self.timestamp_micros().into_dart()
    }
}

impl IntoDart for chrono::DateTime<chrono::Local> {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [DateTime](https://api.dart.dev/stable/2.18.0/dart-core/DateTime/DateTime.fromMicrosecondsSinceEpoch.html)
    ///   `DateTime.fromMicrosecondsSinceEpoch(raw, isUtc: false);`
    ///
    /// - hydrate into Rust [DateTime](chrono::DateTime)::<[Local](chrono::Local)>
    ///   ```rust,ignore
    ///   let s = (raw / 1_000_000) as i64;
    ///   let ns = (raw.rem_euclid(1_000_000) * 1_000) as u32;
    ///   chrono::DateTime::<chrono::Local>::from(
    ///     chrono::DateTime::<chrono::Utc>::from_utc(
    ///       chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Utc));
    ///   ```
    ///
    ///   note that it could overflow under the same conditions as of [chrono::NaiveDateTime::from_timestamp](https://docs.rs/chrono/0.4.20/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp)
    fn into_dart(self) -> DartCObject {
        self.timestamp_micros().into_dart()
    }
}

impl IntoDart for chrono::NaiveDateTime {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [DateTime](https://api.dart.dev/stable/2.18.0/dart-core/DateTime/DateTime.fromMicrosecondsSinceEpoch.html)
    ///   `DateTime.fromMicrosecondsSinceEpoch(raw, isUtc: true);`
    ///
    /// - hydrate into Rust [NaiveDateTime](chrono::NaiveDateTime)
    ///   ```rust,ignore
    ///   let s = (raw / 1_000_000) as i64;
    ///   let ns = (raw.rem_euclid(1_000_000) * 1_000) as u32;
    ///   chrono::NaiveDateTime::from_timestamp(s, ns)
    ///   ```
    ///
    ///   note that it could overflow under the same conditions as of [chrono::NaiveDateTime::from_timestamp](https://docs.rs/chrono/0.4.20/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp)
    fn into_dart(self) -> DartCObject {
        self.timestamp_micros().into_dart()
    }
}

impl IntoDart for chrono::Duration {
    /// on the other side of FFI, value should be reconstructed like:
    ///
    /// - hydrate into Dart [Duration](https://api.dart.dev/stable/2.18.0/dart-core/Duration/Duration.html)
    ///   `Duration(microseconds: raw);`
    ///
    /// - hydrate into Rust [Duration](chrono::Duration)
    /// `chrono::Duration::microseconds(raw);`
    fn into_dart(self) -> DartCObject {
        self.num_microseconds().into_dart()
    }
}

impl IntoDart for Vec<chrono::DateTime<chrono::Utc>> {
    fn into_dart(self) -> DartCObject {
        self.iter()
            .map(chrono::DateTime::<chrono::Utc>::timestamp_micros)
            .collect::<Vec<_>>()
            .into_dart()
    }
}

impl<const N: usize> IntoDart for [chrono::DateTime<chrono::Utc>; N] {
    fn into_dart(self) -> DartCObject {
        let vec: Vec<_> = self.into();
        vec.into_dart()
    }
}

impl IntoDart for Vec<chrono::DateTime<chrono::Local>> {
    fn into_dart(self) -> DartCObject {
        self.iter()
            .map(chrono::DateTime::<chrono::Local>::timestamp_micros)
            .collect::<Vec<_>>()
            .into_dart()
    }
}

impl<const N: usize> IntoDart for [chrono::DateTime<chrono::Local>; N] {
    fn into_dart(self) -> DartCObject {
        let vec: Vec<_> = self.into();
        vec.into_dart()
    }
}

impl IntoDart for Vec<chrono::NaiveDateTime> {
    fn into_dart(self) -> DartCObject {
        self.iter()
            .map(chrono::NaiveDateTime::timestamp_micros)
            .collect::<Vec<_>>()
            .into_dart()
    }
}

impl<const N: usize> IntoDart for [chrono::NaiveDateTime; N] {
    fn into_dart(self) -> DartCObject {
        let vec: Vec<_> = self.into();
        vec.into_dart()
    }
}

impl IntoDart for Vec<chrono::Duration> {
    fn into_dart(self) -> DartCObject {
        self.iter()
            .map(chrono::Duration::num_microseconds)
            .collect::<Vec<_>>()
            .into_dart()
    }
}

impl<const N: usize> IntoDart for [chrono::Duration; N] {
    fn into_dart(self) -> DartCObject {
        let vec: Vec<_> = self.into();
        vec.into_dart()
    }
}
