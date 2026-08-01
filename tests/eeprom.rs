//! Tests eeprom functionality

use std::str::from_utf8;

use ftd2xx_rs::{FtError, classic, ee_ua_size, types::{FtHandle, MyProgramData}};
use rand::{Rng, RngExt};
use regex::bytes;

/// Change serial number of the device, read it back, and return to original
/// state
#[test]
fn test_eeprom_by_changing_serial_number() -> Result<(), FtError> {
    let handle: FtHandle = classic::open(0).unwrap();
    let mut old_eeprom_data = classic::ee_read(handle).unwrap();
    assert!(old_eeprom_data.program_data.Signature1 == 0x00000000);
    assert!(old_eeprom_data.program_data.Signature2 == 0xFFFFFFFF);
    assert!(old_eeprom_data.program_data.Version == 4);

    let old_serial_number:Box<[u8; 64]> = old_eeprom_data.serial_number.clone();
    let new_serial_number = "cotti_serial";

    old_eeprom_data.set_serial_number(new_serial_number);

    println!("{:?}", old_serial_number);
    println!("{:?}", old_eeprom_data.serial_number);

    classic::ee_program(handle, &mut old_eeprom_data)?;

    let new_eeprom_data = classic::ee_read(handle).unwrap();
    println!("{:?}", new_eeprom_data.serial_number);
    println!("{:?}", new_serial_number.as_bytes());

    assert!(&new_eeprom_data.serial_number[..new_serial_number.len()] == new_serial_number.as_bytes());

    old_eeprom_data.set_serial_number(str::from_utf8(old_serial_number.as_slice()).unwrap());
    classic::ee_program(handle, &mut old_eeprom_data)?;

    let new_eeprom_data = classic::ee_read(handle).unwrap();
    assert!(new_eeprom_data.serial_number == old_serial_number);

    classic::close(handle)?;
    Ok(())
}

/// Change a configuration parameter in the eeprom, read it back, and return
/// to original state
#[test]
fn test_eeprom_by_changing_configuration() -> Result<(), FtError> {
    let handle: FtHandle = classic::open(0).unwrap();
    let mut old_eeprom_data = classic::ee_read(handle).unwrap();
    assert!(old_eeprom_data.program_data.Signature1 == 0x00000000);
    assert!(old_eeprom_data.program_data.Signature2 == 0xFFFFFFFF);
    assert!(old_eeprom_data.program_data.Version == 4);

    let old_drive_current = old_eeprom_data.program_data.ADriveCurrent;
    let new_drive_current = if old_drive_current == 16 {4} else {old_drive_current + 4};

    old_eeprom_data.program_data.ADriveCurrent = new_drive_current;
    classic::ee_program(handle, &mut old_eeprom_data)?;

    let new_eeprom_data = classic::ee_read(handle).unwrap();
    assert!(new_eeprom_data.program_data.ADriveCurrent == new_drive_current);

    old_eeprom_data.program_data.ADriveCurrent = old_drive_current;
    classic::ee_program(handle, &mut old_eeprom_data)?;

    let new_eeprom_data = classic::ee_read(handle).unwrap();
    assert!(new_eeprom_data.program_data.ADriveCurrent == old_drive_current);

    classic::close(handle)?;
    Ok(())
}

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
