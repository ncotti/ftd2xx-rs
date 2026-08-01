#![warn(missing_docs)]

//! Crate doc
//!

pub mod classic;
pub mod my_type;
pub mod types;

pub use classic::*;
pub use my_type::Version;

pub use types::{DevInfo, FtError};

/// Scans all currently connected devices, and returns a list with their
/// information. By default, only devices with VID=0x0403 and
/// PID = {0x6001, 0x6010, 0x6006} are listed. To use a custom {VID, PID} tuple,
/// use the `scan_custom()` function instead.
///
/// Example:
/// ```
/// use ftd2xx_rs::scan;
///
/// let device_infos: Vec<DevInfo> = scan();
/// for info in device_infos {
///     println!("{}", info);
/// }
/// ```
pub fn scan() -> Result<Vec<DevInfo>, FtError> {
    let device_qtty = classic::create_device_info_list()?;

    let device_infos: Vec<DevInfo> = if device_qtty > 0 {
        classic::get_device_info_list(device_qtty)?
    } else {
        Vec::new()
    };
    Ok(device_infos)
}

/// Scans all currently connected devices, and returns a list with their
/// information. It will also search for the custom vendor_id (vid) and
/// product_id (pid) tuple, besides the default ones.
///
/// Example:
/// ```
/// use ftd22_rs::scan_custom;
///
/// let devices_infos: Vec<DevInfo> = scan_custom(0xABCD, 0x1234);
/// for info in device_infos {
///     println!("{}", info);
/// }
/// ```
pub fn scan_custom(vid: u16, pid: u16) -> Result<Vec<DevInfo>, FtError> {
    classic::set_vid_pid(vid, pid)?;
    scan()
}

// pub struct Device {
//     handle: FtHandle,
//     info: bool,
//     eeprom: bool,
//     version: Version,
// }
