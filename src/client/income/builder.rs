use crate::client::engine::decoder::online_push::GroupMessagePart;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct GroupMessageBuilder(BTreeMap<i32, GroupMessagePart>);

impl GroupMessageBuilder {
    pub fn new(part: GroupMessagePart) -> Self {
        Self(BTreeMap::from([(part.pkg_index, part)]))
    }

    /// insert a part into builder, if it's already finish, return built message
    pub fn join(mut self, part: GroupMessagePart) -> Result<GroupMessagePart, Self> {
        let pkg_num = part.pkg_num;
        self.0.insert(part.pkg_index, part);
        if self.0.len() as i32 >= pkg_num {
            //? need check
            // let mut base = self.0.pop_first().unwrap().1; // unstable yet
            let mut base = GroupMessagePart::default();
            for (i, (_, part)) in self.0.into_iter().enumerate() {
                if i == 0 {
                    base = part;
                } else {
                    base.elems.extend(part.elems);
                }
            }
            Ok(base)
        } else {
            Err(self)
        }
    }
}
