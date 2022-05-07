use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoInfo {
    pub file_name: String,
    pub file_md5: Vec<u8>,
    pub file_size: i64,
    pub thumb_file_md5: Vec<u8>,
    pub thumb_file_size: i64,
}
