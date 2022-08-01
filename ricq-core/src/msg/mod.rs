use std::fmt;

use elem::RQElem;
use elem::*;

use crate::pb::msg;

pub mod elem;
mod fragment;
mod macros;

pub type MessageElem = msg::elem::Elem;

#[derive(Debug, Default, Clone)]
pub struct MessageChain(pub Vec<MessageElem>);

impl MessageChain {
    pub fn new<E: Into<Vec<MessageElem>>>(e: E) -> Self {
        Self(e.into())
    }

    pub fn push<E: Into<Vec<MessageElem>>>(&mut self, e: E) {
        self.0.extend(e.into())
    }

    pub fn anonymous(&self) -> Option<Anonymous> {
        self.0.iter().find_map(|e| match e {
            MessageElem::AnonGroupMsg(anonymous) => Some(Anonymous::from(anonymous.clone())),
            _ => None,
        })
    }

    pub fn reply(&self) -> Option<Reply> {
        self.0.iter().find_map(|e| match e {
            MessageElem::SrcMsg(src_msg) => Some(Reply::from(src_msg.clone())),
            _ => None,
        })
    }

    pub fn with_anonymous(&mut self, anonymous: Anonymous) {
        self.0.insert(0, MessageElem::from(anonymous))
    }

    pub fn with_reply(&mut self, reply: Reply) {
        let index = if self.anonymous().is_some() { 1 } else { 0 };
        self.0.insert(index, MessageElem::from(reply))
    }
}

impl<E> FromIterator<E> for MessageChain
where
    E: Into<Vec<MessageElem>>,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        Self(iter.into_iter().flat_map(Into::into).collect())
    }
}

impl IntoIterator for MessageChain {
    type Item = RQElem;
    type IntoIter = impl Iterator<Item = RQElem> + 'static;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .filter_map(|e| match e {
                MessageElem::SrcMsg(_) => None,
                MessageElem::AnonGroupMsg(_) => None,
                _ => Some(e),
            })
            .map(RQElem::from)
    }
}

impl From<Vec<msg::Elem>> for MessageChain {
    fn from(elements: Vec<msg::Elem>) -> Self {
        Self(elements.into_iter().filter_map(|e| e.elem).collect())
    }
}

impl From<MessageChain> for Vec<msg::Elem> {
    fn from(e: MessageChain) -> Self {
        e.0.into_iter()
            .map(|e| msg::Elem { elem: Some(e) })
            .collect()
    }
}

impl fmt::Display for MessageChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.clone().into_iter() {
            fmt::Display::fmt(&x, f)?
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct MessageChainBuilder {
    pub elems: Vec<MessageElem>,
    pub buf_string: String,
}

impl MessageChainBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<E: PushBuilder>(&mut self, elem: E) -> &mut Self {
        E::push_builder(elem, self);
        self
    }

    pub fn push_str(&mut self, str: &str) -> &mut Self {
        self.buf_string.push_str(str);
        self
    }

    pub fn build(mut self) -> MessageChain {
        self.flush();
        MessageChain::new(self.elems)
    }

    fn flush(&mut self) {
        flush_builder(self);
    }
}

pub trait PushElem {
    fn push_to(elem: Self, vec: &mut Vec<MessageElem>);
}

pub trait PushBuilder {
    fn push_builder(elem: Self, builder: &mut MessageChainBuilder);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut chain = MessageChain::default();
        chain.push(Text::new("hello".into()));
        for e in chain.into_iter() {
            println!("{:?}", e)
        }
    }

    #[test]
    fn test_display() {
        let mut chain = MessageChain::default();
        chain.with_anonymous(Anonymous::default());
        chain.with_reply(Reply::default());
        chain.push(Text::new("hello".into()));
        chain.push(At::new(12345));
        chain.push(Text::new("world".into()));
        chain.push(Face::new(1));
        chain.push(Dice::new(1));
        chain.push(FingerGuessing::Rock);
        chain.push(MarketFace {
            name: "xx".into(),
            ..Default::default()
        });
        chain.push(LightApp::new("{}".into()));
        println!("{}", chain);
        println!("{:?}", chain.reply());
        println!("{:?}", chain.anonymous());
        for item in chain {
            println!("{:?}", item)
        }
    }

    #[test]
    fn test_builder() {
        let mut builder = MessageChainBuilder::new();
        builder.push(Anonymous::default());
        builder.push(Reply::default());
        builder.push_str("hello");
        builder.push(At::new(12345));
        builder.push_str("world");
        builder.push(Face::new(1));
        builder.push(Dice::new(1));
        builder.push(Text::new("hello".into()));
        builder.push_str("world2");
        builder.push(FingerGuessing::Rock);
        builder.push(MarketFace {
            name: "xx".into(),
            ..Default::default()
        });
        builder.push(LightApp::new("{}".into()));
        let chain = builder.build();
        println!("{}", chain);
        assert!(chain.reply().is_some());
        assert!(chain.anonymous().is_some());
        for item in chain {
            println!("{:?}", item)
        }
    }
}
