use std::any::TypeId;
use std::convert::Infallible;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures::FutureExt;
use tower::util::BoxCloneService;
use tower::{Service, ServiceBuilder};

use rq_engine::{GroupMessageEvent, PrivateMessageEvent};

pub struct Body;
pub struct Request<B>(B);
pub type Response = ();

pub fn new_service<F, Fut, E>(f: F) -> BoxCloneService<E, (), Infallible>
where
    F: FnOnce(E) -> Fut + Copy + Send + 'static + Sync,
    Fut: Future<Output = ()> + Send,
    E: Send + 'static,
{
    let service = ServiceBuilder::new().service_fn(move |req| async move {
        f(req).await;
        Ok(())
    });
    BoxCloneService::new(service)
}

pub struct Handlers {
    pub private_message_handlers: Vec<BoxCloneService<PrivateMessageEvent, (), Infallible>>,
    pub group_message_handlers: Vec<BoxCloneService<GroupMessageEvent, (), Infallible>>,
}

pub trait AddHandler<E> {
    fn add_handler<F, Fut>(&mut self, f: F)
    where
        F: FnOnce(E) -> Fut + Copy + Send + 'static + Sync,
        Fut: Future<Output = ()> + Send;
    // E: Send + 'static;
}

impl AddHandler<GroupMessageEvent> for Handlers {
    fn add_handler<F, Fut>(&mut self, f: F)
    where
        F: FnOnce(GroupMessageEvent) -> Fut + Copy + Send + 'static + Sync,
        Fut: Future<Output = ()> + Send,
        GroupMessageEvent: Send + 'static,
    {
        // TypeId::of::<E>();
        let service = ServiceBuilder::new().service_fn(move |req| async move {
            f(req).await;
            Ok(())
        });
        let service = BoxCloneService::new(service);
        self.group_message_handlers.push(service);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let mut router = Handlers {
            private_message_handlers: vec![],
            group_message_handlers: vec![],
        };
        router.add_handler(process_group1);
        router.add_handler(process_group2);
        let g = GroupMessageEvent::default();
        for mut r in router.group_message_handlers {
            r.call(g.clone()).await;
        }
    }

    async fn process_group1(e: GroupMessageEvent) {
        println!("1: {:?}", e)
    }
    async fn process_group2(e: GroupMessageEvent) {
        println!("2: {:?}", e)
    }
}
