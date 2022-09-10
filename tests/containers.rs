use allo_isolate::{ffi::DartCObjectType, IntoDart, Isolate, ZeroCopyBuffer};

mod vm;

fn main() {
    // Create a Dart VM call before we could handle sending a message back to
    // Dart
    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);
    assert!(!isolate.post(vec![String::from("Rust"); 8]));
    assert!(!isolate.post(vec![String::from("Dart"); 1024]));
    assert!(!isolate.post(vec![42i8; 100]));
    assert!(!isolate.post(vec![42u8; 100]));
    assert!(!isolate.post(vec![42i16; 100]));
    assert!(!isolate.post(vec![42u16; 100]));
    assert!(!isolate.post(vec![42i32; 100]));
    assert!(!isolate.post(vec![42u32; 100]));
    assert!(!isolate.post(vec![42i64; 100]));
    assert!(!isolate.post(vec![42u64; 100]));
    assert!(!isolate.post(vec![42.0f32; 100]));
    assert!(!isolate.post(vec![42.0f64; 100]));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42.0f32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42.0f64; 100])));

    assert!(!isolate.post([42i8; 100]));
    assert!(!isolate.post([42u8; 100]));
    assert!(!isolate.post([42i16; 100]));
    assert!(!isolate.post([42u16; 100]));
    assert!(!isolate.post([42i32; 100]));
    assert!(!isolate.post([42u32; 100]));
    assert!(!isolate.post([42i64; 100]));
    assert!(!isolate.post([42u64; 100]));
    assert!(!isolate.post([42.0f32; 100]));
    assert!(!isolate.post([42.0f64; 100]));
    assert!(!isolate.post(ZeroCopyBuffer([42i8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42i16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42i32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42i64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42.0f32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42.0f64; 100])));
    // Provide the pointer that allows Rust to communicate messages back to the
    // Dart VM
    unsafe {
        allo_isolate::store_dart_post_cobject(vm::dart_post_cobject);
    }

    // Post some messages that will succeed
    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);
    assert!(isolate.post(42i8));
    assert!(isolate.post(42u8));
    assert!(isolate.post(42i16));
    assert!(isolate.post(42u16));
    assert!(isolate.post(42i32));
    assert!(isolate.post(42u32));
    assert!(isolate.post(42i64));
    assert!(isolate.post(42u64));
    assert!(isolate.post(42i128));
    assert!(isolate.post(42u128));
    assert!(isolate.post(42usize));
    assert!(isolate.post(true));
    assert!(isolate.post(false));

    // Create another isolate and port that still works
    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);

    assert!(isolate.post(String::new()));
    assert!(isolate.post(String::from("Hello Dart")));
    assert!(isolate.post("Hello Dart"));

    // Create another isolate and port that still works
    let port = vm::port();
    assert_ne!(port, -1);
    let isolate2 = Isolate::new(port);

    // Send data to the new port
    assert!(isolate2.post(String::new()));
    assert!(isolate2.post(String::from("Hello Dart")));
    assert!(isolate2.post("Hello Dart"));
    assert!(isolate.post(ZeroCopyBuffer(vec![42.0f64; 100])));

    // Send data to the old port
    assert!(isolate.post(String::new()));
    assert!(isolate.post(String::from("Hello Dart")));
    assert!(isolate.post("Hello Dart"));
    assert!(isolate.post(ZeroCopyBuffer(vec![42.0f64; 100])));

    // Send data to the new port again
    assert!(isolate2.post(String::new()));
    assert!(isolate2.post(String::from("Hello Dart")));
    assert!(isolate2.post("Hello Dart"));
    assert!(isolate.post(ZeroCopyBuffer(vec![42.0f64; 100])));

    // Create another port and send all the data successfully
    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);

    assert!(isolate.post(vec![String::from("Rust"); 8]));
    assert!(isolate.post(vec![String::from("Dart"); 1024]));
    assert!(isolate.post(vec![
        vec![String::from("Rust"); 8],
        vec![String::from("Dart"); 1024]
    ]));
    assert!(isolate.post(vec![
        vec![
            vec![String::from("Rust"); 8],
            vec![String::from("Dart"); 1024]
        ],
        vec![
            vec![String::from("Rust"); 8],
            vec![String::from("Dart"); 1024]
        ]
    ]));
    assert!(isolate.post(vec![42i8; 100]));
    assert!(isolate.post(vec![42u8; 100]));
    assert!(isolate.post(vec![42i16; 100]));
    assert!(isolate.post(vec![42u16; 100]));
    assert!(isolate.post(vec![42i32; 100]));
    assert!(isolate.post(vec![42u32; 100]));
    assert!(isolate.post(vec![42i64; 100]));
    assert!(isolate.post(vec![42u64; 100]));
    assert!(isolate.post(vec![42.0f32; 100]));
    assert!(isolate.post(vec![42.0f64; 100]));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i8; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u8; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i16; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u16; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i32; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u32; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i64; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u64; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42.0f32; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42.0f64; 100])));

    assert!(isolate.post([42i8; 100]));
    assert!(isolate.post([42u8; 100]));
    assert!(isolate.post([42i16; 100]));
    assert!(isolate.post([42u16; 100]));
    assert!(isolate.post([42i32; 100]));
    assert!(isolate.post([42u32; 100]));
    assert!(isolate.post([42i64; 100]));
    assert!(isolate.post([42u64; 100]));
    assert!(isolate.post([42.0f32; 100]));
    assert!(isolate.post([42.0f64; 100]));
    assert!(isolate.post(ZeroCopyBuffer([42i8; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u8; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42i16; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u16; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42i32; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u32; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42i64; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u64; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42.0f32; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42.0f64; 100])));
    {
        // https://github.com/sunshine-protocol/allo-isolate/pull/17
        let u32_into_dart = 0xfe112233_u32.into_dart();
        assert_eq!(DartCObjectType::DartInt64, u32_into_dart.ty);
        unsafe {
            assert_eq!(0xfe112233_i64, u32_into_dart.value.as_int64);
        }
    }

    assert!(isolate.post(return_anyhow_error()));
    assert!(isolate.post(return_backtrace()));

    println!("all done!");
}

fn return_anyhow_error() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("sample error"))
}

fn return_backtrace() -> backtrace::Backtrace { backtrace::Backtrace::new() }

#[cfg(test)]
mod tests {
    #[test]
    fn can_run_valgrind_main() { super::main(); }
}
