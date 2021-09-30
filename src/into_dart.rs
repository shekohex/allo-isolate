use std::{ffi::CString, mem::ManuallyDrop};

use crate::{dart_array::DartArray, ffi::*};

/// A trait to convert between Rust types and Dart Types that could then
/// be sent to the isolate
///
/// see: [`crate::Isolate::post`]
pub trait IntoDart {
    /// Consumes `Self` and Performs the conversion.
    fn into_dart(self) -> DartCObject;
}

impl<T> IntoDart for T
    where
        T: Into<DartCObject>,
{
    fn into_dart(self) -> DartCObject { self.into() }
}

impl IntoDart for () {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartNull,
            // I don't know what value should we send, so I send a false
            // I guess dart vm check for the type first, so if it is null it
            // return null and ignore the value
            value: DartCObjectValue { as_bool: false },
        }
    }
}

impl IntoDart for i32 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue { as_int32: self },
        }
    }
}

impl IntoDart for i64 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt64,
            value: DartCObjectValue { as_int64: self },
        }
    }
}

impl IntoDart for f32 {
    fn into_dart(self) -> DartCObject { (self as f64).into_dart() }
}

impl IntoDart for f64 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartDouble,
            value: DartCObjectValue { as_double: self },
        }
    }
}

impl IntoDart for bool {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartBool,
            value: DartCObjectValue { as_bool: self },
        }
    }
}

impl IntoDart for String {
    fn into_dart(self) -> DartCObject {
        let s = CString::new(self).unwrap_or_default();
        s.into_dart()
    }
}

impl IntoDart for &'_ str {
    fn into_dart(self) -> DartCObject { self.to_string().into_dart() }
}

impl IntoDart for CString {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartString,
            value: DartCObjectValue {
                as_string: self.into_raw(),
            },
        }
    }
}

impl IntoDart for Vec<u8> {
    fn into_dart(self) -> DartCObject {
        let mut vec = ManuallyDrop::new(self);
        let data = DartNativeTypedData {
            ty: DartTypedDataType::Uint8,
            length: vec.len() as isize,
            values: vec.as_mut_ptr(),
        };
        let value = DartCObjectValue {
            as_typed_data: data,
        };
        DartCObject {
            ty: DartCObjectType::DartTypedData,
            value,
        }
    }
}

impl IntoDart for Vec<i8> {
    fn into_dart(self) -> DartCObject {
        let mut vec = ManuallyDrop::new(self);
        let data = DartNativeTypedData {
            ty: DartTypedDataType::Int8,
            length: vec.len() as isize,
            values: vec.as_mut_ptr() as *mut _,
        };
        let value = DartCObjectValue {
            as_typed_data: data,
        };
        DartCObject {
            ty: DartCObjectType::DartTypedData,
            value,
        }
    }
}

impl IntoDart for ZeroCopyBuffer<Vec<u8>> {
    fn into_dart(self) -> DartCObject {
        let mut vec = ManuallyDrop::new(self.0);
        vec.shrink_to_fit();
        let length = vec.len();
        assert_eq!(length, vec.capacity());
        let ptr = vec.as_mut_ptr();

        DartCObject {
            ty: DartCObjectType::DartExternalTypedData,
            value: DartCObjectValue {
                as_external_typed_data: DartNativeExternalTypedData {
                    ty: DartTypedDataType::Uint8,
                    length: length as isize,
                    data: ptr,
                    peer: ptr,
                    callback: deallocate_rust_buffer,
                },
            },
        }
    }
}

impl<T> IntoDart for Vec<T>
    where
        T: IntoDart,
{
    fn into_dart(self) -> DartCObject { DartArray::from(self).into_dart() }
}

impl<T> IntoDart for Option<T>
    where
        T: IntoDart,
{
    fn into_dart(self) -> DartCObject {
        match self {
            Some(v) => v.into_dart(),
            None => ().into_dart(),
        }
    }
}

impl<T, E> IntoDart for Result<T, E>
    where
        T: IntoDart,
        E: ToString,
{
    fn into_dart(self) -> DartCObject {
        match self {
            Ok(v) => v.into_dart(),
            Err(e) => e.to_string().into_dart(),
        }
    }
}

/// A workaround to send raw pointers to dart over the port.
/// it will be sent as int64 on 64bit targets, and as int32 on 32bit targets.
#[cfg(target_pointer_width = "64")]
impl<T> IntoDart for *const T {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt64,
            value: DartCObjectValue {
                as_int64: self as _,
            },
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl<T> IntoDart for *mut T {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt64,
            value: DartCObjectValue {
                as_int64: self as _,
            },
        }
    }
}

#[cfg(target_pointer_width = "32")]
impl<T> IntoDart for *const T {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as _,
            },
        }
    }
}

#[cfg(target_pointer_width = "32")]
impl<T> IntoDart for *mut T {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as _,
            },
        }
    }
}
