use allo_isolate::{ffi::ZeroCopyBuffer, Isolate};
mod vm;

fn main() {
    unsafe {
        allo_isolate::store_dart_post_cobject(vm::dart_post_cobject);
    }
    let port = vm::port();
    assert!(port != -1);
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
    assert!(port != -1);
    let isolate = Isolate::new(port);

    assert!(isolate.post(String::new()));
    assert!(isolate.post(String::from("Hello Dart")));
    assert!(isolate.post("Hello Dart"));

    let port = vm::port();
    assert!(port != -1);
    let isolate = Isolate::new(port);

    assert!(isolate.post(vec![String::from("Rust"); 8]));
    assert!(isolate.post(vec![String::from("Dart"); 1024]));
    assert!(isolate.post(vec![42u8; 100]));
    assert!(isolate.post(vec![42u128; 100]));
    assert!(isolate.post(ZeroCopyBuffer(vec![42u8; 100])));
}
