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

#[repr(u8)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4,
}

#[repr(u8)]
pub enum BitsPerWord {
    Bits7 = 7,
    Bits8 = 8,
}

#[repr(u8)]
pub enum StopBits {
    StopBits1 = 0,
    StopBits2 = 2,
}

pub struct UartInfo {
    pub parity: Parity,
    pub bits: BitsPerWord,
    pub stop_bits: StopBits,
}

#[repr(u16)]
pub enum FlowControl {
    None = FT_FLOW_NONE as u16,
    RtsCts = FT_FLOW_RTS_CTS as u16,
    DtrDsr = FT_FLOW_DTR_DSR as u16,
    XonXoff = FT_FLOW_XON_XOFF as u16,
}

pub struct EventCause {
    pub rx_char: bool,
    pub modem_status: bool,
    pub line_status: bool,
}

pub struct EepromCommon {
    signature_1: u32,
    signature_2: u32,
    version: u32,
    vid: u16,
    pid: u16,
    manufacturer: String,
    manufacturer_id: String,
    description: String,
    serial_number: String,
    max_power: u16,
    pnp: bool,
    self_powered: bool,
    remote_wakeup: bool,
}

pub struct EepromFT4232H {
    common: EepromCommon,
    enable_pulldown: bool,
    enable_serial_number: bool,
    cha: EepromFT4232HChannel,
    chb: EepromFT4232HChannel,
    chc: EepromFT4232HChannel,
    chd: EepromFT4232HChannel,
}

pub struct EepromFT4232HChannel {
    slow_slew: bool,
    schmitt_input: bool,
    drive_current: u8,
}

#[derive(Debug)]
pub struct MyProgramData {
    pub program_data: FT_PROGRAM_DATA
}


impl MyProgramData {
    pub fn new_default() -> Self {
        MyProgramData {
            program_data: FT_PROGRAM_DATA {
                // Common
                Signature1: 0x00000000,
                Signature2: 0xFFFFFFFF,
                Version: 4,
                VendorId: 0,
                ProductId: 0,
                Manufacturer: [0; 64].as_mut_ptr(),
                ManufacturerId: [0; 64].as_mut_ptr(),
                Description: [0; 64].as_mut_ptr(),
                SerialNumber: [0; 64].as_mut_ptr(),
                MaxPower: 0,
                PnP: 0,
                SelfPowered: 0,
                RemoteWakeup: 0,
                // Rev4
                Rev4: 0,
                IsoIn: 0,
                IsoOut: 0,
                PullDownEnable: 0,
                SerNumEnable: 0,
                USBVersionEnable: 0,
                USBVersion: 0,
                // Rev5
                Rev5: 0,
                IsoInA: 0,
                IsoInB: 0,
                IsoOutA: 0,
                IsoOutB: 0,
                PullDownEnable5: 0,
                SerNumEnable5: 0,
                USBVersionEnable5: 0,
                USBVersion5: 0,
                AIsHighCurrent: 0,
                BIsHighCurrent: 0,
                IFAIsFifo: 0,
                IFAIsFifoTar: 0,
                IFAIsFastSer: 0,
                AIsVCP: 0,
                IFBIsFifo: 0,
                IFBIsFifoTar: 0,
                IFBIsFastSer: 0,
                BIsVCP: 0,
                // Rev 6
                UseExtOsc: 0,
                HighDriveIOs: 0,
                EndpointSize: 0,
                PullDownEnableR: 0,
                SerNumEnableR: 0,
                InvertTXD: 0,
                InvertRXD: 0,
                InvertRTS: 0,
                InvertCTS: 0,
                InvertDTR: 0,
                InvertDSR: 0,
                InvertDCD: 0,
                InvertRI: 0,
                Cbus0: 0,
                Cbus1: 0,
                Cbus2: 0,
                Cbus3: 0,
                Cbus4: 0,
                RIsD2XX: 0,
                // Rev 7
                PullDownEnable7: 0,
                SerNumEnable7: 0,
                ALSlowSlew: 0,
                ALSchmittInput: 0,
                ALDriveCurrent: 0,
                AHSlowSlew: 0,
                AHSchmittInput: 0,
                AHDriveCurrent: 0,
                BLSlowSlew: 0,
                BLSchmittInput: 0,
                BLDriveCurrent: 0,
                BHSlowSlew: 0,
                BHSchmittInput: 0,
                BHDriveCurrent: 0,
                IFAIsFifo7: 0,
                IFAIsFifoTar7: 0,
                IFAIsFastSer7: 0,
                AIsVCP7: 0,
                IFBIsFifo7: 0,
                IFBIsFifoTar7: 0,
                IFBIsFastSer7: 0,
                BIsVCP7: 0,
                PowerSaveEnable: 0,
                // Rev 8
                PullDownEnable8: 0,
                SerNumEnable8: 0,
                ASlowSlew: 0,
                ASchmittInput: 0,
                ADriveCurrent: 0,
                BSlowSlew: 0,
                BSchmittInput: 0,
                BDriveCurrent: 0,
                CSlowSlew: 0,
                CSchmittInput: 0,
                CDriveCurrent: 0,
                DSlowSlew: 0,
                DSchmittInput: 0,
                DDriveCurrent: 0,
                ARIIsTXDEN: 0,
                BRIIsTXDEN: 0,
                CRIIsTXDEN: 0,
                DRIIsTXDEN: 0,
                AIsVCP8: 0,
                BIsVCP8: 0,
                CIsVCP8: 0,
                DIsVCP8: 0,
                // Rev 9
                PullDownEnableH: 0,
                SerNumEnableH: 0,
                ACSlowSlewH: 0,
                ACSchmittInputH: 0,
                ACDriveCurrentH: 0,
                ADSlowSlewH: 0,
                ADSchmittInputH: 0,
                ADDriveCurrentH: 0,
                Cbus0H: 0,
                Cbus1H: 0,
                Cbus2H: 0,
                Cbus3H: 0,
                Cbus4H: 0,
                Cbus5H: 0,
                Cbus6H: 0,
                Cbus7H: 0,
                Cbus8H: 0,
                Cbus9H: 0,
                IsFifoH: 0,
                IsFifoTarH: 0,
                IsFastSerH: 0,
                IsFT1248H: 0,
                FT1248CpolH: 0,
                FT1248LsbH: 0,
                FT1248FlowControlH: 0,
                IsVCPH: 0,
                PowerSaveEnableH: 0,
            }
        }
    }
}

#[repr(u8)]
pub enum BitMode {
    Reset = 0x0,
    AsyncBitBang = 0x1,
    MPSSE = 0x2,
    SyncBitBang = 0x4,
    MCUHostBusEmulation = 0x8,
    FastOptoIsolatedSerial = 0x10,
    CBUSBitBang = 0x20,
    SingleChannelSync245FIFOMode = 0x40,
}