//! EEPROM structures for the following devices:
//! * FT232R

use crate::eeprom::{DevType, Eeprom, EepromHeader, EepromStrings};
use ftd2xx_sys::FT_EEPROM_232R;

/// FT4232H EEPROM configuration.
#[derive(Debug, Clone)]
pub struct EepromFt232r {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    /// Allow high current
    pub is_high_current: bool,
    /// Use external oscillator
    pub use_external_oscillator: bool,
    /// Invert TXD
    pub invert_txd: bool,
    /// Invert RXD
    pub invert_rxd: bool,
    /// Invert RTS
    pub invert_rts: bool,
    /// Invert CTS
    pub invert_cts: bool,
    /// Invert DTR
    pub invert_dtr: bool,
    /// Invert DSR
    pub invert_dsr: bool,
    /// Invert DCD
    pub invert_dcd: bool,
    /// Invert RI
    pub invert_ri: bool,
    /// CBUS Pin Multiplexation
    pub cbus: [Ft232rCbus; 5],
}

impl Eeprom for EepromFt232r {
    type FtEeprom = FT_EEPROM_232R;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt232r {
    fn default() -> Self {
        EepromFt232r {
            common: EepromHeader::new(DevType::Dev232R),
            strings: EepromStrings::default(),
            is_high_current: false,
            use_external_oscillator: false,
            invert_txd: false,
            invert_rxd: false,
            invert_rts: false,
            invert_cts: false,
            invert_dtr: false,
            invert_dsr: false,
            invert_dcd: false,
            invert_ri: false,
            cbus: [
                Ft232rCbus::Txled,
                Ft232rCbus::Rxled,
                Ft232rCbus::Txden,
                Ft232rCbus::Pwron,
                Ft232rCbus::Sleep,
            ],
        }
    }
}

/// FT232R CBUS Pin Mux
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Ft232rCbus {
    Txden = 0x00,
    Pwron = 0x01,
    Rxled = 0x02,
    Txled = 0x03,
    TxRxLed = 0x04,
    Sleep = 0x05,
    Clk48 = 0x06,
    Clk24 = 0x07,
    Clk12 = 0x08,
    Clk6 = 0x09,
    IoMode = 0x0A,
    BitBangWrite = 0x0B,
    BitBangRead = 0x0C,
    RxfTxeRdWr = 0x0D,
}

impl From<u8> for Ft232rCbus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Ft232rCbus::Txden,
            0x01 => Ft232rCbus::Pwron,
            0x02 => Ft232rCbus::Rxled,
            0x03 => Ft232rCbus::Txled,
            0x04 => Ft232rCbus::TxRxLed,
            0x05 => Ft232rCbus::Sleep,
            0x06 => Ft232rCbus::Clk48,
            0x07 => Ft232rCbus::Clk24,
            0x08 => Ft232rCbus::Clk12,
            0x09 => Ft232rCbus::Clk6,
            0x0A => Ft232rCbus::IoMode,
            0x0B => Ft232rCbus::BitBangWrite,
            0x0C => Ft232rCbus::BitBangRead,
            0x0D => Ft232rCbus::RxfTxeRdWr,
            _ => Ft232rCbus::RxfTxeRdWr,
        }
    }
}

impl From<&EepromFt232r> for FT_EEPROM_232R {
    fn from(t: &EepromFt232r) -> Self {
        FT_EEPROM_232R {
            common: (&t.common).into(),
            IsHighCurrent: t.is_high_current as u8,
            UseExtOsc: t.use_external_oscillator as u8,
            InvertTXD: t.invert_txd as u8,
            InvertRXD: t.invert_rxd as u8,
            InvertRTS: t.invert_rts as u8,
            InvertCTS: t.invert_cts as u8,
            InvertDTR: t.invert_dtr as u8,
            InvertDSR: t.invert_dsr as u8,
            InvertDCD: t.invert_dcd as u8,
            InvertRI: t.invert_ri as u8,
            Cbus0: t.cbus[0] as u8,
            Cbus1: t.cbus[1] as u8,
            Cbus2: t.cbus[2] as u8,
            Cbus3: t.cbus[3] as u8,
            Cbus4: t.cbus[4] as u8,
            DriverType: true as u8, // D2XX driver
        }
    }
}

impl From<EepromFt232r> for FT_EEPROM_232R {
    fn from(t: EepromFt232r) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_232R> for EepromFt232r {
    fn from(t: FT_EEPROM_232R) -> Self {
        EepromFt232r {
            common: t.common.into(),
            strings: EepromStrings::default(),
            is_high_current: t.IsHighCurrent != 0,
            use_external_oscillator: t.UseExtOsc != 0,
            invert_txd: t.InvertTXD != 0,
            invert_rxd: t.InvertRXD != 0,
            invert_rts: t.InvertRTS != 0,
            invert_cts: t.InvertCTS != 0,
            invert_dtr: t.InvertDTR != 0,
            invert_dsr: t.InvertDSR != 0,
            invert_dcd: t.InvertDCD != 0,
            invert_ri: t.InvertRI != 0,
            cbus: [
                t.Cbus0.into(),
                t.Cbus1.into(),
                t.Cbus2.into(),
                t.Cbus3.into(),
                t.Cbus4.into(),
            ],
        }
    }
}
