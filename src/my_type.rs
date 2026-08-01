//! Type doc

use ftd2xx_sys::*;
use std::{fmt, ptr::null_mut};

/// Holds the current library version, as v.<major>.<minor>.<build>
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
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
    pub program_data: FT_PROGRAM_DATA,
    pub manufacturer: Box<[u8; 64]>,
    pub manufacturer_id: Box<[u8; 64]>,
    pub description: Box<[u8; 64]>,
    pub serial_number: Box<[u8; 64]>,
}

impl MyProgramData {
    pub fn set_manufacturer(&mut self, input: &str) {
        self.manufacturer = Box::new([0; 64]);

        let bytes = input.as_bytes();
        let len = bytes.len().min(self.manufacturer.len());

        self.manufacturer[..len].copy_from_slice(&bytes[..len]);

        self.program_data.Manufacturer = self.manufacturer.as_mut_ptr() as *mut i8;
    }

    pub fn set_manufacturer_id(&mut self, input: &str) {
        self.manufacturer_id = Box::new([0; 64]);

        let bytes = input.as_bytes();
        let len = bytes.len().min(self.manufacturer.len());

        self.manufacturer_id[..len].copy_from_slice(&bytes[..len]);

        self.program_data.ManufacturerId = self.manufacturer_id.as_mut_ptr() as *mut i8;
    }

    pub fn set_description(&mut self, input: &str) {
        self.description = Box::new([0; 64]);

        let bytes = input.as_bytes();
        let len = bytes.len().min(self.description.len());

        self.description[..len].copy_from_slice(&bytes[..len]);

        self.program_data.Description = self.description.as_mut_ptr() as *mut i8;
    }

    pub fn set_serial_number(&mut self, input: &str) {
        self.serial_number = Box::new([0; 64]);

        let bytes = input.as_bytes();
        let len = bytes.len().min(self.serial_number.len());

        self.serial_number[..len].copy_from_slice(&bytes[..len]);

        self.program_data.SerialNumber = self.serial_number.as_mut_ptr() as *mut i8;
    }

    pub fn new_default() -> Self {
        let mut data = MyProgramData {
            manufacturer: Box::new([0; 64]),
            manufacturer_id: Box::new([0; 64]),
            description: Box::new([0; 64]),
            serial_number: Box::new([0; 64]),

            program_data: FT_PROGRAM_DATA {
                // Common
                Signature1: 0x00000000,
                Signature2: 0xFFFFFFFF,
                Version: 4,
                VendorId: 0,
                ProductId: 0,
                Manufacturer: null_mut(),
                ManufacturerId: null_mut(),
                Description: null_mut(),
                SerialNumber: null_mut(),
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
            },
        };

        data.program_data.Manufacturer = data.manufacturer.as_mut_ptr() as *mut i8;
        data.program_data.ManufacturerId = data.manufacturer_id.as_mut_ptr() as *mut i8;
        data.program_data.Description = data.description.as_mut_ptr() as *mut i8;
        data.program_data.SerialNumber = data.serial_number.as_mut_ptr() as *mut i8;

        data
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
