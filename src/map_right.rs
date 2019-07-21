use futures::{sink, *};

/// A stream combinator which will change the type of the right variant of a
/// stream from one type to another.
///
/// This is produced by the `EitherStreamExt::map_left` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct MapRight<S, F> {
    stream: S,
    f: F,
}

pub fn new<S, F, L, R, U>(s: S, f: F) -> MapRight<S, F>
where
    S: Stream<Item = future::Either<L, R>>,
    F: FnMut(R) -> U,
{
    MapRight { stream: s, f }
}

impl<S, F> MapRight<S, F> {
    /// Acquires a reference to the underlying stream that this combinator is
    /// pulling from.
    pub fn get_ref(&self) -> &S {
        &self.stream
    }

    /// Acquires a mutable reference to the underlying stream that this
    /// combinator is pulling from.
    ///
    /// Note that care must be taken to avoid tampering with the state of the
    /// stream which may otherwise confuse this combinator.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    /// Consumes this combinator, returning the underlying stream.
    ///
    /// Note that this may discard intermediate state of this combinator, so
    /// care should be taken to avoid losing resources when this is called.
    pub fn into_inner(self) -> S {
        self.stream
    }
}

// Forwarding impl of Sink from the underlying stream
impl<S, F> sink::Sink for MapRight<S, F>
where
    S: sink::Sink,
{
    type SinkItem = S::SinkItem;
    type SinkError = S::SinkError;

    fn start_send(&mut self, item: S::SinkItem) -> StartSend<S::SinkItem, S::SinkError> {
        self.stream.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), S::SinkError> {
        self.stream.poll_complete()
    }

    fn close(&mut self) -> Poll<(), S::SinkError> {
        self.stream.close()
    }
}

impl<S, F, L, R, U> Stream for MapRight<S, F>
where
    S: Stream<Item = future::Either<L, R>>,
    F: FnMut(R) -> U,
{
    type Item = future::Either<L, U>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<future::Either<L, U>>, S::Error> {
        let option = try_ready!(self.stream.poll());

        Ok(Async::Ready(option.map(|either| match either {
            future::Either::A(x) => future::Either::A(x),
            future::Either::B(x) => future::Either::B((self.f)(x)),
        })))
    }
}
