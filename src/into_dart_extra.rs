//! The conversions in this file is not lossless. On the contrary, it is lossy
//! and the type that Dart receives will not be the same as the type you send in
//! Rust. For example, numeric types can become String.

use crate::{ffi::*, IntoDart};

impl IntoDart for u32 {
    fn into_dart(self) -> DartCObject { (self as i64).into_dart() }
}

impl IntoDart for u64 {
    fn into_dart(self) -> DartCObject { (self as i64).into_dart() }
}

impl IntoDart for i128 {
    fn into_dart(self) -> DartCObject { self.to_string().into_dart() }
}

impl IntoDart for u128 {
    fn into_dart(self) -> DartCObject { self.to_string().into_dart() }
}
