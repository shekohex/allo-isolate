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
    assert!(isolate.post('🎊'));

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

    #[cfg(feature = "anyhow")]
    assert!(isolate.post(return_anyhow_error()));
    #[cfg(feature = "backtrace")]
    assert!(isolate.post(return_backtrace()));
    #[cfg(feature = "chrono")]
    {
        assert!(isolate.post(return_chrono_naive_date_time()));
        assert!(isolate.post(return_chrono_date_time_utc()));
        assert!(isolate.post(return_chrono_date_time_local()));
        assert!(isolate.post(return_chrono_duration()));
    }
    #[cfg(feature = "uuid")]
    {
        assert!(isolate.post(return_uuid()));
        assert!(isolate.post(return_uuids()))
    }

    println!("all done!");
}

#[cfg(feature = "anyhow")]
fn return_anyhow_error() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("sample error"))
}

#[cfg(feature = "backtrace")]
fn return_backtrace() -> backtrace::Backtrace {
    backtrace::Backtrace::new()
}

#[cfg(feature = "chrono")]
fn return_chrono_naive_date_time() -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd(2016, 7, 8).and_hms_micro(9, 10, 11, 123_456)
}
#[cfg(feature = "chrono")]
fn return_chrono_date_time_utc() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}
#[cfg(feature = "chrono")]
fn return_chrono_date_time_local() -> chrono::DateTime<chrono::Local> {
    chrono::Local::now()
}
#[cfg(feature = "chrono")]
fn return_chrono_duration() -> chrono::Duration {
    chrono::Duration::hours(24)
}

#[cfg(feature = "uuid")]
fn return_uuid() -> uuid::Uuid {
    uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
}
#[cfg(feature = "uuid")]
fn return_uuids() -> Vec<uuid::Uuid> {
    vec![
        uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        uuid::Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_run_valgrind_main() {
        super::main();
    }
}
