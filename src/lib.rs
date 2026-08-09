#![warn(missing_docs)]

//! Crate doc
//!

pub mod classic;
pub mod eeprom;
pub mod types;
pub mod utils;

/// FtHandle is a void* FT_HANDLE. It is used to manage any FT device.
pub type FtHandle = ftd2xx_sys::FT_HANDLE;

pub use classic::*;

pub use types::{DevInfo, FtError, Version};

pub use eeprom::ft4232h::EepromFt4232h;

/// Scans all currently connected devices, and returns a list with their
/// information. By default, only devices with VID=0x0403 and
/// PID = {0x6001, 0x6010, 0x6006} are listed. To use a custom {VID, PID} tuple,
/// use the `scan_custom()` function instead.
///
/// Example:
/// ```
/// use ftd2xx_rs::scan;
/// use ftd2xx_rs::DevInfo;
///
/// let device_infos: Vec<DevInfo> = scan();
/// for info in device_infos {
///     println!("{}", info);
/// }
/// ```
pub fn scan() -> Result<Vec<DevInfo>, FtError> {
    let device_qtty = classic::create_device_info_list()?;
    let device_infos: Vec<DevInfo> = classic::get_device_info_list(device_qtty)?;
    Ok(device_infos)
}

/// Scans all currently connected devices, and returns a list with their
/// information. It will also search for the custom vendor_id (vid) and
/// product_id (pid) tuple, besides the default ones.
///
/// Example:
/// ```
/// use ftd2xx_rs::scan_custom;
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

/// Generic device
pub struct Device {
    /// FT Device info
    pub info: DevInfo,

    /// FTD2XX library version.
    pub version: Version,

    /// FT Device handle
    handle: FtHandle,
}

impl TryFrom<u32> for Device {
    type Error = FtError;

    /// Open a device using an `u32` value. It will be interpreted at first
    /// as an index, and, it doesn't work, as an USB location index.
    fn try_from(value: u32) -> Result<Self, FtError> {
        let infos = scan()?;

        if infos.len() == 0 {
            return Err(FtError::DeviceNotFound);
        }

        let info = if value < infos.len() as u32 {
            infos.into_iter().nth(value as usize)
        } else {
            infos.into_iter().find(|info| info.usb_location_id == value)
        };

        if info.is_none() {
            return Err(FtError::DeviceNotFound);
        }

        Self::try_from(info.unwrap())
    }
}

impl TryFrom<&str> for Device {
    type Error = FtError;

    /// Open a device using the description. The device description must match
    /// exactly.
    fn try_from(description: &str) -> Result<Self, FtError> {
        let infos = scan()?;

        if infos.len() == 0 {
            return Err(FtError::DeviceNotFound);
        }

        let info = infos
            .into_iter()
            .find(|info| info.description == description);

        if info.is_none() {
            return Err(FtError::DeviceNotFound);
        }

        Self::try_from(info.unwrap())
    }
}

impl TryFrom<DevInfo> for Device {
    type Error = FtError;

    /// Open a device using the `DevInfo` obtained from a previous `scan()`.
    fn try_from(info: DevInfo) -> Result<Self, FtError> {
        let version = classic::get_library_version()?;

        let handle = classic::open_ex_by_location(info.usb_location_id)?;

        let dev = Self {
            info: info,
            version: version,
            handle: handle,
        };

        Ok(dev)
    }
}

/// Close FT Handle on destructor.
impl Drop for Device {
    fn drop(&mut self) {
        unsafe { classic::close(self.handle).unwrap_err_unchecked() };
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    /// Setup: No devices connected
    /// Expected: Scan function should return an error.
    #[test]
    fn test_scan_with_no_devices() {
        utils::press_button_to_continue("Disconnect all devices");
    }

    /// Setup: FT4232H connected
    /// Expected: The four channels are discovered as different devices
    #[test]
    fn test_scan() {
        utils::press_button_to_continue("Have a singular FT4232H connected...");
    }

    /// Setup: Two FT4232H connected.
    /// Expected: 8 devices should be found, 2 devices, 4 channels each
    #[test]
    fn test_scan_multiple() {
        utils::press_button_to_continue("Have two FT4232H devices connected...");
    }

    
}