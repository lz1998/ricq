use std::fmt;

use elem::RQElem;

use crate::pb::msg;

pub mod elem;

#[derive(Debug, Default, Clone)]
pub struct MessageChain(pub Vec<msg::elem::Elem>);

impl MessageChain {
    pub fn iter(&self) -> impl Iterator<Item = RQElem> + '_ {
        self.0.iter().map(|e| RQElem::from(e.to_owned()))
    }

    pub fn into_iter(self) -> impl Iterator<Item = RQElem> + 'static {
        self.0.into_iter().map(|e| RQElem::from(e.to_owned()))
    }

    pub fn push<E: Into<Vec<msg::elem::Elem>>>(&mut self, e: E) {
        self.0.extend(e.into())
    }
}

impl From<Vec<msg::Elem>> for MessageChain {
    fn from(elements: Vec<msg::Elem>) -> Self {
        Self(elements.into_iter().filter_map(|e| e.elem).collect())
    }
}

impl Into<Vec<msg::Elem>> for MessageChain {
    // TODO https://github.com/mamoe/mirai/blob/f95482989d7a27cfe62004276601f616ccb55cf8/mirai-core/src/commonMain/kotlin/message/messageToElems.kt#L261
    fn into(self) -> Vec<msg::Elem> {
        self.0
            .into_iter()
            .map(|e| msg::Elem { elem: Some(e) })
            .collect()
    }
}

impl fmt::Display for MessageChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.iter() {
            fmt::Display::fmt(&x, f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut chain = MessageChain::default();
        chain.push(elem::text::Text::new("hello".into()));
        for e in chain.into_iter() {
            println!("{:?}", e)
        }
    }

    #[test]
    fn test_display() {
        let mut chain = MessageChain::default();
        chain.push(elem::text::Text::new("hello".into()));
        chain.push(elem::at::At::new(12345));
        chain.push(elem::text::Text::new("world".into()));
        chain.push(elem::face::Face::new(1));
        chain.push(elem::market_face::Dice::new(1));
        chain.push(elem::market_face::FingerGuessing::Rock);
        chain.push(elem::market_face::MarketFace {
            name: "xx".into(),
            ..Default::default()
        });
        println!("{}", chain)
    }
}
