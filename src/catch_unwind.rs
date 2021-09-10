use core::future::Future;
use pin_project::pin_project;
use std::{
    any::Any,
    panic,
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct CatchUnwind<F: Future>(#[pin] F);

impl<F: Future> CatchUnwind<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F> Future for CatchUnwind<F>
where
    F: Future,
{
    type Output = Result<F::Output, Box<dyn Any + Send>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let f = this.0;
        panic::catch_unwind(panic::AssertUnwindSafe(|| f.poll(cx)))?.map(Ok)
    }
}
