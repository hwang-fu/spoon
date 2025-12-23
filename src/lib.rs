// Spoon async runtime

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A future that is immediately ready with a value.
struct Ready<T> {
    value: Option<T>,
}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: take the value and return Ready
    }
}
