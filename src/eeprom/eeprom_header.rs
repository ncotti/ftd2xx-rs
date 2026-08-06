//! Doc

use crate::types::{DevType, FT_DEFAULT_PRODUCT_ID, FT_DEFAULT_VENDOR_ID};
use ftd2xx_sys::FT_EEPROM_HEADER;

/// Common EEPROM header used for all devices, based on the `FT_EEPROM_HEADER`.
#[derive(Debug)]
pub struct EepromHeader {
    /// Device type. This field is nor read or written to the EEPROM, but
    /// rather used by the FTD2XX library to know the EEPROM layout.
    device_type: DevType,

    /// Vendor ID.
    pub vid: u16,
    /// Product ID.
    pub pid: u16,

    /// If `true`, the `serial_number` will be announced when the USB device is
    /// connected and can be retrieved from a preliminary scan, without
    /// having to read it from the EEPROM. If `false`, it won't be visible.
    pub serial_number_enable: bool,

    /// Max power drawn from the USB interface. Valid values are: [0;500]
    /// If set to zero, the device will be `self_powered`, i.e. it won't
    /// draw any current from the USB interface, but rather use its own
    /// external power source.
    /// TODO, check, actual power: 2mA * value, this is current.
    pub max_power: u16,

    /// Enable remote wake-up. TODO
    pub remote_wakeup: bool,

    /// If `true`, the device's IO pins will be connected to an internal
    /// pulldown resistor when the in USB suspend mode.
    pub pulldown_enable: bool,
}

impl From<EepromHeader> for FT_EEPROM_HEADER {
    fn from(t: EepromHeader) -> Self {
        FT_EEPROM_HEADER {
            deviceType: t.device_type as u32,
            VendorId: t.vid,
            ProductId: t.pid,
            SerNumEnable: t.serial_number_enable as u8,
            MaxPower: t.max_power,
            SelfPowered: (t.max_power == 0) as u8,
            RemoteWakeup: t.remote_wakeup as u8,
            PullDownEnable: t.pulldown_enable as u8,
        }
    }
}

impl From<FT_EEPROM_HEADER> for EepromHeader {
    fn from(t: FT_EEPROM_HEADER) -> Self {
        EepromHeader {
            device_type: DevType::from(t.deviceType as u8),
            vid: t.VendorId,
            pid: t.ProductId,
            serial_number_enable: t.SerNumEnable != 0,
            max_power: t.MaxPower,
            remote_wakeup: t.RemoteWakeup != 0,
            pulldown_enable: t.PullDownEnable != 0,
        }
    }
}

impl EepromHeader {
    /// Create a new EEPROM Header configuration with sensible default values.
    pub fn new(device_type: DevType) -> Self {
        EepromHeader {
            device_type: device_type,
            vid: FT_DEFAULT_VENDOR_ID,
            pid: FT_DEFAULT_PRODUCT_ID,
            serial_number_enable: true,
            max_power: 250, // TODO, check if 500mA is a valid value
            remote_wakeup: false,
            pulldown_enable: true,
        }
    }
}
