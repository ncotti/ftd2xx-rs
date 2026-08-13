//! EEPROM structures for the following devices:
//! * FT2232

use ftd2xx_sys::d2xx::FT_EEPROM_2232;

use crate::eeprom::{DevType, Eeprom, EepromHeader, EepromStrings};

/// EEPROM configuration for an FT2232 device.
#[derive(Debug, Clone)]
pub struct EepromFt2232 {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    /// Channel A configuration
    pub cha: EepromFt2232Channel,
    /// Channel B configuration
    pub chb: EepromFt2232Channel,
}

impl Eeprom for EepromFt2232 {
    type FtEeprom = FT_EEPROM_2232;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt2232 {
    fn default() -> Self {
        EepromFt2232 {
            common: EepromHeader::new(DevType::Dev2232C),
            strings: EepromStrings::default(),
            cha: EepromFt2232Channel::default(),
            chb: EepromFt2232Channel::default(),
        }
    }
}

/// FT2232 EEPROM channel configuration
#[derive(Debug, Clone, Default)]
pub struct EepromFt2232Channel {
    /// Is high current
    pub is_high_current: bool,
    /// FIFO 245
    pub is_fifo: bool,
    /// FIFO 245 CPU Target
    pub is_fifo_target: bool,
    /// Fast serial
    pub is_fast_serial: bool,
}

impl From<&EepromFt2232> for FT_EEPROM_2232 {
    fn from(t: &EepromFt2232) -> Self {
        FT_EEPROM_2232 {
            common: (&t.common).into(),

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

impl From<EepromFt2232> for FT_EEPROM_2232 {
    fn from(t: EepromFt2232) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_2232> for EepromFt2232 {
    fn from(t: FT_EEPROM_2232) -> Self {
        EepromFt2232 {
            common: t.common.into(),
            strings: EepromStrings::default(),
            cha: EepromFt2232Channel {
                is_high_current: t.AIsHighCurrent != 0,
                is_fifo: t.AIsFifo != 0,
                is_fifo_target: t.AIsFifoTar != 0,
                is_fast_serial: t.AIsFastSer != 0,
            },
            chb: EepromFt2232Channel {
                is_high_current: t.BIsHighCurrent != 0,
                is_fifo: t.BIsFifo != 0,
                is_fifo_target: t.BIsFifoTar != 0,
                is_fast_serial: t.BIsFastSer != 0,
            },
        }
    }
}
