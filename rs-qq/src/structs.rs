use crate::engine::hex::encode_hex;
use crate::{RQError, RQResult};
use rq_engine::msg::elem::{calculate_image_resource_id, FriendImage, GroupImage};
use tokio::sync::RwLock;

pub use crate::engine::structs::*;

// TODO 大群会占用大量内存，可以考虑提供 trait，用磁盘存储
#[derive(Default, Debug)]
pub struct Group {
    pub info: GroupInfo,
    pub members: RwLock<Vec<GroupMemberInfo>>,
}

pub struct ImageInfo {
    pub md5: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub image_type: i32,
    pub size: u32,
    pub filename: String,
    pub format: image::ImageFormat,
}

impl ImageInfo {
    pub fn try_new(data: &[u8]) -> RQResult<Self> {
        let img_reader = image::io::Reader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .map_err(RQError::IO)?;
        let format = img_reader.format().unwrap_or(image::ImageFormat::Png);
        let md5 = md5::compute(data).to_vec();

        let (width, height) = img_reader.into_dimensions().unwrap_or((720, 480));
        Ok(ImageInfo {
            filename: format!(
                "{}.{}",
                encode_hex(&md5),
                format.extensions_str().first().expect("image_format error")
            ),
            md5,
            width,
            height,
            image_type: match format {
                image::ImageFormat::Jpeg => 1000,
                image::ImageFormat::Png => 1001,
                image::ImageFormat::WebP => 1002,
                image::ImageFormat::Bmp => 1005,
                image::ImageFormat::Gif => 2000,
                _ => 1000,
            },
            size: data.len() as u32,
            format,
        })
    }

    pub fn into_friend_image(self, image_id: String) -> FriendImage {
        FriendImage {
            image_id,
            md5: self.md5,
            size: self.size as i32,
            ..Default::default()
        }
    }

    pub fn into_group_image(self, file_id: u64) -> GroupImage {
        GroupImage {
            image_id: calculate_image_resource_id(&self.md5, false),
            file_id: file_id as i64,
            size: self.size as i32,
            width: self.width as i32,
            height: self.height as i32,
            md5: self.md5,
            image_type: self.image_type,
            ..Default::default()
        }
    }
}
