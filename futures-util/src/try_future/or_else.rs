use super::{TryChain, TryChainAction};
use core::pin::Pin;
use futures_core::future::{FusedFuture, Future, TryFuture};
use futures_core::task::{LocalWaker, Poll};
use pin_utils::unsafe_pinned;

/// Future for the [`or_else`](super::TryFutureExt::or_else) combinator.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct OrElse<Fut1, Fut2, F> {
    try_chain: TryChain<Fut1, Fut2, F>,
}

impl<Fut1, Fut2, F> OrElse<Fut1, Fut2, F>
    where Fut1: TryFuture,
          Fut2: TryFuture,
{
    unsafe_pinned!(try_chain: TryChain<Fut1, Fut2, F>);

    /// Creates a new `Then`.
    pub(super) fn new(future: Fut1, f: F) -> OrElse<Fut1, Fut2, F> {
        OrElse {
            try_chain: TryChain::new(future, f),
        }
    }
}

impl<Fut1, Fut2, F> FusedFuture for OrElse<Fut1, Fut2, F>
    where Fut1: TryFuture,
          Fut2: TryFuture<Ok = Fut1::Ok>,
          F: FnOnce(Fut1::Error) -> Fut2,
{
    fn is_terminated(&self) -> bool {
        self.try_chain.is_terminated()
    }
}

impl<Fut1, Fut2, F> Future for OrElse<Fut1, Fut2, F>
    where Fut1: TryFuture,
          Fut2: TryFuture<Ok = Fut1::Ok>,
          F: FnOnce(Fut1::Error) -> Fut2,
{
    type Output = Result<Fut2::Ok, Fut2::Error>;

    fn poll(
        self: Pin<&mut Self>,
        lw: &LocalWaker,
    ) -> Poll<Self::Output> {
        self.try_chain().poll(lw, |result, async_op| {
            match result {
                Ok(ok) => TryChainAction::Output(Ok(ok)),
                Err(err) => TryChainAction::Future(async_op(err)),
            }
        })
    }
}
