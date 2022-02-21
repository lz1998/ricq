use std::fmt;

use crate::command::common::PbToBytes;
use crate::pb::msg;
use crate::pb::msg::CustomFace;

#[derive(Default, Debug, Clone)]
pub struct GroupImage {
    pub image_id: String,
    pub file_id: i64,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub md5: Vec<u8>,
    pub url: String,

    pub flash: bool,
}

impl From<GroupImage> for Vec<msg::elem::Elem> {
    fn from(e: GroupImage) -> Self {
        let mut cface = msg::CustomFace {
            file_type: Some(66),
            useful: Some(1),
            biz_type: Some(5),
            width: Some(e.width),
            height: Some(e.height),
            file_id: Some(e.file_id as i32),
            file_path: Some(e.image_id),
            // TODO decode type
            image_type: Some(1000),
            size: Some(e.size),
            flag: Some(vec![0; 4]),
            ..Default::default()
        };
        if e.flash {
            let flash = msg::MsgElemInfoServtype3 {
                flash_troop_pic: Some(cface),
                ..Default::default()
            }
            .to_bytes();
            let flash_elem = msg::elem::Elem::CommonElem(msg::CommonElem {
                service_type: Some(3),
                pb_elem: Some(flash.to_vec()),
                ..Default::default()
            });
            let text_hint = msg::elem::Elem::Text(msg::Text {
                str: Some("[闪照]请使用新版手机QQ查看闪照。".to_owned()),
                ..Default::default()
            });
            vec![flash_elem, text_hint]
        } else {
            cface.pb_reserve = Some(msg::ResvAttr::default().to_bytes().to_vec());
            vec![msg::elem::Elem::CustomFace(cface)]
        }
    }
}

impl From<msg::CustomFace> for GroupImage {
    fn from(custom_face: CustomFace) -> Self {
        if custom_face.md5().is_empty() {
            return Self::default();
        }
        let url = if let Some(orig_url) = &custom_face.orig_url {
            format!("https://gchat.qpic.cn{}", orig_url)
        } else {
            format!(
                "https://gchat.qpic.cn/gchatpic_new/0/0-0-{}{}",
                calculate_image_resource_id(&custom_face.md5()[1..37], true),
                "/0?term=2"
            )
        };
        // guild image todo
        return Self {
            file_id: custom_face.file_id() as i64,
            image_id: custom_face.file_path().to_owned(),
            size: custom_face.size(),
            width: custom_face.width(),
            height: custom_face.height(),
            url,
            md5: custom_face.md5.unwrap_or_default(),
            flash: false,
        };
    }
}

impl TryFrom<msg::CommonElem> for GroupImage {
    type Error = msg::CommonElem;

    fn try_from(e: msg::CommonElem) -> Result<Self, Self::Error> {
        if let Ok(pb_elem) = msg::MsgElemInfoServtype3::from_bytes(e.pb_elem()) {
            if let Some(p) = pb_elem.flash_troop_pic {
                let mut group_image: GroupImage = p.into(); //todo:url check
                group_image.flash = true;
                return Ok(group_image);
            }
        }
        Err(e)
    }
}

fn to_uuid(md5: &str) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        &md5[0..8],
        &md5[8..12],
        &md5[12..16],
        &md5[16..20],
        &md5[20..32],
    )
}

pub fn calculate_image_resource_id(md5: &[u8], no_dash: bool) -> String {
    let mut r = "{".to_owned();
    let md5 = crate::hex::encode_hex(md5).to_uppercase();
    if no_dash {
        r.push_str(&md5);
    } else {
        r.push_str(&to_uuid(&md5));
    }
    r.push_str("}.png");
    r
}

impl fmt::Display for GroupImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[GroupImage: {}]", self.url)
    }
}
