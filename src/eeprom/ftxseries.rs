//! Doc

use ftd2xx_sys::{FT_EEPROM_X_SERIES};

use crate::eeprom::{DriveCurrent, EepromHeader};

/// EEPROM configuration for an FT_X_Series device.
#[allow(missing_docs)]
pub struct EepromFtXSeries {
    common: EepromHeader,
    ac_slow_slew: bool,
    ac_schmitt_input: bool,
    ac_drive_current: DriveCurrent,
    ad_slow_slew: bool,
    ad_schmitt_input: bool,
    ad_drive_current: DriveCurrent,
    cbus: [u8; 7],
    invert_txd: bool,
    invert_rxd: bool,
    invert_rts: bool,
    invert_cts: bool,
    invert_dtr: bool,
    invert_dsr: bool,
    invert_dcd: bool,
    invert_ri: bool,
    bdc_enable: bool,
    bcd_force_cbus_pwren: bool,
    bcd_disable_sleep: bool,
    i2c_slave_address: u16,
    i2c_device_id: u32,
    i2c_disable_schmitt: bool,
    ft1248_cpol: bool,
    ft1248_lsb: bool,
    ft1248_flow_control: bool,
    rs485_echo_suppress: bool,
    power_save_enable: bool,
}

impl From<EepromFtXSeries> for FT_EEPROM_X_SERIES {
    fn from(t: EepromFtXSeries) -> Self {
        FT_EEPROM_X_SERIES {
            common: t.common.into(),
            ACSlowSlew: t.ac_slow_slew as u8,
            ACSchmittInput: t.ac_schmitt_input as u8,
            ACDriveCurrent: t.ac_drive_current as u8,
            ADSlowSlew: t.ad_slow_slew as u8,
            ADSchmittInput: t.ad_schmitt_input as u8,
            ADDriveCurrent: t.ad_slow_slew as u8,
            Cbus0: t.cbus[0],
            Cbus1: t.cbus[1],
            Cbus2: t.cbus[2],
            Cbus3: t.cbus[3],
            Cbus4: t.cbus[4],
            Cbus5: t.cbus[5],
            Cbus6: t.cbus[6],
            InvertTXD: t.invert_txd as u8,
            InvertRXD: t.invert_rxd as u8,
            InvertRTS: t.invert_rts as u8,
            InvertCTS: t.invert_cts as u8,
            InvertDTR: t.invert_dtr as u8,
            InvertDSR: t.invert_dsr as u8,
            InvertDCD: t.invert_dcd as u8,
            InvertRI: t.invert_ri as u8,
            BCDEnable: t.bdc_enable as u8,
            BCDForceCbusPWREN: t.bcd_force_cbus_pwren as u8,
            BCDDisableSleep: t.bcd_disable_sleep as u8,
            I2CSlaveAddress: t.i2c_slave_address,
            I2CDeviceId: t.i2c_device_id,
            I2CDisableSchmitt: t.i2c_disable_schmitt as u8,
            FT1248Cpol: t.ft1248_cpol as u8,
            FT1248Lsb: t.ft1248_lsb as u8,
            FT1248FlowControl: t.ft1248_flow_control as u8,
            RS485EchoSuppress: t.rs485_echo_suppress as u8,
            PowerSaveEnable: t.power_save_enable as u8,
            DriverType: false as u8,
        }
    }
}