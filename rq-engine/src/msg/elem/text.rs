use std::fmt;

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

impl Into<Vec<msg::elem::Elem>> for Text {
    fn into(self) -> Vec<msg::elem::Elem> {
        vec![msg::elem::Elem::Text(msg::Text {
            str: Some(self.content),
            ..Default::default()
        })]
    }
}

impl From<msg::Text> for Text {
    fn from(e: msg::Text) -> Self {
        Self {
            content: e.str.unwrap_or_default(),
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
