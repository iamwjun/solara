use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, wake_raw, wake_by_ref_raw, drop_raw);

unsafe fn clone_raw(_: *const ()) -> RawWaker {
    RawWaker::new(std::ptr::null(), &VTABLE)
}

unsafe fn wake_raw(_: *const ()) {}

unsafe fn wake_by_ref_raw(_: *const ()) {}

unsafe fn drop_raw(_: *const ()) {}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    let mut pinned = pin!(future);

    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
