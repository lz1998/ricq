use std::fmt;

use elem::RQElem;
use elem::*;

use crate::pb::msg;

pub mod elem;
mod fragment;
mod macros;

pub type MessageElem = msg::elem::Elem;

/// [`MessageChain`]消息链, 用于发送消息
///
///
/// ## 示例
///
/// ```rust
/// use ricq_core::msg::elem::{At, Text};
/// use ricq_core::msg::MessageChain;
/// let mut chain = MessageChain::default();
/// chain.push(Text::new(String::from("Hello")));
/// chain.push(At::new(12345));
/// chain.push(Text::new(String::from("world")));
/// ```
///
/// 另请参阅: [`MessageChainBuilder`]
///
#[derive(Debug, Default, Clone)]
pub struct MessageChain(pub Vec<MessageElem>);

impl MessageChain {
    /// 从消息元素构造[`MessageChain`]
    ///
    /// ## 示例
    ///
    /// ```rust
    /// use ricq_core::msg::elem::Text;
    /// use ricq_core::msg::MessageChain;
    /// let chain = MessageChain::new(Text::new(String::from("Hello world!")));
    /// ```
    ///
    pub fn new<E: Into<Vec<MessageElem>>>(e: E) -> Self {
        Self(e.into())
    }

    /// 将消息元素添加至[`MessageChain`]
    ///
    /// ## 示例
    ///
    /// ```rust
    /// use ricq_core::msg::elem::Text;
    /// use ricq_core::msg::MessageChain;
    /// let mut chain = MessageChain::default();
    /// chain.push(Text::new(String::from("Hello")));
    /// ```
    ///
    pub fn push<E: Into<Vec<MessageElem>>>(&mut self, e: E) {
        self.0.extend(e.into())
    }

    pub fn anonymous(&self) -> Option<Anonymous> {
        self.0.iter().find_map(|e| match e {
            MessageElem::AnonGroupMsg(anonymous) => Some(Anonymous::from(anonymous.clone())),
            _ => None,
        })
    }

    /// 获取此[`MessageChain`]中的引用回复
    pub fn reply(&self) -> Option<Reply> {
        self.0.iter().find_map(|e| match e {
            MessageElem::SrcMsg(src_msg) => Some(Reply::from(src_msg.clone())),
            _ => None,
        })
    }

    pub fn with_anonymous(&mut self, anonymous: Anonymous) {
        self.0.insert(0, MessageElem::from(anonymous))
    }

    /// 添加引用回复
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
            .filter(|e| !matches!(e, MessageElem::SrcMsg(_) | MessageElem::AnonGroupMsg(_)))
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

/// [`MessageChain`]构造器
///
///
/// ## 示例
///
/// ```rust
/// use ricq_core::msg::elem::At;
/// use ricq_core::msg::MessageChainBuilder;
/// let mut builder = MessageChainBuilder::new();
/// builder.push_str("Hello")
///     .push(At::new(12345))
///     .push_str("world");
/// let chain = builder.build();
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct MessageChainBuilder {
    pub elems: Vec<MessageElem>,
    pub buf_string: String,
}

impl MessageChainBuilder {
    /// 创建一个新的[`MessageChainBuilder`]
    ///
    /// ## 示例
    /// ```rust
    /// use ricq_core::msg::MessageChainBuilder;
    /// let mut builder = MessageChainBuilder::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// 为当前[`MessageChainBuilder`]添加一个消息元素
    ///
    /// ## 示例
    /// ```rust
    /// use ricq_core::msg::elem::Text;
    /// use ricq_core::msg::MessageChainBuilder;
    /// let mut builder = MessageChainBuilder::new();
    /// builder.push(Text::new(String::from("Hello world!")));
    /// ```
    ///
    pub fn push<E: PushBuilder>(&mut self, elem: E) -> &mut Self {
        E::push_builder(elem, self);
        self
    }

    /// 向当前[`MessageChainBuilder`]的添加一段字符串
    ///
    /// 本函数会将字符串直接添加于[`MessageChainBuilder`]内部的字符串缓存，在每次push其他元素时刷新
    ///
    /// ## 示例
    /// ```rust
    /// use ricq_core::msg::MessageChainBuilder;
    /// let mut builder = MessageChainBuilder::new();
    /// builder.push_str("Hello world!");
    /// ```
    ///
    pub fn push_str(&mut self, str: &str) -> &mut Self {
        self.buf_string.push_str(str);
        self
    }

    /// 将此[`MessageChainBuilder`]构造为[`MessageChain`]
    ///
    /// ## 示例
    /// ```rust
    /// use ricq_core::msg::{MessageChain, MessageChainBuilder};
    /// let mut builder = MessageChainBuilder::new();
    /// let chain: MessageChain = builder.build();
    /// ```
    ///
    pub fn build(mut self) -> MessageChain {
        self.flush();
        MessageChain::new(self.elems)
    }

    /// 清空内部字符串缓存, 将其构造为消息元素
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
            println!("{e:?}")
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
        println!("{chain}");
        println!("{:?}", chain.reply());
        println!("{:?}", chain.anonymous());
        for item in chain {
            println!("{item:?}")
        }
    }

    #[test]
    fn test_builder() {
        let mut builder = MessageChainBuilder::new();
        builder
            .push(Anonymous::default())
            .push(Reply::default())
            .push_str("hello")
            .push(At::new(12345))
            .push_str("world")
            .push(Face::new(1))
            .push(Dice::new(1))
            .push(Text::new("hello".into()))
            .push_str("world2")
            .push(FingerGuessing::Rock)
            .push(MarketFace {
                name: "xx".into(),
                ..Default::default()
            })
            .push(LightApp::new("{}".into()));

        let chain = builder.build();
        println!("{chain}");
        assert!(chain.reply().is_some());
        assert!(chain.anonymous().is_some());
        for item in chain {
            println!("{item:?}")
        }
    }
}
