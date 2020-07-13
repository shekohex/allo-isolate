use crate::{dart_array::DartArray, ffi::*};
use std::ffi::CString;

/// A trait to convert between Rust types and Dart Types that could then
/// be sended to the isolate
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

impl IntoDart for u8 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as i32,
            },
        }
    }
}

impl IntoDart for i8 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as i32,
            },
        }
    }
}

impl IntoDart for u16 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as i32,
            },
        }
    }
}

impl IntoDart for i16 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as i32,
            },
        }
    }
}

impl IntoDart for u32 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt32,
            value: DartCObjectValue {
                as_int32: self as i32,
            },
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

impl IntoDart for u64 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartInt64,
            value: DartCObjectValue {
                as_int64: self as i64,
            },
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

impl IntoDart for i128 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartString,
            value: DartCObjectValue {
                // the value of CString get droped when we drop DartCObject
                // and that is happen after we send the message to the dart vm.
                as_string: CString::new(self.to_string())
                    // this safe, since i128 can be converted to string
                    .unwrap_or_default()
                    .into_raw(),
            },
        }
    }
}

impl IntoDart for u128 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartString,
            value: DartCObjectValue {
                as_string: CString::new(self.to_string())
                    // this safe, since u128 can be converted to string
                    .unwrap_or_default()
                    .into_raw(),
            },
        }
    }
}

impl IntoDart for f32 {
    fn into_dart(self) -> DartCObject {
        DartCObject {
            ty: DartCObjectType::DartDouble,
            value: DartCObjectValue {
                as_double: self as f64,
            },
        }
    }
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
            None => DartCObject {
                ty: DartCObjectType::DartNull,
                // I don't know what value should we send, so I send a false
                // I guess dart vm check for the type first, so if it is null it
                // return null and ignore the value
                value: DartCObjectValue { as_bool: false },
            },
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
