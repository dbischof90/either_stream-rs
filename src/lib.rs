mod map_left;
mod map_right;
mod and_then_left;
mod and_then_right;

use futures::future;
use futures::prelude::*;

/// Provides the `EitherStreamExt` trait, allowing to apply functions
/// on either the left or right variant of a `futures::Stream` of `futures::future::Either <L, R>`.
///
/// By simply importing the trait
/// ```
///     use either_stream::EitherStreamExt;
/// ```
/// one can use the provided combinators like the standard combinators provided by
/// the `futures::stream` module. Note here that currently this crate is only compatible with
/// `futures-0.1.*`.

pub trait EitherStreamExt<L, R>: Stream<Item = future::Either<L, R>> {
    fn map_left<U, F>(self, f: F) -> map_left::MapLeft<Self, F>
    where
        F: FnMut(L) -> U,
        Self: Sized;

    fn map_right<U, F>(self, f: F) -> map_right::MapRight<Self, F>
    where
        F: FnMut(R) -> U,
        Self: Sized;

    fn and_then_right<U, F>(self, f: F) -> and_then_right::AndThenRight<Self, F, U>
    where
        F: FnMut(R) -> U,
        Self: Sized,
        U: IntoFuture<Error = Self::Error>;
    
    fn and_then_left<U, F>(self, f: F) -> and_then_left::AndThenLeft<Self, F, U>
    where
        F: FnMut(L) -> U,
        Self: Sized,
        U: IntoFuture<Error = Self::Error>;
}

impl<L, R, T: Stream<Item = future::Either<L, R>>> EitherStreamExt<L, R> for T {
    fn map_left<U, F>(self, f: F) -> map_left::MapLeft<Self, F>
    where
        F: FnMut(L) -> U,
        Self: Sized,
    {
        map_left::new(self, f)
    }

    fn map_right<U, F>(self, f: F) -> map_right::MapRight<Self, F>
    where
        F: FnMut(R) -> U,
        Self: Sized,
    {
        map_right::new(self, f)
    }

    fn and_then_left<U, F>(self, f: F) -> and_then_left::AndThenLeft<Self, F, U>
    where
        F: FnMut(L) -> U,
        Self: Sized,
        U: IntoFuture<Error = Self::Error>
    {
        and_then_left::new(self, f)
    }

    fn and_then_right<U, F>(self, f: F) -> and_then_right::AndThenRight<Self, F, U>
    where
        F: FnMut(R) -> U,
        Self: Sized,
        U: IntoFuture<Error = Self::Error>
    {
        and_then_right::new(self, f)
    }
}
