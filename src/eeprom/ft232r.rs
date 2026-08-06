//! Crate doc
//! 

use ftd2xx_sys::FT_EEPROM_232R;
use crate::eeprom::EepromHeader;

/// EEPROM configuration for an FT232R device.
#[allow(missing_docs)]
pub struct EepromFt232r {
    pub common: EepromHeader,
    pub is_high_current: bool,
    pub use_external_oscillator: bool,
    pub invert_txd: bool,
    pub invert_rxd: bool,
    pub invert_rts: bool,
    pub invert_cts: bool,
    pub invert_dtr: bool,
    pub invert_dsr: bool,
    pub invert_dcd: bool,
    pub invert_ri: bool,
    pub cbus: [u8; 5],
}

impl From<EepromFt232r> for FT_EEPROM_232R {
    fn from(t: EepromFt232r) -> Self {
        FT_EEPROM_232R {
            common: t.common.into(),
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
            Cbus0: t.cbus[0],
            Cbus1: t.cbus[1],
            Cbus2: t.cbus[2],
            Cbus3: t.cbus[3],
            Cbus4: t.cbus[4],
            DriverType: true as u8, // D2XX driver
        }
    }
}