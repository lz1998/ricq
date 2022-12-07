/// Optimized `from_utf8_lossy`, presumes that most of buffer is valid utf-8.
///
/// The `String::from_utf8_lossy` always allocate memory.
pub fn utf8_to_string(buf: impl Into<Vec<u8>>) -> String {
    let buf: Vec<u8> = buf.into(); // for `bytes::Bytes`
    match std::str::from_utf8(&buf) {
        Ok(_) => unsafe {
            println!("[utf8_to_string] pass");
            String::from_utf8_unchecked(buf)
        },
        Err(_) => {
            println!("[utf8_to_string] failback");
            String::from_utf8_lossy(&buf).into_owned() // don't use `.to_string()`
        }
    }
}
