mod de;
mod reader;
mod ser;
mod test;

pub use de::{Jce, JceGet};
pub use jce_derive::JcePut;
pub use reader::{JceObject, JceStruct};
pub use ser::{JceMut, JcePut, JceToObject};

#[macro_export]
macro_rules! JceStruct {
    ($struct_name: ident {
        $($tag:expr=>$item_name: ident: $item_type: ty,)*
    }) => {
        pub struct $struct_name {
            $(pub $item_name: $item_type,)*
        }

        impl JcePut for $struct_name {
            fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
                jce_mut.put_head(10, tag);
                self.put_raw(jce_mut);
                jce_mut.put_head(11, 0)
            }


            fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
                $(self.$item_name.put(jce_mut, $tag);)*
                jce_mut
            }
        }

        impl JceGet for $struct_name {
            fn read(jce: &mut Jce) -> Self {
                jce.new = true;
                $(let $item_name = jce.get_by_tag($tag);)*
                jce.end_object();
                $struct_name {
                    $($item_name,)*
                }
            }

            fn empty() -> Self {
                panic!("jce get empty, should have a object")
            }
        }
    };
}
