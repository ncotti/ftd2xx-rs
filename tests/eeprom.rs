//! Tests eeprom functionality

use ftd2xx_rs::{
    FtError, classic, FtHandle,
};
use rand::{RngExt};

/// Write some value in EEPROM user area, and read it back
#[test]
fn test_user_area() -> Result<(), FtError> {
    let mut rng = rand::rng();

    let handle: FtHandle = classic::open(0).unwrap();
    let user_area_size = classic::ee_ua_size(handle).unwrap();

    assert!(user_area_size > 4);

    let mut expected_bytes: Vec<u8> = vec![rng.random(), rng.random(), rng.random(), rng.random()];

    classic::ee_ua_write(handle, &mut expected_bytes)?;
    let bytes_read = classic::ee_ua_read(handle).unwrap();

    println!("{}", user_area_size);
    println!("{:?}", bytes_read);
    println!("{:?}", expected_bytes);

    assert!(bytes_read.len() >= expected_bytes.len());
    for (expected_byte, read_byte) in std::iter::zip(expected_bytes, bytes_read) {
        assert!(expected_byte == read_byte);
    }

    classic::close(handle)?;
    Ok(())
}
