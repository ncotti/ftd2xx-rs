//! Device information structure

use crate::types::dev_type::DevType;
use crate::utils::i8_array_to_string;
use ftd2xx_sys::{d2xx, mpsse_i2c, mpsse_spi};
use std::fmt;

/// Default Vendor ID for FTDI
pub const FT_DEFAULT_VENDOR_ID: u16 = 0x403;

/// Default PRODUCT ID.
pub const FT_DEFAULT_PRODUCT_ID: u16 = 0x6001;

/// FT device information as returned by the `scan()` methods.
#[derive(Debug, Clone)]
pub struct DevInfo {
    /// If "true", the device's port is open.
    pub open: bool,

    /// If "true", the device is enumerated as a high-speed USB device (480 Mb/s),
    /// if "false", is a full-speed USB device (12 Mb/s).
    pub high_speed_usb: bool,

    /// Device type.
    pub dev_type: DevType,

    /// Vendor ID.
    pub vid: u16,

    /// Product ID.
    pub pid: u16,

    /// USB location ID. It is a way to uniquely identify a device.
    pub usb_location_id: u32,

    /// Device's serial number, as stored in the EEPROM. This string will be
    /// empty, unless the `SerNumEnableX` flag in the EEPROM is set to `true`.
    pub serial_number: String,

    /// Device's description, as stored in the EEPROM.
    pub description: String,
}

impl From<d2xx::FT_DEVICE_LIST_INFO_NODE> for DevInfo {
    fn from(ft_device_info: d2xx::FT_DEVICE_LIST_INFO_NODE) -> Self {
        Self {
            open: (ft_device_info.Flags & 0b01 != 0),
            high_speed_usb: (ft_device_info.Flags & 0b10 != 0),
            dev_type: DevType::from(ft_device_info.Type as u8),
            vid: ((ft_device_info.ID >> 16) & 0xFFFF) as u16,
            pid: (ft_device_info.ID & 0xFFFF) as u16,
            usb_location_id: ft_device_info.LocId,
            serial_number: i8_array_to_string(&ft_device_info.SerialNumber),
            description: i8_array_to_string(&ft_device_info.Description),
        }
    }
}

impl From<mpsse_i2c::FT_DEVICE_LIST_INFO_NODE> for DevInfo {
    fn from(ft_device_info: mpsse_i2c::FT_DEVICE_LIST_INFO_NODE) -> Self {
        Self {
            open: (ft_device_info.Flags & 0b01 != 0),
            high_speed_usb: (ft_device_info.Flags & 0b10 != 0),
            dev_type: DevType::from(ft_device_info.Type as u8),
            vid: ((ft_device_info.ID >> 16) & 0xFFFF) as u16,
            pid: (ft_device_info.ID & 0xFFFF) as u16,
            usb_location_id: ft_device_info.LocId,
            serial_number: i8_array_to_string(&ft_device_info.SerialNumber),
            description: i8_array_to_string(&ft_device_info.Description),
        }
    }
}

impl From<mpsse_spi::FT_DEVICE_LIST_INFO_NODE> for DevInfo {
    fn from(ft_device_info: mpsse_spi::FT_DEVICE_LIST_INFO_NODE) -> Self {
        Self {
            open: (ft_device_info.Flags & 0b01 != 0),
            high_speed_usb: (ft_device_info.Flags & 0b10 != 0),
            dev_type: DevType::from(ft_device_info.Type as u8),
            vid: ((ft_device_info.ID >> 16) & 0xFFFF) as u16,
            pid: (ft_device_info.ID & 0xFFFF) as u16,
            usb_location_id: ft_device_info.LocId,
            serial_number: i8_array_to_string(&ft_device_info.SerialNumber),
            description: i8_array_to_string(&ft_device_info.Description),
        }
    }
}

impl fmt::Display for DevInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.dev_type)?;
        writeln!(f, "VID: {:#X}, PID: {:#X}", self.vid, self.pid)?;
        if !self.serial_number.is_empty() {
            writeln!(f, "Serial number: {}", self.serial_number)?;
        }
        if !self.description.is_empty() {
            writeln!(f, "Description: {}", self.description)?;
        }
        let status = if self.open { "Open" } else { "Closed" };
        let usb_type = if self.high_speed_usb {
            "High-Speed USB (480 Mb/s)"
        } else {
            "Full-Speed USB (12 Mb/s)"
        };

        writeln!(f, "Status: {}, {}", status, usb_type)?;
        write!(f, "USB Location ID: {:#X}", self.usb_location_id)
    }
}
