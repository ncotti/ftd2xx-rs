//! EEPROM structures for the following device:
//! * FT232B

use crate::eeprom::{DevType, Eeprom, EepromHeader, EepromStrings};
use ftd2xx_sys::d2xx::FT_EEPROM_232B;

/// EEPROM configuration for an FT232B device.
#[derive(Debug, Clone)]
pub struct EepromFt232b {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
}

impl Eeprom for EepromFt232b {
    type FtEeprom = FT_EEPROM_232B;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt232b {
    fn default() -> Self {
        EepromFt232b {
            common: EepromHeader::new(DevType::DevBM),
            strings: EepromStrings::default(),
        }
    }
}

impl From<&EepromFt232b> for FT_EEPROM_232B {
    fn from(t: &EepromFt232b) -> Self {
        FT_EEPROM_232B {
            common: (&t.common).into(),
        }
    }
}

impl From<EepromFt232b> for FT_EEPROM_232B {
    fn from(t: EepromFt232b) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_232B> for EepromFt232b {
    fn from(t: FT_EEPROM_232B) -> Self {
        EepromFt232b {
            common: t.common.into(),
            strings: EepromStrings::default(),
        }
    }
}
