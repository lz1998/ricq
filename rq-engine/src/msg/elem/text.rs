use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Into<msg::Elem> for Text {
    fn into(self) -> msg::Elem {
        msg::Elem {
            elem: Some(msg::elem::Elem::Text(msg::Text {
                str: Some(self.content),
                ..Default::default()
            })),
        }
    }
}

impl From<msg::Text> for Text {
    fn from(e: msg::Text) -> Self {
        Self {
            content: e.str.unwrap_or_default(),
        }
    }
}
