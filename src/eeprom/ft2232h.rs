//! EEPROM structures for the following devices:
//! * FT2232H
//! * FT2232HP
//! * FT2233HP

use crate::eeprom::{DevType, DriveCurrent, Eeprom, EepromHeader, EepromPD, EepromStrings};

use ftd2xx_sys::d2xx::{FT_EEPROM_2232H, FT_EEPROM_2232HP, FT_EEPROM_2233HP};

/// FT2232H EEPROM configuration.
#[derive(Debug, Clone)]
pub struct EepromFt2232h {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    /// Channel A configuration.
    pub cha: EepromFt2232hChannel,
    /// Channel B configuration.
    pub chb: EepromFt2232hChannel,
    /// Enable power save mode.
    pub power_save_enable: bool,
}

impl Eeprom for EepromFt2232h {
    type FtEeprom = FT_EEPROM_2232H;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt2232h {
    fn default() -> Self {
        EepromFt2232h {
            common: EepromHeader::new(DevType::Dev2232H),
            strings: EepromStrings::default(),
            cha: EepromFt2232hChannel::default(),
            chb: EepromFt2232hChannel::default(),
            power_save_enable: false,
        }
    }
}

/// FT2232H EEPROM configuration for each of the device's channels (A and B)
#[derive(Debug, Clone, Default)]
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

impl From<&EepromFt2232h> for FT_EEPROM_2232H {
    fn from(t: &EepromFt2232h) -> Self {
        FT_EEPROM_2232H {
            common: (&t.common).into(),
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

impl From<EepromFt2232h> for FT_EEPROM_2232H {
    fn from(t: EepromFt2232h) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_2232H> for EepromFt2232h {
    fn from(t: FT_EEPROM_2232H) -> Self {
        EepromFt2232h {
            common: t.common.into(),
            strings: EepromStrings::default(),
            cha: EepromFt2232hChannel {
                low_slow_slew: t.ALSlowSlew != 0,
                low_schmitt_input: t.ALSchmittInput != 0,
                low_drive_current: t.ALDriveCurrent.into(),
                high_slow_slew: t.AHSlowSlew != 0,
                high_schmitt_input: t.AHSchmittInput != 0,
                high_drive_current: t.AHDriveCurrent.into(),
                is_fifo: t.AIsFifo != 0,
                is_fifo_target: t.AIsFifoTar != 0,
                is_fast_serial: t.AIsFastSer != 0,
            },
            chb: EepromFt2232hChannel {
                low_slow_slew: t.BLSlowSlew != 0,
                low_schmitt_input: t.BLSchmittInput != 0,
                low_drive_current: t.BLDriveCurrent.into(),
                high_slow_slew: t.BHSlowSlew != 0,
                high_schmitt_input: t.BHSchmittInput != 0,
                high_drive_current: t.BHDriveCurrent.into(),
                is_fifo: t.BIsFifo != 0,
                is_fifo_target: t.BIsFifoTar != 0,
                is_fast_serial: t.BIsFastSer != 0,
            },
            power_save_enable: t.PowerSaveEnable != 0,
        }
    }
}

/// FT2232HP EEPROM configuration
#[derive(Debug, Clone)]
pub struct EepromFt2232hp {
    /// Base FT2232H configuration
    pub ft2232h: EepromFt2232h,
    /// Power delivery aggregate configuration
    pub pd: EepromPD,
}

impl Eeprom for EepromFt2232hp {
    type FtEeprom = FT_EEPROM_2232HP;

    fn strings(&self) -> &EepromStrings {
        &self.ft2232h.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.ft2232h.strings
    }
}

impl Default for EepromFt2232hp {
    fn default() -> Self {
        EepromFt2232hp {
            ft2232h: EepromFt2232h {
                common: EepromHeader::new(DevType::Dev2232HP),
                strings: EepromStrings::default(),
                cha: EepromFt2232hChannel::default(),
                chb: EepromFt2232hChannel::default(),
                power_save_enable: false,
            },
            pd: EepromPD::default(),
        }
    }
}

impl From<&EepromFt2232hp> for FT_EEPROM_2232HP {
    fn from(t: &EepromFt2232hp) -> Self {
        FT_EEPROM_2232HP {
            ft2232h: (&t.ft2232h).into(),
            pd: (&t.pd).into(),
        }
    }
}

impl From<EepromFt2232hp> for FT_EEPROM_2232HP {
    fn from(t: EepromFt2232hp) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_2232HP> for EepromFt2232hp {
    fn from(t: FT_EEPROM_2232HP) -> Self {
        EepromFt2232hp {
            ft2232h: t.ft2232h.into(),
            pd: EepromPD::default(),
        }
    }
}

/// FT2233HP EEPROM configuration
#[derive(Debug, Clone)]
pub struct EepromFt2233hp {
    /// Base FT2232H configuration
    pub ft2232h: EepromFt2232h,
    /// Power delivery aggregate configuration
    pub pd: EepromPD,
}

impl Eeprom for EepromFt2233hp {
    type FtEeprom = FT_EEPROM_2233HP;

    fn strings(&self) -> &EepromStrings {
        &self.ft2232h.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.ft2232h.strings
    }
}

impl Default for EepromFt2233hp {
    fn default() -> Self {
        EepromFt2233hp {
            ft2232h: EepromFt2232h {
                common: EepromHeader::new(DevType::Dev2232HP),
                strings: EepromStrings::default(),
                cha: EepromFt2232hChannel::default(),
                chb: EepromFt2232hChannel::default(),
                power_save_enable: false,
            },
            pd: EepromPD::default(),
        }
    }
}

impl From<&EepromFt2233hp> for FT_EEPROM_2233HP {
    fn from(t: &EepromFt2233hp) -> Self {
        FT_EEPROM_2233HP {
            ft2232h: (&t.ft2232h).into(),
            pd: (&t.pd).into(),
        }
    }
}

impl From<EepromFt2233hp> for FT_EEPROM_2233HP {
    fn from(t: EepromFt2233hp) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_2233HP> for EepromFt2233hp {
    fn from(t: FT_EEPROM_2233HP) -> Self {
        EepromFt2233hp {
            ft2232h: t.ft2232h.into(),
            pd: EepromPD::default(),
        }
    }
}
