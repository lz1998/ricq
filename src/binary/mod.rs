mod binary_reader;
mod binary_writer;

pub use binary_reader::BinaryReader;
pub use binary_writer::BinaryWriter;

pub(crate) fn calculate_image_resource_id(md5: &[u8], no_dash: bool) -> String {
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
