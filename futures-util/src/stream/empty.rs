use core::marker;

use futures_core::{Stream, Poll, Async};
use futures_core::task;

/// A stream which contains no elements.
///
/// This stream can be created with the `stream::empty` function.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Empty<T, E> {
    _data: marker::PhantomData<(T, E)>,
}

/// Creates a stream which contains no elements.
///
/// The returned stream will always return `Ready(None)` when polled.
pub fn empty<T, E>() -> Empty<T, E> {
    Empty { _data: marker::PhantomData }
}

impl<T, E> Stream for Empty<T, E> {
    type Item = T;
    type Error = E;

    fn poll_next(&mut self, _: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}
