use allo_isolate::{ffi::DartCObjectType, IntoDart, Isolate, ZeroCopyBuffer};
use std::collections::{HashMap, HashSet};

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
    assert!(!isolate.post(vec![42isize; 100]));
    assert!(!isolate.post(vec![42usize; 100]));
    assert!(!isolate.post(vec![42.0f32; 100]));
    assert!(!isolate.post(vec![42.0f64; 100]));
    assert!(!isolate.post(vec![true; 100]));
    assert!(!isolate.post(vec![false; 100]));
    assert!(!isolate.post(ZeroCopyBuffer(vec![0u8; 0])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42i64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42u64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42isize; 100])));
    assert!(!isolate.post(ZeroCopyBuffer(vec![42usize; 100])));
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
    assert!(!isolate.post([42isize; 100]));
    assert!(!isolate.post([42usize; 100]));
    assert!(!isolate.post([42.0f32; 100]));
    assert!(!isolate.post([42.0f64; 100]));
    assert!(!isolate.post([true; 100]));
    assert!(!isolate.post([false; 100]));
    assert!(!isolate.post(ZeroCopyBuffer([0u8; 0])));
    assert!(!isolate.post(ZeroCopyBuffer([42i8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u8; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42i16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u16; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42i32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u32; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42i64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42u64; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42isize; 100])));
    assert!(!isolate.post(ZeroCopyBuffer([42usize; 100])));
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
    assert!(isolate.post(42isize));
    assert!(isolate.post(42usize));
    assert!(isolate.post(42i128));
    assert!(isolate.post(42u128));
    assert!(isolate.post(42usize));
    assert!(isolate.post(42isize));
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
    assert!(isolate.post(vec![42isize; 100]));
    assert!(isolate.post(vec![42usize; 100]));
    assert!(isolate.post(vec![42.0f32; 100]));
    assert!(isolate.post(vec![42.0f64; 100]));
    assert!(isolate.post(vec![true; 100]));
    assert!(isolate.post(vec![false; 100]));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i8; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u8; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i16; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u16; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i32; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u32; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42i64; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u64; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42isize; 100])));
    assert!(isolate.post(ZeroCopyBuffer(vec![42usize; 100])));
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
    assert!(isolate.post([42i64; 100]));
    assert!(isolate.post([42usize; 100]));
    assert!(isolate.post([42.0f32; 100]));
    assert!(isolate.post([42.0f64; 100]));
    assert!(isolate.post([true; 100]));
    assert!(isolate.post([false; 100]));
    assert!(isolate.post(ZeroCopyBuffer([42i8; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u8; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42i16; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u16; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42i32; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u32; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42i64; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42u64; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42isize; 100])));
    assert!(isolate.post(ZeroCopyBuffer([42usize; 100])));
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
    assert!(isolate.post(std::backtrace::Backtrace::capture()));
    #[cfg(feature = "backtrace")]
    assert!(isolate.post(return_backtrace()));
    #[cfg(feature = "chrono")]
    {
        assert!(isolate.post(return_chrono_naive_date()));
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

    assert!(isolate.post(("asd", "asd".to_string(), 123)));
    assert!(isolate.post(((true,), (123,))));
    assert!(isolate.post((ZeroCopyBuffer(vec![-1]), vec![-1], 1.1)));
    assert!(isolate.post((1, 2, 3, 4, 5, 6, 7, 8, 9, (10, 11))));

    assert!(
        isolate.post(HashMap::from([("key".to_owned(), "value".to_owned())]))
    );
    assert!(isolate.post(HashMap::from([(100, 200)])));
    assert!(isolate.post(HashMap::from([(100, 200u8)])));
    assert!(isolate.post(HashMap::from([(100, vec![42u8])])));
    assert!(isolate.post(HashSet::from(["value".to_owned()])));
    assert!(isolate.post(HashSet::from([200])));
    assert!(isolate.post(HashSet::from([200u8])));

    assert!(isolate.post(vec![vec![10u8]]));

    // Test 2D array support - this is what PR #66 enables
    let arr_2d: [[bool; 3]; 2] = [[true, false, true], [false, true, false]];
    assert!(isolate.post(arr_2d));

    // Test case to verify that dropping vectors using ZeroCopyBuffer no longer causes a panic
    let a: ZeroCopyBuffer<Vec<u64>> = ZeroCopyBuffer(vec![]);
    let b = a.into_dart();
    drop(b);

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
fn return_chrono_naive_date() -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(1776, 7, 4)
        .expect("The input date for testing is required to be valid")
}
#[cfg(feature = "chrono")]
fn return_chrono_naive_date_time() -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd_opt(2016, 7, 8)
        .map(|nd| nd.and_hms_micro_opt(9, 10, 11, 123_456))
        .flatten()
        .expect("The input date and time for testing are required to be valid")
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
