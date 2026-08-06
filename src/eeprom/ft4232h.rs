//! EEPROM structures for the devices:
//! * FT4232H
//! * FT4232HP
//! * FT4233HP

use crate::eeprom::{DevType, DriveCurrent, Eeprom, EepromHeader, EepromPD, EepromStrings};
use ftd2xx_sys::{FT_EEPROM_4232H, FT_EEPROM_4232HP, FT_EEPROM_4233HP};

/// FT4232H EEPROM configuration.
#[derive(Debug)]
pub struct EepromFt4232h {
    pub common: EepromHeader,
    pub cha: EepromFt4232hChannel,
    pub chb: EepromFt4232hChannel,
    pub chc: EepromFt4232hChannel,
    pub chd: EepromFt4232hChannel,
    pub strings: EepromStrings,
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

/// FT4232H EEPROM configuration for each of the device's channels
#[allow(missing_docs)]
#[derive(Debug)]
pub struct EepromFt4232hChannel {
    slow_slew: bool,
    schmitt_input: bool,
    drive_current: DriveCurrent,
    use_ri_as_txden: bool,
}

impl Default for EepromFt4232hChannel {
    fn default() -> Self {
        EepromFt4232hChannel {
            slow_slew: false,
            schmitt_input: false,
            drive_current: DriveCurrent::Current4mA,
            use_ri_as_txden: false,
        }
    }
}

impl From<EepromFt4232h> for FT_EEPROM_4232H {
    fn from(t: EepromFt4232h) -> Self {
        FT_EEPROM_4232H {
            common: t.common.into(),
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

/// EEPROM configuration for a FT4232HP device.
#[allow(missing_docs)]
pub struct EepromFt4232hp {
    ft4232h: EepromFt4232h,
    pd: EepromPD,
}

impl From<EepromFt4232hp> for FT_EEPROM_4232HP {
    fn from(t: EepromFt4232hp) -> Self {
        FT_EEPROM_4232HP {
            ft4232h: t.ft4232h.into(),
            pd: t.pd.into(),
        }
    }
}

/// EEPROM configuration for a FT4233HP device.
#[allow(missing_docs)]
pub struct EepromFt4233hp {
    ft4232h: EepromFt4232h,
    pd: EepromPD,
}

impl From<EepromFt4233hp> for FT_EEPROM_4233HP {
    fn from(t: EepromFt4233hp) -> Self {
        FT_EEPROM_4233HP {
            ft4232h: t.ft4232h.into(),
            pd: t.pd.into(),
        }
    }
}
