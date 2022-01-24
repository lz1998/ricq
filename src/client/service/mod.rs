use std::convert::Infallible;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures::FutureExt;
use tower::util::BoxCloneService;
use tower::Service;

pub struct Body;
pub struct Request<B>(B);
pub type Response = ();

#[async_trait]
pub trait Handler<T, B = Body>: Clone + Send + Sized + 'static {
    async fn call(self, req: Request<B>) -> ();
}

pub struct IntoService<H, T, B> {
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> IntoService<H, T, B> {
    pub(super) fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> fmt::Debug for IntoService<H, T, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoService")
            .field(&format_args!("..."))
            .finish()
    }
}

impl<H, T, B> Clone for IntoService<H, T, B>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for IntoService<H, T, B>
where
    H: Handler<T, B> + Clone + Send + 'static,
    B: Send + 'static,
{
    type Response = ();
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        let future = async move {
            Handler::call(handler, req).await;
            Ok(())
        };
        Box::pin(future)
    }
}

pub struct Route<B = Body, E = Infallible>(pub(crate) BoxCloneService<Request<B>, Response, E>);

impl<B, E> Route<B, E> {
    pub(super) fn new<T>(svc: T) -> Self
    where
        T: Service<Request<B>, Response = (), Error = E> + Clone + Send + 'static,
        T::Future: Send + 'static,
    {
        Self(BoxCloneService::new(svc))
    }
}
