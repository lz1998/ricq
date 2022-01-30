use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct MarketFace {
    pub name: String,
    pub face_id: Vec<u8>,
    pub tab_id: i32,
    pub item_type: i32,
    pub sub_type: i32,
    pub media_type: i32,
    pub encrypt_key: Vec<u8>,
    pub magic_value: String,
}

// TODO fix push message chain
impl Into<Vec<msg::Elem>> for MarketFace {
    fn into(self) -> Vec<msg::Elem> {
        vec![
            msg::Elem {
                elem: Some(msg::elem::Elem::MarketFace(msg::MarketFace {
                    face_name: Some(self.name.as_bytes().to_vec()),
                    item_type: Some(self.item_type as u32),
                    face_info: Some(1),
                    face_id: Some(self.face_id),
                    tab_id: Some(self.tab_id as u32),
                    sub_type: Some(self.sub_type as u32),
                    key: Some(self.encrypt_key),
                    media_type: Some(self.media_type as u32),
                    image_width: Some(200),
                    image_height: Some(200),
                    mobileparam: Some(self.magic_value.as_bytes().to_vec()),
                    ..Default::default()
                })),
            },
            msg::Elem {
                elem: Some(msg::elem::Elem::Text(msg::Text {
                    str: Some(self.name),
                    ..Default::default()
                })),
            },
        ]
    }
}

impl From<msg::MarketFace> for MarketFace {
    fn from(e: msg::MarketFace) -> Self {
        Self {
            name: String::from_utf8(e.face_name.unwrap_or_default()).unwrap_or_default(),
            face_id: e.face_id.unwrap_or_default(),
            tab_id: e.tab_id.unwrap_or_default() as i32,
            item_type: e.item_type.unwrap_or_default() as i32,
            sub_type: e.sub_type.unwrap_or_default() as i32,
            media_type: e.media_type.unwrap_or_default() as i32,
            encrypt_key: e.key.unwrap_or_default(),
            magic_value: String::from_utf8(e.mobileparam.unwrap_or_default()).unwrap_or_default(),
        }
    }
}
