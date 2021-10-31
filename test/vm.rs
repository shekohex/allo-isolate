use allo_isolate::{
    ffi::{
        DartCObject, DartCObjectType, DartCObjectValue, DartNativeArray,
        DartTypedDataType,
    },
    IntoDart,
};
use std::{
    collections::HashMap,
    ffi::{c_void, CStr},
    sync::Mutex,
};

thread_local! {
  static DART_VM: DartVM = DartVM::new();
}

struct DartVM {
    ports: Mutex<HashMap<i64, VMIsolate>>,
}

impl DartVM {
    fn new() -> Self {
        Self {
            ports: Default::default(),
        }
    }

    /// Allocate a new port so you can post messages to
    pub fn port(&self) -> i64 {
        let port = fastrand::i64(1..i64::MAX);
        if let Ok(mut ports) = self.ports.lock() {
            ports.insert(port, VMIsolate::new());
            port
        } else {
            -1
        }
    }

    pub fn post(&self, port: i64, object: *mut DartCObject) -> bool {
        if let Ok(mut ports) = self.ports.lock() {
            if let Some(isolate) = ports.get_mut(&port) {
                isolate.exec(object)
            } else {
                false
            }
        } else {
            false
        }
    }
}

struct VMIsolate;

impl VMIsolate {
    const fn new() -> Self { Self }

    fn exec(&mut self, object: *mut DartCObject) -> bool {
        use DartCObjectType::*;
        assert!(!object.is_null(), "got a null object");
        let o = unsafe { &*object };
        match o.ty {
            DartNull => {
                DartCObject {
                    ty: DartNull,
                    value: DartCObjectValue { as_bool: false },
                };
            },
            DartBool => {
                DartCObject {
                    ty: DartBool,
                    value: DartCObjectValue {
                        as_bool: unsafe { o.value.as_bool },
                    },
                };
            },
            DartInt32 => {
                DartCObject {
                    ty: DartInt32,
                    value: DartCObjectValue {
                        as_int32: unsafe { o.value.as_int32 },
                    },
                };
            },
            DartInt64 => {
                DartCObject {
                    ty: DartInt64,
                    value: DartCObjectValue {
                        as_int64: unsafe { o.value.as_int64 },
                    },
                };
            },
            DartDouble => {
                DartCObject {
                    ty: DartDouble,
                    value: DartCObjectValue {
                        as_double: unsafe { o.value.as_double },
                    },
                };
            },
            DartString => {
                {
                    let s =
                        unsafe { CStr::from_ptr(o.value.as_string) }.to_owned();
                    DartCObject {
                        ty: DartString,
                        value: DartCObjectValue {
                            as_string: s.into_raw(),
                        },
                    }
                };
            },
            DartArray => {
                // do something with o
                // I'm semulating that I copied some data into the VM here
                let v: Vec<_> = vec![0u32; 0]
                    .into_iter()
                    .map(IntoDart::into_dart)
                    .map(Box::new)
                    .map(Box::into_raw)
                    .collect();
                let mut v = v.into_boxed_slice();
                DartCObject {
                    ty: DartArray,
                    value: DartCObjectValue {
                        as_array: DartNativeArray {
                            length: v.len() as isize,
                            values: v.as_mut_ptr(),
                        },
                    },
                };
            },
            DartTypedData => {
                let v = unsafe { o.value.as_typed_data };
                let ty = v.ty;
                match ty {
                    DartTypedDataType::Int8 => {
                        let _ = unsafe {
                            from_buf_raw(v.values as *mut i8, v.length as usize)
                        };
                    },
                    DartTypedDataType::Uint8 => {
                        let _ = unsafe {
                            from_buf_raw(v.values as *mut u8, v.length as usize)
                        };
                    },
                    _ => unimplemented!(),
                };
            },
            DartExternalTypedData => {
                let v = unsafe { o.value.as_external_typed_data };
                let ty = v.ty;
                match ty {
                    DartTypedDataType::Uint8 => {
                        let _ = unsafe {
                            let output = from_buf_raw(
                                v.data as *mut u8,
                                v.length as usize,
                            );
                            let cb = v.callback;
                            cb(v.length as *mut c_void, v.peer);
                            output
                        };
                    },
                    _ => unimplemented!(),
                };
            },
            _ => {
                unimplemented!();
            },
        };
        true
    }
}

unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);
    dst.set_len(elts);
    std::ptr::copy(ptr, dst.as_mut_ptr(), elts);
    dst
}

pub extern "C" fn dart_post_cobject(port: i64, msg: *mut DartCObject) -> bool {
    DART_VM.with(|vm| vm.post(port, msg))
}

pub fn port() -> i64 { DART_VM.with(|vm| vm.port()) }
