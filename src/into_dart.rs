use std::{
    ffi::{c_void, CString},
    mem::ManuallyDrop,
};

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

pub(crate) trait DartTypedDataTypeVisitor {
    fn visit<T: DartTypedDataTypeTrait>(&self);
}

/// The Rust type for corresponding [DartTypedDataType]
pub(crate) trait DartTypedDataTypeTrait {
    fn dart_typed_data_type() -> DartTypedDataType;

    unsafe extern "C" fn deallocate_rust_zero_copy_buffer(
        isolate_callback_data: *mut c_void,
        peer: *mut c_void,
    );
}

macro_rules! dart_typed_data_type_trait_impl {
    ($($dart_type:path => $rust_type:ident),+) => {
        $(
            impl DartTypedDataTypeTrait for $rust_type {
                fn dart_typed_data_type() -> DartTypedDataType {
                    $dart_type
                }

                #[doc(hidden)]
                #[no_mangle]
                unsafe extern "C" fn deallocate_rust_zero_copy_buffer(
                    isolate_callback_data: *mut c_void,
                    peer: *mut c_void,
                ) {
                    let len = (isolate_callback_data as isize) as usize;
                    let ptr = peer as *mut $rust_type;
                    drop(Vec::from_raw_parts(ptr, len, len));
                }
            }
        )+

        pub(crate) fn visit_dart_typed_data_type<V: DartTypedDataTypeVisitor>(ty: DartTypedDataType, visitor: &V) {
            match ty {
                $(
                    $dart_type => visitor.visit::<$rust_type>(),
                )+
            }
        }
    }
}

dart_typed_data_type_trait_impl!(
    DartTypedDataType::Int8 => i8,
    DartTypedDataType::Uint8 => u8,
    DartTypedDataType::Int16 => i16,
    DartTypedDataType::Uint16 => u16,
    DartTypedDataType::Int32 => i32,
    DartTypedDataType::Uint32 => u32,
    DartTypedDataType::Int64 => i64,
    DartTypedDataType::Uint64 => u64,
    DartTypedDataType::Float32 => f32,
    DartTypedDataType::Float64 => f64
);

impl<T> IntoDart for Vec<T>
where
    T: DartTypedDataTypeTrait,
{
    fn into_dart(self) -> DartCObject {
        let mut vec = ManuallyDrop::new(self);
        let data = DartNativeTypedData {
            ty: T::dart_typed_data_type(),
            length: vec.len() as isize,
            values: vec.as_mut_ptr() as *mut _,
        };
        DartCObject {
            ty: DartCObjectType::DartTypedData,
            value: DartCObjectValue {
                as_typed_data: data,
            },
        }
    }
}

impl<T> IntoDart for ZeroCopyBuffer<Vec<T>>
where
    T: DartTypedDataTypeTrait,
{
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
                    ty: T::dart_typed_data_type(),
                    length: length as isize,
                    data: ptr,
                    peer: ptr,
                    callback: T::deallocate_rust_zero_copy_buffer,
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
