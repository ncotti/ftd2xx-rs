//! EEPROM structures for the following devices:
//! * FTXSeries

use ftd2xx_sys::d2xx::FT_EEPROM_X_SERIES;

use crate::eeprom::{DevType, DriveCurrent, Eeprom, EepromHeader, EepromStrings};

/// FTXSeries EEPROM configuration.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct EepromFtXSeries {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    pub ac_slow_slew: bool,
    pub ac_schmitt_input: bool,
    pub ac_drive_current: DriveCurrent,
    pub ad_slow_slew: bool,
    pub ad_schmitt_input: bool,
    pub ad_drive_current: DriveCurrent,
    pub cbus: [EepromFtXSeriesCbus; 7],
    pub invert_txd: bool,
    pub invert_rxd: bool,
    pub invert_rts: bool,
    pub invert_cts: bool,
    pub invert_dtr: bool,
    pub invert_dsr: bool,
    pub invert_dcd: bool,
    pub invert_ri: bool,
    pub bdc_enable: bool,
    pub bcd_force_cbus_pwren: bool,
    pub bcd_disable_sleep: bool,
    pub i2c_slave_address: u16,
    pub i2c_device_id: u32,
    pub i2c_disable_schmitt: bool,
    pub ft1248_cpol: bool,
    pub ft1248_lsb: bool,
    pub ft1248_flow_control: bool,
    pub rs485_echo_suppress: bool,
    pub power_save_enable: bool,
}

impl Eeprom for EepromFtXSeries {
    type FtEeprom = FT_EEPROM_X_SERIES;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

/// CBUS Pin multiplexing options
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum EepromFtXSeriesCbus {
    #[default]
    Tristate = 0x00,
    Txled = 0x01,
    Rxled = 0x02,
    TxRxLed = 0x03,
    Pwren = 0x04,
    Sleep = 0x05,
    Drive0 = 0x06,
    Drive1 = 0x07,
    IoMode = 0x08,
    TxDen = 0x09,
    Clk24 = 0x0A,
    Clk12 = 0x0B,
    Clk6 = 0x0C,
    BcdCharger = 0x0D,
    BcdChargerN = 0x0E,
    I2CTXE = 0x0F,
    I2CRXF = 0x10,
    VbusSense = 0x011,
    BitBangWrite = 0x12,
    BitBangRead = 0x13,
    Timestamp = 0x14,
    KeepAwake = 0x015,
}

impl Default for EepromFtXSeries {
    fn default() -> Self {
        EepromFtXSeries {
            common: EepromHeader::new(DevType::DevXSeries),
            strings: EepromStrings::default(),
            ac_slow_slew: false,
            ac_schmitt_input: false,
            ac_drive_current: DriveCurrent::Current4mA,
            ad_slow_slew: false,
            ad_schmitt_input: false,
            ad_drive_current: DriveCurrent::Current4mA,
            cbus: [EepromFtXSeriesCbus::default(); 7],
            invert_txd: false,
            invert_rxd: false,
            invert_rts: false,
            invert_cts: false,
            invert_dtr: false,
            invert_dsr: false,
            invert_dcd: false,
            invert_ri: false,
            bdc_enable: false,
            bcd_force_cbus_pwren: false,
            bcd_disable_sleep: false,
            i2c_slave_address: 0x40,
            i2c_device_id: 0,
            i2c_disable_schmitt: false,
            ft1248_cpol: false,
            ft1248_lsb: false,
            ft1248_flow_control: false,
            rs485_echo_suppress: false,
            power_save_enable: false,
        }
    }
}

impl From<&EepromFtXSeries> for FT_EEPROM_X_SERIES {
    fn from(t: &EepromFtXSeries) -> Self {
        FT_EEPROM_X_SERIES {
            common: (&t.common).into(),
            ACSlowSlew: t.ac_slow_slew as u8,
            ACSchmittInput: t.ac_schmitt_input as u8,
            ACDriveCurrent: t.ac_drive_current as u8,
            ADSlowSlew: t.ad_slow_slew as u8,
            ADSchmittInput: t.ad_schmitt_input as u8,
            ADDriveCurrent: t.ad_drive_current as u8,
            Cbus0: t.cbus[0] as u8,
            Cbus1: t.cbus[1] as u8,
            Cbus2: t.cbus[2] as u8,
            Cbus3: t.cbus[3] as u8,
            Cbus4: t.cbus[4] as u8,
            Cbus5: t.cbus[5] as u8,
            Cbus6: t.cbus[6] as u8,
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

impl From<EepromFtXSeries> for FT_EEPROM_X_SERIES {
    fn from(t: EepromFtXSeries) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_X_SERIES> for EepromFtXSeries {
    fn from(t: FT_EEPROM_X_SERIES) -> Self {
        EepromFtXSeries {
            common: t.common.into(),
            strings: EepromStrings::default(),
            ac_slow_slew: t.ACSlowSlew != 0,
            ac_schmitt_input: t.ACSchmittInput != 0,
            ac_drive_current: t.ACDriveCurrent.into(),
            ad_slow_slew: t.ADSlowSlew != 0,
            ad_schmitt_input: t.ADSchmittInput != 0,
            ad_drive_current: t.ADDriveCurrent.into(),
            cbus: [EepromFtXSeriesCbus::default(); 7],
            invert_txd: t.InvertTXD != 0,
            invert_rxd: t.InvertRXD != 0,
            invert_rts: t.InvertRTS != 0,
            invert_cts: t.InvertCTS != 0,
            invert_dtr: t.InvertDTR != 0,
            invert_dsr: t.InvertDSR != 0,
            invert_dcd: t.InvertDCD != 0,
            invert_ri: t.InvertRI != 0,
            bdc_enable: t.BCDEnable != 0,
            bcd_force_cbus_pwren: t.BCDForceCbusPWREN != 0,
            bcd_disable_sleep: t.BCDDisableSleep != 0,
            i2c_slave_address: t.I2CSlaveAddress,
            i2c_device_id: t.I2CDeviceId,
            i2c_disable_schmitt: t.I2CDisableSchmitt != 0,
            ft1248_cpol: t.FT1248Cpol != 0,
            ft1248_lsb: t.FT1248Lsb != 0,
            ft1248_flow_control: t.FT1248FlowControl != 0,
            rs485_echo_suppress: t.RS485EchoSuppress != 0,
            power_save_enable: t.PowerSaveEnable != 0,
        }
    }
}
