use std::ops::{Add, AddAssign};

use crate::pb::msg;

pub struct ElemChain(Vec<msg::Elem>);

impl ElemChain {
    pub fn push<T>(&mut self, elem: T)
    where
        T: Into<Vec<msg::Elem>>,
    {
        self.0.extend(elem.into());
    }

    #[allow(dead_code)]
    pub fn freeze(self) -> Vec<msg::Elem> {
        self.0
    }
}

impl<T> Add<T> for ElemChain
where
    T: Into<Vec<msg::Elem>>,
{
    type Output = ElemChain;

    fn add(mut self, rhs: T) -> Self::Output {
        self.push(rhs);
        self
    }
}

impl<T> AddAssign<T> for ElemChain
where
    T: Into<Vec<msg::Elem>>,
{
    fn add_assign(&mut self, rhs: T) {
        self.push(rhs);
    }
}
