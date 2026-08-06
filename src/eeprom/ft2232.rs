//! Doc

use ftd2xx_sys::FT_EEPROM_2232;

use crate::eeprom::EepromHeader;


/// EEPROM configuration for an FT2232 device.
pub struct EepromFt2232 {
    /// EEPROM common configuration
    pub common: EepromHeader,
    /// Channel A configuration
    pub cha: EepromFt2232Channel,
    /// Channel B configuration
    pub chb: EepromFt2232Channel,
}

/// FT2232 EEPROM configuration for each of the device's channels (A and B)
#[allow(missing_docs)]
pub struct EepromFt2232Channel {
    pub is_high_current: bool,
    pub is_fifo: bool,
    pub is_fifo_target: bool,
    pub is_fast_serial: bool,
}

impl From<EepromFt2232> for FT_EEPROM_2232 {
    fn from(t: EepromFt2232) -> Self {
        FT_EEPROM_2232 {
            common: t.common.into(),

            // Channel A
            AIsHighCurrent: t.cha.is_high_current as u8,
            AIsFifo: t.cha.is_fifo as u8,
            AIsFifoTar: t.cha.is_fifo_target as u8,
            AIsFastSer: t.cha.is_fast_serial as u8,
            ADriverType: false as u8, // D2XX driver

            // Channel B
            BIsHighCurrent: t.chb.is_high_current as u8,
            BIsFifo: t.chb.is_fifo as u8,
            BIsFifoTar: t.chb.is_fifo_target as u8,
            BIsFastSer: t.chb.is_fast_serial as u8,
            BDriverType: false as u8, // D2XX driver
        }
    }
}