//! Doc

use crate::eeprom::EepromHeader;
use ftd2xx_sys::FT_EEPROM_232B;

/// EEPROM configuration for an FT232B device.
pub struct EepromFt232b {
    /// EEPROM common configuration
    common: EepromHeader,
}

impl From<EepromFt232b> for FT_EEPROM_232B {
    fn from(t: EepromFt232b) -> Self {
        FT_EEPROM_232B {
            common: t.common.into(),
        }
    }
}
