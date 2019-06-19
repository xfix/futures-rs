use core::marker::PhantomData;
use core::pin::Pin;
use futures_core::task::{Context, Poll};
use futures_core::never::Never;
use futures_sink::Sink;

/// Sink for the [`drain`] function.
#[derive(Debug)]
#[must_use = "sinks do nothing unless polled"]
pub struct Drain<T> {
    marker: PhantomData<T>,
}

/// Create a sink that will just discard all items given to it.
///
/// Similar to [`io::Sink`](::std::io::Sink).
///
/// # Examples
///
/// ```
/// #![feature(async_await)]
/// # futures::executor::block_on(async {
/// use futures::sink::{self, SinkExt};
///
/// let mut drain = sink::drain();
/// drain.send(5).await?;
/// # Ok::<(), futures::never::Never>(()) }).unwrap();
/// ```
pub fn drain<T>() -> Drain<T> {
    Drain { marker: PhantomData }
}

impl<T> Sink<T> for Drain<T> {
    type SinkError = Never;

    fn poll_ready(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::SinkError>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(
        self: Pin<&mut Self>,
        _item: T,
    ) -> Result<(), Self::SinkError> {
        Ok(())
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::SinkError>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::SinkError>> {
        Poll::Ready(Ok(()))
    }
}
