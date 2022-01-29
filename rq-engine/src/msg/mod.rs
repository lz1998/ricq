use crate::pb::msg;
use crate::pb::msg::Elem;
use elem::RQElem;

pub mod elem;

#[derive(Debug, Default, Clone)]
pub struct MessageChain(pub Vec<msg::Elem>);

impl MessageChain {
    pub fn iter(&self) -> impl Iterator<Item = RQElem> + '_ {
        self.0.iter().map(|e| RQElem::from(e.to_owned()))
    }

    pub fn into_iter(self) -> impl Iterator<Item = RQElem> + 'static {
        self.0.into_iter().map(|e| RQElem::from(e.to_owned()))
    }

    pub fn push<E: Into<msg::Elem>>(&mut self, e: E) {
        self.0.push(e.into())
    }
}

impl Into<Vec<msg::Elem>> for MessageChain {
    fn into(self) -> Vec<Elem> {
        self.0
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
}
