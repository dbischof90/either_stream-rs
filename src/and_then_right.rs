use futures::{sink, *};

/// A stream combinator which chains a computation onto values produced by the
/// right variant of a stream.
///
/// This structure is produced by the `Stream::and_then_right` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct AndThenRight<S, F, U>
where
    U: IntoFuture,
{
    stream: S,
    future: Option<U::Future>,
    f: F,
}

pub fn new<S, F, L, R, U>(s: S, f: F) -> AndThenRight<S, F, U>
where
    S: Stream<Item = future::Either<L, R>>,
    F: FnMut(R) -> U,
    U: IntoFuture<Error = S::Error>,
{
    AndThenRight {
        stream: s,
        future: None,
        f,
    }
}

impl<S, F, U> AndThenRight<S, F, U>
where
    U: IntoFuture,
{
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
impl<S, F, U: IntoFuture> sink::Sink for AndThenRight<S, F, U>
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

impl<S, F, L, R, U> Stream for AndThenRight<S, F, U>
where
    S: Stream<Item = future::Either<L, R>>,
    F: FnMut(R) -> U,
    U: IntoFuture<Error = S::Error>,
{
    type Item = future::Either<L, U::Item>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<future::Either<L, U::Item>>, S::Error> {
        if self.future.is_none() {
            let item = match try_ready!(self.stream.poll()) {
                None => return Ok(Async::Ready(None)),
                Some(future::Either::B(x)) => (self.f)(x).into_future(),
                Some(future::Either::A(x)) => return Ok(Async::Ready(Some(future::Either::A(x)))),
            };
            self.future = Some(item);
        }
        assert!(self.future.is_some());

        match self.future.as_mut().unwrap().poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                self.future = None;
                Err(e)
            }
            Ok(Async::Ready(y)) => {
                self.future = None;
                Ok(Async::Ready(Some(future::Either::B(y))))
            }
        }
    }
}
