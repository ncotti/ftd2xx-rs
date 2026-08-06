//! Doc
//!

use crate::eeprom::{DriveCurrent, EepromHeader, EepromPD};

use ftd2xx_sys::{FT_EEPROM_2232H, FT_EEPROM_2232HP, FT_EEPROM_2233HP};

/// EEPROM configuration for an FT2232H device.
#[allow(missing_docs)]
pub struct EepromFt2232h {
    pub common: EepromHeader,
    pub cha: EepromFt2232hChannel,
    pub chb: EepromFt2232hChannel,
    pub power_save_enable: bool,
}

/// FT2232H EEPROM configuration for each of the device's channels (A and B)
#[allow(missing_docs)]
pub struct EepromFt2232hChannel {
    pub low_slow_slew: bool,
    pub low_schmitt_input: bool,
    pub low_drive_current: DriveCurrent,
    pub high_slow_slew: bool,
    pub high_schmitt_input: bool,
    pub high_drive_current: DriveCurrent,
    pub is_fifo: bool,
    pub is_fifo_target: bool,
    pub is_fast_serial: bool,
}

impl From<EepromFt2232h> for FT_EEPROM_2232H {
    fn from(t: EepromFt2232h) -> Self {
        FT_EEPROM_2232H {
            common: t.common.into(),
            ALSlowSlew: t.cha.low_slow_slew as u8,
            ALSchmittInput: t.cha.low_schmitt_input as u8,
            ALDriveCurrent: t.cha.low_drive_current as u8,
            AHSlowSlew: t.cha.high_slow_slew as u8,
            AHSchmittInput: t.cha.high_schmitt_input as u8,
            AHDriveCurrent: t.cha.high_drive_current as u8,
            BLSlowSlew: t.chb.low_slow_slew as u8,
            BLSchmittInput: t.chb.low_schmitt_input as u8,
            BLDriveCurrent: t.chb.low_drive_current as u8,
            BHSlowSlew: t.chb.high_slow_slew as u8,
            BHSchmittInput: t.chb.high_schmitt_input as u8,
            BHDriveCurrent: t.chb.high_drive_current as u8,
            AIsFifo: t.cha.is_fifo as u8,
            AIsFifoTar: t.cha.is_fifo_target as u8,
            AIsFastSer: t.cha.is_fast_serial as u8,
            BIsFifo: t.chb.is_fifo as u8,
            BIsFifoTar: t.chb.is_fifo_target as u8,
            BIsFastSer: t.chb.is_fast_serial as u8,
            PowerSaveEnable: t.power_save_enable as u8,
            ADriverType: false as u8, // D2XX driver
            BDriverType: false as u8, // D2XX driver
        }
    }
}

/// EEPROM configuration for a FT2232HP device.
#[allow(missing_docs)]
pub struct EepromFt2232hp {
    ft2232h: EepromFt2232h,
    pd: EepromPD,
}

impl From<EepromFt2232hp> for FT_EEPROM_2232HP {
    fn from(t: EepromFt2232hp) -> Self {
        FT_EEPROM_2232HP {
            ft2232h: t.ft2232h.into(),
            pd: t.pd.into(),
        }
    }
}

/// EEPROM configuration for a FT2233HP device.
#[allow(missing_docs)]
pub struct EepromFt2233hp {
    ft2232h: EepromFt2232h,
    pd: EepromPD,
}

impl From<EepromFt2233hp> for FT_EEPROM_2233HP {
    fn from(t: EepromFt2233hp) -> Self {
        FT_EEPROM_2233HP {
            ft2232h: t.ft2232h.into(),
            pd: t.pd.into(),
        }
    }
}
