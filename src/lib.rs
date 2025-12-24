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
        let value = self
            .get_mut()
            .value
            .take()
            .expect("polled after completion");
        Poll::Ready(value)
    }
}

#[allow(dead_code)]
fn ready<T>(value: T) -> Ready<T> {
    Ready { value: Some(value) }
}

#[allow(dead_code)]
struct CountDown {
    count: u32,
}

impl Unpin for CountDown {}

impl Future for CountDown {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.count == 0 {
            Poll::Ready(())
        } else {
            this.count -= 1;
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{RawWaker, RawWakerVTable, Waker};

    fn dummy_waker() -> Waker {
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);

        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }

    #[test]
    fn test_ready_future() {
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);

        let mut fut = ready(42);
        let pinned = Pin::new(&mut fut);

        match pinned.poll(&mut cx) {
            Poll::Ready(v) => assert_eq!(v, 42),
            Poll::Pending => panic!("expected Ready"),
        }
    }
}
