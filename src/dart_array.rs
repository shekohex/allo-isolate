//! A FFI Compatible Array for Dart

use core::slice;
use std::mem::ManuallyDrop;

use ffi::{DartCObject, DartCObjectType, DartCObjectValue, DartNativeArray};

use crate::{ffi, IntoDart};

/// A wrapper around a list of `DartCObject` that will be dropped after been
/// sent to dart vm.
#[derive(Debug, Clone)]
pub struct DartArray {
    inner: Box<[*mut DartCObject]>,
}

impl<T: IntoDart + Copy, const N: usize> From<[T; N]> for DartArray {
    fn from(vec: [T; N]) -> Self {
        let vec: Vec<_> = vec.
            iter()
            // convert them to dart objects
            .map(|&x| IntoDart::into_dart(x))
            // box them to be transferred to dart
            .map(Box::new)
            // as pointers
            .map(Box::into_raw)
            // then collect everything
            .collect();
        let inner = vec.into_boxed_slice();
        Self { inner }
    }
}

impl<T: IntoDart> From<Vec<T>> for DartArray {
    fn from(vec: Vec<T>) -> Self {
        let vec: Vec<_> = vec.
            into_iter()
            // convert them to dart objects
            .map(IntoDart::into_dart)
            // box them to be transferred to dart
            .map(Box::new)
            // as pointers
            .map(Box::into_raw)
            // then collect everything
            .collect();
        let inner = vec.into_boxed_slice();
        Self { inner }
    }
}

impl IntoDart for DartArray {
    fn into_dart(self) -> ffi::DartCObject {
        let mut s = ManuallyDrop::new(self);
        // we drop vec when DartCObject get dropped
        let (data, len) = (s.inner.as_mut_ptr(), s.inner.len());

        let array = DartNativeArray {
            length: len as isize,
            values: data,
        };
        DartCObject {
            ty: DartCObjectType::DartArray,
            value: DartCObjectValue { as_array: array },
        }
    }
}

impl From<DartNativeArray> for DartArray {
    fn from(arr: DartNativeArray) -> Self {
        let inner = unsafe {
            let slice =
                slice::from_raw_parts_mut(arr.values, arr.length as usize);
            Box::from_raw(slice)
        };
        Self { inner }
    }
}

impl Drop for DartArray {
    fn drop(&mut self) {
        for v in self.inner.iter() {
            unsafe {
                Box::from_raw(*v);
            }
        }
    }
}
