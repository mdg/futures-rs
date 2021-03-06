#![feature(pin, arbitrary_self_types, futures_api)]

use futures::future::{self, FutureExt};
use futures_test::task::panic_context;

#[test]
fn fuse() {
    let mut future = future::ready::<i32>(2).fuse();
    let cx = &mut panic_context();
    assert!(future.poll_unpin(cx).is_ready());
    assert!(future.poll_unpin(cx).is_pending());
}
