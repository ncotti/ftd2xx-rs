//! Type doc

use crate::device_type::DeviceType;
use ftd2xx_sys::*;
use std::{ffi::c_char, fmt};

/// Holds the current library version, as v.<major>.<minor>.<build>
#[derive(Debug, Copy, Clone)]
pub struct Version {
    major: u8,
    minor: u8,
    build: u8,
}

impl Version {
    /// Creates a new version struct from a "version number".
    /// E.g., version "v3.01.15 == v<major>.<minor>.<build>" is expected to be
    /// the number 0x00030115.
    pub fn new(version: u32) -> Self {
        Self {
            major: ((((version >> 20) & 0xf) * 10) + ((version >> 16) & 0xf)) as u8,
            minor: ((((version >> 12) & 0xf) * 10) + ((version >> 8) & 0xf)) as u8,
            build: ((((version >> 4) & 0xf) * 10) + ((version >> 0) & 0xf)) as u8,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.build)
    }
}

#[derive(Debug)]
pub struct DeviceInfo {
    /// If "true", the device's port is open.
    open: bool,

    /// If "true", the device is enumerated as a high-speed USB device (480 Mb/s),
    /// if "false", is a full-speed USB device (12 Mb/s).
    high_speed_usb: bool,

    /// Device type.
    dev_type: DeviceType,

    vid: u16,
    pid: u16,

    usb_location_id: u32,

    serial_number: [c_char; 16],
    description: String,

    handle: FT_HANDLE,
}

impl DeviceInfo {
    pub fn new(ft_device_info: FT_DEVICE_LIST_INFO_NODE) -> Self {
        Self {
            open: (ft_device_info.Flags & 0b0 != 0),
            high_speed_usb: (ft_device_info.Flags & 0b10 != 0),
            dev_type: DeviceType::from(ft_device_info.Type as u8),
            vid: ((ft_device_info.ID >> 16) & 0xFFFF) as u16,
            pid: (ft_device_info.ID & 0xFFFF) as u16,
            usb_location_id: ft_device_info.LocId,
            serial_number: ft_device_info.SerialNumber,
            description: i8_array_to_string(&ft_device_info.Description),
            handle: ft_device_info.ftHandle,
        }
    }
}

fn i8_array_to_string(buf: &[i8]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());

    let bytes: Vec<u8> = buf[..len].iter().map(|&b| b as u8).collect();

    String::from_utf8_lossy(&bytes).into_owned()
}
