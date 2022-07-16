#[macro_export]
macro_rules! to_elem_vec_impl {
    ($t:ty) => {
        impl From<$t> for Vec<MessageElem> {
            fn from(e: $t) -> Self {
                let mut vec = vec![];
                <$t>::push_to(e, &mut vec);
                vec
            }
        }
    }
}

#[macro_export]
macro_rules! push_builder_impl {
    ($t:ty) => {
        impl PushBuilder for $t {
            fn push_builder(elem: Self, builder: &mut MessageChainBuilder) {
                builder.flush();
                Self::push_to(elem, &mut builder.elems);
            }
        }
    }
}