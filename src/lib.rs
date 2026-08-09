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
/// Example: TODO doctest
///
/// use ftd2xx_rs::scan;
/// use ftd2xx_rs::DevInfo;
///
/// let device_infos: Vec<DevInfo> = scan();
/// for info in device_infos {
///     println!("{}", info);
/// }
///
pub fn scan() -> Result<Vec<DevInfo>, FtError> {
    let device_qtty = classic::create_device_info_list()?;
    let device_infos: Vec<DevInfo> = classic::get_device_info_list(device_qtty)?;
    Ok(device_infos)
}

/// Scans all currently connected devices, and returns a list with their
/// information. It will also search for the custom vendor_id (vid) and
/// product_id (pid) tuple, besides the default ones.
///
/// Example: TODO doctest
///
/// #[cfg(feature = "test-ft4232h")]
/// use ftd2xx_rs::scan_custom;
///
/// let devices_infos: Vec<DevInfo> = scan_custom(0xABCD, 0x1234);
/// for info in device_infos {
///     println!("{}", info);
/// }
///
pub fn scan_custom(vid: u16, pid: u16) -> Result<Vec<DevInfo>, FtError> {
    classic::set_vid_pid(vid, pid)?;
    scan()
}

/// Generic device
#[derive(Debug)]
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

    /// Open a device using an `u32` value. It will be interpreted first
    /// as an index, and, it doesn't work, as an USB location index.
    fn try_from(value: u32) -> Result<Self, FtError> {
        let infos = scan()?;

        let info = if value < infos.len() as u32 {
            infos.into_iter().nth(value as usize)
        } else {
            infos.into_iter().find(|info| info.usb_location_id == value)
        };

        if info.is_none() {
            return Err(FtError::DeviceNotFound);
        }

        let info = info.expect("Already checked that it is Some()");
        Self::try_from(info)
    }
}

impl TryFrom<&str> for Device {
    type Error = FtError;

    /// Open a device using the description or the serial number.
    fn try_from(description: &str) -> Result<Self, FtError> {
        if description.is_empty() {
            return Err(FtError::DeviceNotFound);
        }

        let infos = scan()?;

        let info = infos
            .into_iter()
            .find(|info| info.description == description);

        if info.is_none() {
            return Err(FtError::DeviceNotFound);
        }

        let info = info.expect("Already checked that it is Some()");
        Self::try_from(info)
    }
}

impl TryFrom<String> for Device {
    type Error = FtError;

    fn try_from(description: String) -> Result<Self, FtError> {
        Self::try_from(&description)
    }
}

impl TryFrom<&String> for Device {
    type Error = FtError;

    fn try_from(description: &String) -> Result<Self, FtError> {
        let description: &str = description;
        Self::try_from(description)
    }
}

impl TryFrom<DevInfo> for Device {
    type Error = FtError;

    /// Open a device using the `DevInfo` obtained from a previous `scan()`.
    fn try_from(info: DevInfo) -> Result<Self, FtError> {
        Self::try_from(&info)
    }
}

impl TryFrom<&DevInfo> for Device {
    type Error = FtError;

    /// Open a device using the `DevInfo` obtained from a previous `scan()`.
    fn try_from(info: &DevInfo) -> Result<Self, FtError> {
        let mut info = info.clone();
        info.open = true;

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
        classic::close(self.handle).unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DevType;
    use regex::Regex;

    /// Get library version
    #[test]
    fn test_get_library_version() {
        let version = get_library_version().unwrap();
        let re = Regex::new(r"^v\d{1,2}\.\d{1,2}\.\d{1,2}$").unwrap();
        println!("{}", version);
        assert!(re.is_match(&version.to_string()));
    }

    /// Scan function should return an error if no device is connected.
    #[cfg(feature = "test-dc")]
    #[test]
    fn test_scan_with_no_devices() {
        let ret = scan().unwrap_err();
        assert!(ret == FtError::DeviceNotFound)
    }

    /// The four channels are discovered as different devices
    #[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_scan() -> Result<(), FtError> {
        let devices = scan()?;
        assert!(devices.len() == 4);
        for device in devices {
            assert!(device.dev_type == DevType::Dev4232H);
            assert!(device.open == false);
        }
        Ok(())
    }

    /// Connects to the device. It should be listed as open when scanned.
    /// You shouldn't be able to connect twice.
    #[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_connect_to_device() -> Result<(), FtError> {
        let devices = scan()?;
        assert!(devices[0].open == false);

        println!("Trying to open from dev: {}", devices[0]);
        let probe = Device::try_from(&devices[0])?;
        assert!(probe.info.usb_location_id == devices[0].usb_location_id);
        assert!(probe.info.open == true);

        // New scan should reveal that the device has been opened
        let new_devices = scan()?;
        assert!(new_devices[0].open == true);

        // Trying to create a new device from an opened one should return an
        // error
        let double_probe = Device::try_from(&new_devices[0]).unwrap_err();
        assert!(double_probe == FtError::DeviceNotOpened);

        // Dropping the device and scanning again should be listed as closed
        drop(probe);
        let devices = scan()?;
        for device in devices {
            assert!(device.open == false);
        }

        Ok(())
    }

    #[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_connect_with_description() -> Result<(), FtError> {
        let devices = scan()?;
        assert!(devices[0].open == false);

        let probe: Device = Device::try_from(&devices[0].description)?;
        assert!(probe.info.open == true);
        assert!(probe.info.description == devices[0].description);

        let wrong_description = "This description is wrong";
        let wrong_probe = Device::try_from(wrong_description).unwrap_err();
        assert!(wrong_probe == FtError::DeviceNotFound);

        Ok(())
    }

    #[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_connect_with_serial_number() -> Result<(), FtError> {
        let devices = scan()?;

        if devices[0].serial_number.is_empty() {
            todo!("Write the eeprom")
        }

        let probe = Device::try_from(&devices[0].serial_number)?;
        assert!(probe.info.usb_location_id == devices[0].usb_location_id);

        Ok(())
    }

    #[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_connect_with_numbers() -> Result<(), FtError> {
        let devices = scan()?;

        assert!(devices.len() == 4);
        let device = Device::try_from(3)?;
        assert!(device.info.usb_location_id == devices[3].usb_location_id);

        // Device index out of scope, or also wrong usb index
        let device_wrong = Device::try_from(8).unwrap_err();
        assert!(device_wrong == FtError::DeviceNotFound);

        let device2 = Device::try_from(devices[1].usb_location_id)?;
        assert!(device2.info.usb_location_id == devices[1].usb_location_id);

        Ok(())
    }
}