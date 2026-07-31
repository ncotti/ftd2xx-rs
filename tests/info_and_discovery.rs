//! This file tests
//! 
//! 
use std::io::{self, Write};
use ftd2xx_rs::{FtError, classic};
use ftd2xx_rs::types::*;
use ftd2xx_rs::device_type::*;

fn press_button_to_continue(message: &str) {
    println!("{}", message);
    print!("Press ENTER to continue...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

#[test]
#[ignore = "Should only be manually run with no devices connected"]
fn scanning_with_no_devices_connected() -> Result<(), FtError> {
    press_button_to_continue("Disconnect all devices.");
    let device_number = classic::create_device_info_list()?;
    assert!(device_number == 0);
    println!("{}", device_number);
    let devices = classic::get_device_info_list(device_number)?;
    assert!(devices.is_empty());
    Ok(())
}

#[test]
fn scanning_an_ft4232h() -> Result<(), FtError> {
    let device_number = classic::create_device_info_list()?;
    assert!(device_number == 4);
    let devices = classic::get_device_info_list(device_number)?;
    assert!(devices.len() == 4);
    for device in devices {
        assert!(device.dev_type == DeviceType::Dev4232H);
    }
    Ok(())
}