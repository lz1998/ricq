use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use cached::{Cached, CachedAsync};
use futures::FutureExt;
use tower::util::BoxCloneService;
use tower::{Service, ServiceBuilder, ServiceExt};

use rq_engine::{GroupMessageEvent, PrivateMessageEvent};

pub struct Body;
pub struct Request<B>(B);
pub type Response = ();

#[derive(Default)]
pub struct Handlers {
    handlers: HashMap<TypeId, Vec<BoxCloneService<Box<dyn Any>, (), Infallible>>>,
}

impl Handlers {
    fn add<E, F, Fut>(&mut self, f: F)
    where
        F: FnOnce(E) -> Fut + Copy + Send + 'static + Sync,
        Fut: Future<Output = ()> + Send,
        E: Send + 'static,
    {
        let key = TypeId::of::<E>();
        let handlers = self.handlers.entry(key).or_insert_with(|| Vec::new());

        let s: BoxCloneService<Box<dyn Any>, (), Infallible> = ServiceBuilder::new()
            .boxed_clone()
            .map_request(|req: Box<dyn Any>| {
                let req: E = req.downcast().ok().map(|boxed| *boxed).unwrap();
                req
            })
            .service_fn(move |req| async move {
                f(req).await;
                Ok(())
            });
        handlers.push(s);
    }

    async fn handle<E: 'static + Clone>(&self, e: E) {
        let k = TypeId::of::<E>();
        if let Some(handlers) = self.handlers.get(&k) {
            for h in handlers {
                h.clone().call(Box::new(e.clone())).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let mut h = Handlers::default();
        h.add(process_group1);
        h.add(process_group2);
        h.add(process_private);
        let g = GroupMessageEvent::default();
        h.handle(g).await;
        let p = PrivateMessageEvent::default();
        h.handle(p).await;
    }

    async fn process_group1(e: GroupMessageEvent) {
        println!("group1: {:?}", e)
    }
    async fn process_group2(e: GroupMessageEvent) {
        println!("group2: {:?}", e)
    }
    async fn process_private(e: PrivateMessageEvent) {
        println!("private: {:?}", e)
    }
}
