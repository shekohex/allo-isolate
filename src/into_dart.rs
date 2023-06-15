use std::{
    ffi::{c_void, CString},
    mem::ManuallyDrop,
};

use crate::{
    dart_array::DartArray,
    ffi::{DartHandleFinalizer, *},
};

/// A trait to convert between Rust types and Dart Types that could then
/// be sent to the isolate
///
/// see: [`crate::Isolate::post`]
pub trait IntoDart {
    /// Consumes `Self` and Performs the conversion.
    fn into_dart(self) -> DartCObject;
}

/// A trait that is [`IntoDart`] and is also not a primitive type. It is used to
/// avoid the ambiguity of whether types such as [`Vec<i32>`] should be
/// converted into [`Int32List`] or [`List<int>`]
pub trait IntoDartExceptPrimitive: IntoDart {}

impl<T> IntoDart for T
where
    T: Into<DartCObject>,
{
    fn into_dart(self) -> DartCObject {
        self.into()
    }
}

impl<T> IntoDartExceptPrimitive for T where T: IntoDart + Into<DartCObject> {}

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

#[cfg(feature = "anyhow")]
impl IntoDart for anyhow::Error {
    fn into_dart(self) -> DartCObject {
        format!("{:?}", self).into_dart()
    }
}

#[cfg(feature = "backtrace")]
impl IntoDart for backtrace::Backtrace {
    fn into_dart(self) -> DartCObject {
        format!("{:?}", self).into_dart()
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
    fn into_dart(self) -> DartCObject {
        (self as f64).into_dart()
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

impl IntoDartExceptPrimitive for String {}

impl IntoDart for &'_ str {
    fn into_dart(self) -> DartCObject {
        self.to_string().into_dart()
    }
}

impl IntoDartExceptPrimitive for &'_ str {}

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

impl IntoDartExceptPrimitive for CString {}

/// It is used when you want to write a generic function on different data types
/// and do not want to repeat yourself dozens of times.
/// For example, inside [Drop] of [DartCObject].
pub(crate) trait DartTypedDataTypeVisitor {
    fn visit<T: DartTypedDataTypeTrait>(&self);
}

/// The Rust type for corresponding [DartTypedDataType]
pub trait DartTypedDataTypeTrait {
    fn dart_typed_data_type() -> DartTypedDataType;

    fn function_pointer_of_free_zero_copy_buffer() -> DartHandleFinalizer;
}

fn vec_to_dart_native_external_typed_data<T>(
    vec_from_rust: Vec<T>,
) -> DartCObject
where
    T: DartTypedDataTypeTrait,
{
    let mut vec = ManuallyDrop::new(vec_from_rust);
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
                data: ptr as *mut u8,
                peer: ptr as *mut c_void,
                callback: T::function_pointer_of_free_zero_copy_buffer(),
            },
        },
    }
}

macro_rules! dart_typed_data_type_trait_impl {
    ($($dart_type:path => $rust_type:ident + $free_zero_copy_buffer_func:ident),+) => {
        $(
            impl DartTypedDataTypeTrait for $rust_type {
                fn dart_typed_data_type() -> DartTypedDataType {
                    $dart_type
                }

                fn function_pointer_of_free_zero_copy_buffer() -> DartHandleFinalizer {
                    $free_zero_copy_buffer_func
                }
            }

            impl<const N: usize> IntoDart for [$rust_type;N] {
                fn into_dart(self) -> DartCObject {
                    let vec: Vec<_> = self.into();
                    vec.into_dart()
                }
            }

            #[cfg(not(feature="zero-copy"))]
            impl IntoDart for Vec<$rust_type> {
                fn into_dart(self) -> DartCObject {
                    let mut vec = ManuallyDrop::new(self);
                    let data = DartNativeTypedData {
                        ty: $rust_type::dart_typed_data_type(),
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
            #[cfg(feature="zero-copy")]
            impl IntoDart for Vec<$rust_type> {
                fn into_dart(self) -> DartCObject {
                    vec_to_dart_native_external_typed_data(self)
                }
            }

            #[doc(hidden)]
            #[no_mangle]
            pub(crate) unsafe extern "C" fn $free_zero_copy_buffer_func(
                isolate_callback_data: *mut c_void,
                peer: *mut c_void,
            ) {
                let len = (isolate_callback_data as isize) as usize;
                let ptr = peer as *mut $rust_type;
                drop(Vec::from_raw_parts(ptr, len, len));
            }
        )+

        pub(crate) fn visit_dart_typed_data_type<V: DartTypedDataTypeVisitor>(ty: DartTypedDataType, visitor: &V) {
            match ty {
                $(
                    $dart_type => visitor.visit::<$rust_type>(),
                )+
                _ => panic!("visit_dart_typed_data_type see unexpected DartTypedDataType={:?}", ty)
            }
        }
    }
}

dart_typed_data_type_trait_impl!(
    DartTypedDataType::Int8 => i8 + free_zero_copy_buffer_i8,
    DartTypedDataType::Uint8 => u8 + free_zero_copy_buffer_u8,
    DartTypedDataType::Int16 => i16 + free_zero_copy_buffer_i16,
    DartTypedDataType::Uint16 => u16 + free_zero_copy_buffer_u16,
    DartTypedDataType::Int32 => i32 + free_zero_copy_buffer_i32,
    DartTypedDataType::Uint32 => u32 + free_zero_copy_buffer_u32,
    DartTypedDataType::Int64 => i64 + free_zero_copy_buffer_i64,
    DartTypedDataType::Uint64 => u64 + free_zero_copy_buffer_u64,
    DartTypedDataType::Float32 => f32 + free_zero_copy_buffer_f32,
    DartTypedDataType::Float64 => f64 + free_zero_copy_buffer_f64
);

impl<T> IntoDart for ZeroCopyBuffer<Vec<T>>
where
    T: DartTypedDataTypeTrait,
{
    fn into_dart(self) -> DartCObject {
        vec_to_dart_native_external_typed_data(self.0)
    }
}

impl<T> IntoDartExceptPrimitive for ZeroCopyBuffer<Vec<T>> where
    T: DartTypedDataTypeTrait
{
}

impl<T> IntoDart for Vec<T>
where
    T: IntoDartExceptPrimitive,
{
    fn into_dart(self) -> DartCObject {
        DartArray::from(self.into_iter()).into_dart()
    }
}

impl<T> IntoDartExceptPrimitive for Vec<T> where T: IntoDartExceptPrimitive {}

impl<T, const N: usize> IntoDart for ZeroCopyBuffer<[T; N]>
where
    T: DartTypedDataTypeTrait,
{
    fn into_dart(self) -> DartCObject {
        let vec: Vec<_> = self.0.into();
        ZeroCopyBuffer(vec).into_dart()
    }
}

impl<T, const N: usize> IntoDartExceptPrimitive for ZeroCopyBuffer<[T; N]> where
    T: DartTypedDataTypeTrait
{
}

impl<T, const N: usize> IntoDart for [T; N]
where
    T: IntoDartExceptPrimitive,
{
    fn into_dart(self) -> DartCObject {
        DartArray::from(IntoIterator::into_iter(self)).into_dart()
    }
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

impl<T> IntoDartExceptPrimitive for Option<T> where T: IntoDart {}

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

impl<T, E> IntoDartExceptPrimitive for Result<T, E>
where
    T: IntoDart,
    E: ToString,
{
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

macro_rules! impl_into_dart_for_tuple {
    ($( ($($A:ident)+) )*) => {$(
        impl<$($A: IntoDart),+> IntoDart for ($($A),+,) {
            #[allow(non_snake_case)]
            fn into_dart(self) -> DartCObject {
                let ($($A),+,) = self;
                vec![$($A.into_dart()),+].into_dart()
            }
        }
        impl<$($A: IntoDart),+> IntoDartExceptPrimitive for ($($A),+,) {}
    )*};
}

impl_into_dart_for_tuple! {
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
    (A B C D E F G H)
    (A B C D E F G H I)
    (A B C D E F G H I J)
}
