/// Converts an [i8] array into a String.
pub fn i8_array_to_string(buf: &[i8]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    let bytes: Vec<u8> = buf[..len].iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}
