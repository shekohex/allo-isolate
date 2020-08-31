#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    elided_lifetimes_in_paths,
    rust_2018_idioms,
    clippy::fallible_impl_from,
    clippy::missing_const_for_fn,
    intra_doc_link_resolution_failure
)]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55122894")]

//! Allo Isolate
//! Run Multithreaded Rust along with Dart VM (in isolate).
//!
//! Since you can't call into dart from other threads other than the main
//! thread, that holds our rust code from beaing multithreaded, the way that can be done is using Dart [`Isolate`](https://api.dart.dev/stable/2.8.4/dart-isolate/Isolate-class.html)
//! by creating an isolate, send its [`NativePort`](https://api.dart.dev/stable/2.8.4/dart-ffi/NativePort.html) to Rust side, then rust is freely could run and send the result back on that port.
//!
//! Interacting with Dart VM directly isn't that easy, that is why we created
//! that library, it provides [`IntoDart`] trait to convert between Rust data
//! types and Dart Types, and by default it is implemented for all common rust
//! types.
//!
//! ### Example
//!
//! See [`flutterust`](https://github.com/shekohex/flutterust/tree/master/native/scrap-ffi) and how we used it in the `scrap` package to create a webscrapper using Rust and Flutter.

/// Holds the Raw Dart FFI Types Required to send messages to Isolate
use std::future::Future;

mod dart_array;
mod into_dart;

#[cfg(feature = "catch-unwind")]
mod catch_unwind;

pub mod ffi;
pub use into_dart::IntoDart;

// Please don't use `AtomicPtr` here
// see https://github.com/rust-lang/rfcs/issues/2481
static mut POST_COBJECT: Option<ffi::DartPostCObjectFnType> = None;

/// Stores the function pointer of `Dart_PostCObject`, this only should be
/// called once at the start up of the Dart/Flutter Application. it is exported
/// and marked as `#[no_mangle]`.
///
/// you could use it from Dart as following:
///
/// #### Safety
/// This function should only be called once at the start up of the Dart
/// application.
///
/// ### Example
/// ```dart,ignore
/// import 'dart:ffi';
///
/// typedef dartPostCObject = Pointer Function(
///         Pointer<NativeFunction<Int8 Function(Int64,
/// Pointer<Dart_CObject>)>>);
///
/// // assumes that _dl is the `DynamicLibrary`
/// final storeDartPostCObject =
///     _dl.lookupFunction<dartPostCObject, dartPostCObject>(
/// 'store_dart_post_cobject',
/// );
///
/// // then later call
///
/// storeDartPostCObject(NativeApi.postCObject);
/// ```
#[no_mangle]
pub unsafe extern "C" fn store_dart_post_cobject(ptr: ffi::DartPostCObjectFnType) {
    POST_COBJECT = Some(ptr);
}

/// Simple wrapper around the Dart Isolate Port, nothing
/// else.
#[derive(Copy, Clone, Debug)]
pub struct Isolate {
    port: i64,
}

impl Isolate {
    /// Create a new `Isolate` with a port obtained from Dart VM side.
    ///
    /// #### Example
    /// this a non realistic example lol :D
    /// ```rust
    /// # use allo_isolate::Isolate;
    /// let isolate = Isolate::new(42);
    /// ```
    pub const fn new(port: i64) -> Self {
        Self { port }
    }

    /// Post an object to the [`Isolate`] over the port
    /// Object must implement [`IntoDart`].
    ///
    /// returns `true` if the message posted successfully, otherwise `false`
    ///
    /// #### Safety
    /// This assumes that you called [`store_dart_post_cobject`] and we have
    /// access to the `Dart_PostCObject` function pointer also, we do check
    /// if it is not null.
    ///
    /// #### Example
    /// ```rust
    /// # use allo_isolate::Isolate;
    /// let isolate = Isolate::new(42);
    /// isolate.post("Hello Dart !");
    /// ```
    pub fn post(&self, msg: impl IntoDart) -> bool {
        unsafe {
            if let Some(func) = POST_COBJECT {
                let boxed_msg = Box::new(msg.into_dart());
                let ptr = Box::into_raw(boxed_msg);
                // Send the message
                let result = func(self.port, ptr);
                // free the object
                let boxed_obj = Box::from_raw(ptr);
                drop(boxed_obj);
                // I like that dance haha
                result
            } else {
                false
            }
        }
    }

    /// Consumes `Self`, Runs the task, await for the result and then post it
    /// to the [`Isolate`] over the port
    /// Result must implement [`IntoDart`].
    ///
    /// returns `true` if the message posted successfully, otherwise `false`
    ///
    /// #### Safety
    /// This assumes that you called [`store_dart_post_cobject`] and we have
    /// access to the `Dart_PostCObject` function pointer also, we do check
    /// if it is not null.
    ///
    /// #### Example
    /// ```rust,ignore
    /// # use allo_isolate::Isolate;
    /// use async_std::task;
    /// let isolate = Isolate::new(42);
    /// task::spawn(isolate.task(async { 1 + 2 }));
    /// ```
    pub async fn task<T, R>(self, t: T) -> bool
    where
        T: Future<Output = R> + Send + 'static,
        R: Send + IntoDart + 'static,
    {
        self.post(t.await)
    }

    /// Similar to [`Isolate::task`] but with more logic to catch any panic and
    /// report it back
    #[cfg(feature = "catch-unwind")]
    pub async fn catch_unwind<T, R>(self, t: T) -> Result<bool, Box<dyn std::any::Any + Send>>
    where
        T: Future<Output = R> + Send + 'static,
        R: Send + IntoDart + 'static,
    {
        catch_unwind::CatchUnwind::new(t)
            .await
            .map(|msg| Ok(self.post(msg)))?
    }
}
