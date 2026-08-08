//! EEPROM structures for the following devices:
//! * FT232H
//! * FT232HP
//! * FT233HP

use crate::eeprom::{DevType, DriveCurrent, Eeprom, EepromHeader, EepromPD, EepromStrings};
use ftd2xx_sys::{FT_EEPROM_232H, FT_EEPROM_232HP, FT_EEPROM_233HP};

/// EEPROM configuration for an FT232H device.
#[derive(Debug, Clone)]
pub struct EepromFt232h {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    /// AC bus slow slew rate
    pub ac_slow_slew: bool,
    /// AC bus schmitt trigger input
    pub ac_schmitt_input: bool,
    /// AC bus drive current in mA
    pub ac_drive_current: DriveCurrent,
    /// AD bus slow slew rate
    pub ad_slow_slew: bool,
    /// AC bus schmitt trigger input
    pub ad_schmitt_input: bool,
    /// AD bus schmitt trigger input
    pub ad_drive_current: DriveCurrent,
    /// ACBUS Pin mux options
    pub cbus: [Ft232hCbus; 10],
    /// Clock polarity high for FT1248 mode
    pub ft1248_cpol_high: bool,
    /// LSB first for FT1248 mode
    pub ft1248_lsb: bool,
    /// Flow control for FT1248 mode
    pub ft1248_flow_control: bool,
    /// 245 FIFO
    pub is_fifo: bool,
    /// 245 FIFO CPU Target
    pub is_fifo_target: bool,
    /// Fast serial
    pub is_fast_serial: bool,
    /// FT1248 mode
    pub is_ft1248: bool,
    /// Enable power save mode
    pub power_save_enable: bool,
}

impl Eeprom for EepromFt232h {
    type FtEeprom = FT_EEPROM_232H;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt232h {
    fn default() -> Self {
        EepromFt232h {
            common: EepromHeader::new(DevType::Dev232H),
            strings: EepromStrings::default(),
            ac_slow_slew: false,
            ac_schmitt_input: false,
            ac_drive_current: DriveCurrent::Current4mA,
            ad_slow_slew: false,
            ad_schmitt_input: false,
            ad_drive_current: DriveCurrent::Current4mA,
            cbus: [Ft232hCbus::Tristate; 10],
            ft1248_cpol_high: false,
            ft1248_lsb: false,
            ft1248_flow_control: false,
            is_fifo: false,
            is_fifo_target: false,
            is_fast_serial: false,
            is_ft1248: false,
            power_save_enable: false,
        }
    }
}

/// Pin Mux for ACBus signals. See Appendix B of FT232H for more details.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Ft232hCbus {
    /// Input pull-up
    Tristate = 0x00,
    /// Pulses low when transmitting data (TXD) to external device.
    Txled = 0x01,
    /// Pulses low when receiving data (RXD) from the external device.
    Rxled = 0x02,
    /// Pulses low when transmitting or receiving data from or to external device.
    TxRxLed = 0x03,
    /// Output is low after the device has been configured by USB, then high
    /// during USB suspend.
    Pwren = 0x04,
    /// Goes low during USB suspend mode.
    Sleep = 0x05,
    /// Output low.
    Drive0 = 0x06,
    /// Output high.
    Drive1 = 0x07,
    /// ACBUS Bit Bang.
    IoMode = 0x08,
    /// Enable line driver with RS485
    Txden = 0x09,
    /// 30 MHz clock output.
    Clk30 = 0x0A,
    /// 15 MHz clock output.
    Clk15 = 0x0B,
    /// 7.5 MHz clock output.
    Clk7_5 = 0x0C,
}

impl From<u8> for Ft232hCbus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Ft232hCbus::Tristate,
            0x01 => Ft232hCbus::Txled,
            0x02 => Ft232hCbus::Rxled,
            0x03 => Ft232hCbus::TxRxLed,
            0x04 => Ft232hCbus::Pwren,
            0x05 => Ft232hCbus::Sleep,
            0x06 => Ft232hCbus::Drive0,
            0x07 => Ft232hCbus::Drive1,
            0x08 => Ft232hCbus::IoMode,
            0x09 => Ft232hCbus::Txden,
            0x0A => Ft232hCbus::Clk30,
            0x0B => Ft232hCbus::Clk15,
            0x0C => Ft232hCbus::Clk7_5,
            _ => Ft232hCbus::Tristate,
        }
    }
}

impl From<&EepromFt232h> for FT_EEPROM_232H {
    fn from(t: &EepromFt232h) -> Self {
        FT_EEPROM_232H {
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
            Cbus7: t.cbus[7] as u8,
            Cbus8: t.cbus[8] as u8,
            Cbus9: t.cbus[9] as u8,
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

impl From<EepromFt232h> for FT_EEPROM_232H {
    fn from(t: EepromFt232h) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_232H> for EepromFt232h {
    fn from(t: FT_EEPROM_232H) -> Self {
        EepromFt232h {
            common: t.common.into(),
            strings: EepromStrings::default(),
            ac_slow_slew: t.ACSlowSlew != 0,
            ac_schmitt_input: t.ACSchmittInput != 0,
            ac_drive_current: t.ACDriveCurrent.into(),
            ad_slow_slew: t.ADSlowSlew != 0,
            ad_schmitt_input: t.ADSchmittInput != 0,
            ad_drive_current: t.ADDriveCurrent.into(),
            cbus: [
                t.Cbus0.into(),
                t.Cbus1.into(),
                t.Cbus2.into(),
                t.Cbus3.into(),
                t.Cbus4.into(),
                t.Cbus5.into(),
                t.Cbus6.into(),
                t.Cbus7.into(),
                t.Cbus8.into(),
                t.Cbus9.into(),
            ],
            ft1248_cpol_high: t.FT1248Cpol != 0,
            ft1248_lsb: t.FT1248Lsb != 0,
            ft1248_flow_control: t.FT1248FlowControl != 0,
            is_fifo: t.IsFifo != 0,
            is_fifo_target: t.IsFifoTar != 0,
            is_fast_serial: t.IsFastSer != 0,
            is_ft1248: t.IsFT1248 != 0,
            power_save_enable: t.PowerSaveEnable != 0,
        }
    }
}

/// FT232HP EEPROM configuration
#[derive(Debug, Clone)]
pub struct EepromFt232hp {
    /// Base FT232H configuration
    pub ft232h: EepromFt232h,
    /// Power delivery aggregate configuration
    pub pd: EepromPD,
}

impl Eeprom for EepromFt232hp {
    type FtEeprom = FT_EEPROM_232HP;

    fn strings(&self) -> &EepromStrings {
        &self.ft232h.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.ft232h.strings
    }
}

impl Default for EepromFt232hp {
    fn default() -> Self {
        EepromFt232hp {
            ft232h: EepromFt232h {
                common: EepromHeader::new(DevType::Dev232HP),
                strings: EepromStrings::default(),
                ac_slow_slew: false,
                ac_schmitt_input: false,
                ac_drive_current: DriveCurrent::Current4mA,
                ad_slow_slew: false,
                ad_schmitt_input: false,
                ad_drive_current: DriveCurrent::Current4mA,
                cbus: [Ft232hCbus::Tristate; 10],
                ft1248_cpol_high: false,
                ft1248_lsb: false,
                ft1248_flow_control: false,
                is_fifo: false,
                is_fifo_target: false,
                is_fast_serial: false,
                is_ft1248: false,
                power_save_enable: false,
            },
            pd: EepromPD::default(),
        }
    }
}

impl From<&EepromFt232hp> for FT_EEPROM_232HP {
    fn from(t: &EepromFt232hp) -> Self {
        FT_EEPROM_232HP {
            ft232h: (&t.ft232h).into(),
            pd: (&t.pd).into(),
        }
    }
}

impl From<EepromFt232hp> for FT_EEPROM_232HP {
    fn from(t: EepromFt232hp) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_232HP> for EepromFt232hp {
    fn from(t: FT_EEPROM_232HP) -> Self {
        EepromFt232hp {
            ft232h: t.ft232h.into(),
            pd: EepromPD::default(),
        }
    }
}

/// FT233HP EEPROM configuration
#[derive(Debug, Clone)]
pub struct EepromFt233hp {
    /// Base FT232H configuration
    pub ft232h: EepromFt232h,
    /// Power delivery aggregate configuration
    pub pd: EepromPD,
}

impl Eeprom for EepromFt233hp {
    type FtEeprom = FT_EEPROM_233HP;

    fn strings(&self) -> &EepromStrings {
        &self.ft232h.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.ft232h.strings
    }
}

impl Default for EepromFt233hp {
    fn default() -> Self {
        EepromFt233hp {
            ft232h: EepromFt232h {
                common: EepromHeader::new(DevType::Dev232HP),
                strings: EepromStrings::default(),
                ac_slow_slew: false,
                ac_schmitt_input: false,
                ac_drive_current: DriveCurrent::Current4mA,
                ad_slow_slew: false,
                ad_schmitt_input: false,
                ad_drive_current: DriveCurrent::Current4mA,
                cbus: [Ft232hCbus::Tristate; 10],
                ft1248_cpol_high: false,
                ft1248_lsb: false,
                ft1248_flow_control: false,
                is_fifo: false,
                is_fifo_target: false,
                is_fast_serial: false,
                is_ft1248: false,
                power_save_enable: false,
            },
            pd: EepromPD::default(),
        }
    }
}

impl From<&EepromFt233hp> for FT_EEPROM_233HP {
    fn from(t: &EepromFt233hp) -> Self {
        FT_EEPROM_233HP {
            ft232h: (&t.ft232h).into(),
            pd: (&t.pd).into(),
        }
    }
}

impl From<EepromFt233hp> for FT_EEPROM_233HP {
    fn from(t: EepromFt233hp) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_233HP> for EepromFt233hp {
    fn from(t: FT_EEPROM_233HP) -> Self {
        EepromFt233hp {
            ft232h: t.ft232h.into(),
            pd: EepromPD::default(),
        }
    }
}
