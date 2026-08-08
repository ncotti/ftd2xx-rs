//! EEPROM structures for the following devices:
//! * FT4232H
//! * FT4232HP
//! * FT4233HP

use crate::eeprom::{DevType, DriveCurrent, Eeprom, EepromHeader, EepromPD, EepromStrings};
use ftd2xx_sys::{FT_EEPROM_4232H, FT_EEPROM_4232HP, FT_EEPROM_4233HP};

/// FT4232H EEPROM configuration.
#[derive(Debug, Clone)]
pub struct EepromFt4232h {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    /// Channel A configuration.
    pub cha: EepromFt4232hChannel,
    /// Channel B configuration.
    pub chb: EepromFt4232hChannel,
    /// Channel C configuration.
    pub chc: EepromFt4232hChannel,
    /// Channel D configuration.
    pub chd: EepromFt4232hChannel,
}

impl Eeprom for EepromFt4232h {
    type FtEeprom = FT_EEPROM_4232H;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt4232h {
    fn default() -> Self {
        EepromFt4232h {
            common: EepromHeader::new(DevType::Dev4232H),
            cha: EepromFt4232hChannel::default(),
            chb: EepromFt4232hChannel::default(),
            chc: EepromFt4232hChannel::default(),
            chd: EepromFt4232hChannel::default(),
            strings: EepromStrings::default(),
        }
    }
}

/// FT4232H EEPROM channel configuration
#[derive(Debug, Clone, Default)]
pub struct EepromFt4232hChannel {
    /// Slow slew rate.
    pub slow_slew: bool,
    /// The inputs will have an hysteresis for changing electrical state
    pub schmitt_input: bool,
    /// Maximum current that each I/O pin may source/sink.
    pub drive_current: DriveCurrent,
    /// Enables TXDEN signal for RS485 buses.
    pub use_ri_as_txden: bool,
}

impl From<&EepromFt4232h> for FT_EEPROM_4232H {
    fn from(t: &EepromFt4232h) -> Self {
        FT_EEPROM_4232H {
            common: (&t.common).into(),
            ASlowSlew: t.cha.slow_slew as u8,
            ASchmittInput: t.cha.schmitt_input as u8,
            ADriveCurrent: t.cha.drive_current as u8,
            BSlowSlew: t.chb.slow_slew as u8,
            BSchmittInput: t.chb.schmitt_input as u8,
            BDriveCurrent: t.chb.drive_current as u8,
            CSlowSlew: t.chc.slow_slew as u8,
            CSchmittInput: t.chc.schmitt_input as u8,
            CDriveCurrent: t.chc.drive_current as u8,
            DSlowSlew: t.chd.slow_slew as u8,
            DSchmittInput: t.chd.schmitt_input as u8,
            DDriveCurrent: t.chd.drive_current as u8,
            ARIIsTXDEN: t.cha.use_ri_as_txden as u8,
            BRIIsTXDEN: t.chb.use_ri_as_txden as u8,
            CRIIsTXDEN: t.chc.use_ri_as_txden as u8,
            DRIIsTXDEN: t.chd.use_ri_as_txden as u8,
            ADriverType: false as u8,
            BDriverType: false as u8,
            CDriverType: false as u8,
            DDriverType: false as u8,
        }
    }
}

impl From<EepromFt4232h> for FT_EEPROM_4232H {
    fn from(t: EepromFt4232h) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_4232H> for EepromFt4232h {
    fn from(t: FT_EEPROM_4232H) -> Self {
        EepromFt4232h {
            common: t.common.into(),
            cha: EepromFt4232hChannel {
                slow_slew: t.ASlowSlew != 0,
                schmitt_input: t.ASchmittInput != 0,
                drive_current: DriveCurrent::from(t.ADriveCurrent),
                use_ri_as_txden: t.ARIIsTXDEN != 0,
            },
            chb: EepromFt4232hChannel {
                slow_slew: t.BSlowSlew != 0,
                schmitt_input: t.BSchmittInput != 0,
                drive_current: DriveCurrent::from(t.BDriveCurrent),
                use_ri_as_txden: t.BRIIsTXDEN != 0,
            },
            chc: EepromFt4232hChannel {
                slow_slew: t.CSlowSlew != 0,
                schmitt_input: t.CSchmittInput != 0,
                drive_current: DriveCurrent::from(t.CDriveCurrent),
                use_ri_as_txden: t.CRIIsTXDEN != 0,
            },
            chd: EepromFt4232hChannel {
                slow_slew: t.DSlowSlew != 0,
                schmitt_input: t.DSchmittInput != 0,
                drive_current: DriveCurrent::from(t.DDriveCurrent),
                use_ri_as_txden: t.DRIIsTXDEN != 0,
            },
            strings: EepromStrings::default(),
        }
    }
}

/// FT4232HP EEPROM configuration
#[derive(Debug, Clone)]
pub struct EepromFt4232hp {
    /// Base FT4232H configuration
    pub ft4232h: EepromFt4232h,
    /// Power delivery aggregate configuration
    pub pd: EepromPD,
}

impl Eeprom for EepromFt4232hp {
    type FtEeprom = FT_EEPROM_4232HP;

    fn strings(&self) -> &EepromStrings {
        &self.ft4232h.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.ft4232h.strings
    }
}

impl Default for EepromFt4232hp {
    fn default() -> Self {
        EepromFt4232hp {
            ft4232h: EepromFt4232h {
                common: EepromHeader::new(DevType::Dev4232HP),
                cha: EepromFt4232hChannel::default(),
                chb: EepromFt4232hChannel::default(),
                chc: EepromFt4232hChannel::default(),
                chd: EepromFt4232hChannel::default(),
                strings: EepromStrings::default(),
            },
            pd: EepromPD::default(),
        }
    }
}

impl From<&EepromFt4232hp> for FT_EEPROM_4232HP {
    fn from(t: &EepromFt4232hp) -> Self {
        FT_EEPROM_4232HP {
            ft4232h: (&t.ft4232h).into(),
            pd: (&t.pd).into(),
        }
    }
}

impl From<EepromFt4232hp> for FT_EEPROM_4232HP {
    fn from(t: EepromFt4232hp) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_4232HP> for EepromFt4232hp {
    fn from(t: FT_EEPROM_4232HP) -> Self {
        EepromFt4232hp {
            ft4232h: t.ft4232h.into(),
            pd: EepromPD::default(),
        }
    }
}

/// FT4233HP EEPROM configuration
#[derive(Debug, Clone)]
pub struct EepromFt4233hp {
    /// Base FT4232H configuration
    pub ft4232h: EepromFt4232h,
    /// Power delivery aggregate configuration
    pub pd: EepromPD,
}

impl Eeprom for EepromFt4233hp {
    type FtEeprom = FT_EEPROM_4233HP;

    fn strings(&self) -> &EepromStrings {
        &self.ft4232h.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.ft4232h.strings
    }
}

impl Default for EepromFt4233hp {
    fn default() -> Self {
        EepromFt4233hp {
            ft4232h: EepromFt4232h {
                common: EepromHeader::new(DevType::Dev4232HP),
                cha: EepromFt4232hChannel::default(),
                chb: EepromFt4232hChannel::default(),
                chc: EepromFt4232hChannel::default(),
                chd: EepromFt4232hChannel::default(),
                strings: EepromStrings::default(),
            },
            pd: EepromPD::default(),
        }
    }
}

impl From<&EepromFt4233hp> for FT_EEPROM_4233HP {
    fn from(t: &EepromFt4233hp) -> Self {
        FT_EEPROM_4233HP {
            ft4232h: (&t.ft4232h).into(),
            pd: (&t.pd).into(),
        }
    }
}

impl From<EepromFt4233hp> for FT_EEPROM_4233HP {
    fn from(t: EepromFt4233hp) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_4233HP> for EepromFt4233hp {
    fn from(t: FT_EEPROM_4233HP) -> Self {
        EepromFt4233hp {
            ft4232h: t.ft4232h.into(),
            pd: EepromPD::default(),
        }
    }
}
