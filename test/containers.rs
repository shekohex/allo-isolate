use allo_isolate::{IntoDartTypedData, Isolate, ZeroCopyBuffer};

mod vm;

fn main() {
    unsafe {
        allo_isolate::store_dart_post_cobject(vm::dart_post_cobject);
    }
    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);
    assert!(isolate.post(42i32));
    assert!(isolate.post(42u32));
    assert!(isolate.post(42i64));
    assert!(isolate.post(42u64));
    assert!(isolate.post(42i128));
    assert!(isolate.post(42u128));
    assert!(isolate.post(true));
    assert!(isolate.post(false));

    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);

    assert!(isolate.post(String::new()));
    assert!(isolate.post(String::from("Hello Dart")));
    assert!(isolate.post("Hello Dart"));

    let port = vm::port();
    assert_ne!(port, -1);
    let isolate = Isolate::new(port);

    assert!(isolate.post(vec![String::from("Rust"); 8]));
    assert!(isolate.post(vec![String::from("Dart"); 1024]));
    assert!(isolate.post(vec![42i8; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42u8; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42i16; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42u16; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42i32; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42u32; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42i64; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42u64; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42.0f32; 100].into_dart_typed_data()));
    assert!(isolate.post(vec![42.0f64; 100].into_dart_typed_data()));
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
}
