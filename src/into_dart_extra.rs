//! The conversions in this file is not lossless. On the contrary, it is lossy
//! and the type that Dart receives will not be the same as the type you send in
//! Rust. For example, numeric types can become String.

use crate::{ffi::*, IntoDart};

impl IntoDart for i8 {
    fn into_dart(self) -> DartCObject {
        (self as i32).into_dart()
    }
}

impl IntoDart for i16 {
    fn into_dart(self) -> DartCObject {
        (self as i32).into_dart()
    }
}

impl IntoDart for u8 {
    fn into_dart(self) -> DartCObject {
        (self as i32).into_dart()
    }
}

impl IntoDart for u16 {
    fn into_dart(self) -> DartCObject {
        (self as i32).into_dart()
    }
}

impl IntoDart for u32 {
    fn into_dart(self) -> DartCObject {
        (self as i64).into_dart()
    }
}

impl IntoDart for u64 {
    fn into_dart(self) -> DartCObject {
        (self as i64).into_dart()
    }
}

impl IntoDart for i128 {
    fn into_dart(self) -> DartCObject {
        self.to_string().into_dart()
    }
}

impl IntoDart for u128 {
    fn into_dart(self) -> DartCObject {
        self.to_string().into_dart()
    }
}

#[cfg(target_pointer_width = "64")]
impl IntoDart for usize {
    fn into_dart(self) -> DartCObject {
        (self as u64 as i64).into_dart()
    }
}

#[cfg(target_pointer_width = "32")]
impl IntoDart for usize {
    fn into_dart(self) -> DartCObject {
        (self as u32 as i32).into_dart()
    }
}
