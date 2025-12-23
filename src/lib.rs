// Spoon async runtime

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A future that is immediately ready with a value.
#[allow(dead_code)]
struct Ready<T> {
    value: Option<T>,
}

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let value = self.get_mut().value.take().expect("");
        Poll::Ready(value)
    }
}

#[allow(dead_code)]
fn ready<T>(value: T) -> Ready<T> {
    Ready { value: Some(value) }
}
