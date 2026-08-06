//! Doc
//!

use crate::eeprom::{DriveCurrent, EepromHeader, EepromPD};
use ftd2xx_sys::{FT_EEPROM_232H, FT_EEPROM_232HP, FT_EEPROM_233HP};

/// EEPROM configuration for an FT232H device.
#[allow(missing_docs)]
pub struct EepromFt232h {
    common: EepromHeader,
    ac_slow_slew: bool,
    ac_schmitt_input: bool,
    ac_drive_current: DriveCurrent,
    ad_slow_slew: bool,
    ad_schmitt_input: bool,
    ad_drive_current: DriveCurrent,
    cbus: [u8; 10],
    ft1248_cpol_high: bool,
    ft1248_lsb: bool,
    ft1248_flow_control: bool,
    is_fifo: bool,
    is_fifo_target: bool,
    is_fast_serial: bool,
    is_ft1248: bool,
    power_save_enable: bool,
}

impl From<EepromFt232h> for FT_EEPROM_232H {
    fn from(t: EepromFt232h) -> Self {
        FT_EEPROM_232H {
            common: t.common.into(),
            ACSlowSlew: t.ac_slow_slew as u8,
            ACSchmittInput: t.ac_schmitt_input as u8,
            ACDriveCurrent: t.ac_drive_current as u8,
            ADSlowSlew: t.ad_slow_slew as u8,
            ADSchmittInput: t.ad_schmitt_input as u8,
            ADDriveCurrent: t.ad_drive_current as u8,
            Cbus0: t.cbus[0],
            Cbus1: t.cbus[1],
            Cbus2: t.cbus[2],
            Cbus3: t.cbus[3],
            Cbus4: t.cbus[4],
            Cbus5: t.cbus[5],
            Cbus6: t.cbus[6],
            Cbus7: t.cbus[7],
            Cbus8: t.cbus[8],
            Cbus9: t.cbus[9],
            FT1248Cpol: t.ft1248_cpol_high as u8,
            FT1248Lsb: t.ft1248_lsb as u8,
            FT1248FlowControl: t.ft1248_flow_control as u8,
            IsFifo: t.is_fifo as u8,
            IsFifoTar: t.is_fifo_target as u8,
            IsFastSer: t.is_fast_serial as u8,
            IsFT1248: t.is_ft1248 as u8,
            PowerSaveEnable: t.power_save_enable as u8,
            DriverType: false as u8,
        }
    }
}

/// EEPROM configuration for a FT232HP device.
#[allow(missing_docs)]
pub struct EepromFt232hp {
    ft232h: EepromFt232h,
    pd: EepromPD,
}

impl From<EepromFt232hp> for FT_EEPROM_232HP {
    fn from(t: EepromFt232hp) -> Self {
        FT_EEPROM_232HP {
            ft232h: t.ft232h.into(),
            pd: t.pd.into(),
        }
    }
}

/// EEPROM configuration for a FT233HP device.
#[allow(missing_docs)]
pub struct EepromFt233hp {
    ft232h: EepromFt232h,
    pd: EepromPD,
}

impl From<EepromFt233hp> for FT_EEPROM_233HP {
    fn from(t: EepromFt233hp) -> Self {
        FT_EEPROM_233HP {
            ft232h: t.ft232h.into(),
            pd: t.pd.into(),
        }
    }
}
