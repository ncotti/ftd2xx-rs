//! EEPROM structures for the following devices:
//! * FT4222H

use crate::eeprom::{DevType, DriveCurrent, Eeprom, EepromHeader, EepromStrings};
use ftd2xx_sys::FT_EEPROM_4222H;

/// FT4232H EEPROM configuration.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct EepromFt4222h {
    /// Common EEPROM contents for all devices.
    pub common: EepromHeader,
    /// EEPROM strings: manufacturer, ID, serial number and description.
    strings: EepromStrings,
    pub revision: u8,
    pub i2c_slave_address: u8,
    pub spi_suspend: u8,
    pub suspend_out_pol: bool,
    pub enable_suspend_out: bool,
    pub clock_slow_slew: bool,
    pub clock_drive: DriveCurrent,
    pub slow_slew: [bool; 4],
    pub io_drive: DriveCurrent,
    pub miso_suspend: u8,
    pub simo_suspend: u8,
    pub i02_i03_suspend: u8,
    /// Configuration for GPIO0, GPIO1, GPIO2, GPIO3 and SS (Slave Select).
    pub gpios: [EepromFt4222hGpio; 5],
    pub gpio_falling_edge: bool,
    pub bcd_disable: bool,
    pub bcd_output_active_low: bool,
    pub bcd_drive: DriveCurrent,
}

impl Eeprom for EepromFt4222h {
    type FtEeprom = FT_EEPROM_4222H;

    fn strings(&self) -> &EepromStrings {
        &self.strings
    }

    fn string_mut(&mut self) -> &mut EepromStrings {
        &mut self.strings
    }
}

impl Default for EepromFt4222h {
    fn default() -> Self {
        EepromFt4222h {
            common: EepromHeader::new(DevType::Dev4222H0),
            strings: EepromStrings::default(),
            revision: 'A' as u8,
            i2c_slave_address: 0x40,
            spi_suspend: 0,
            suspend_out_pol: false,
            enable_suspend_out: false,
            clock_slow_slew: false,
            clock_drive: DriveCurrent::Current4mA,
            slow_slew: [false; 4],
            io_drive: DriveCurrent::Current4mA,
            miso_suspend: 2,
            simo_suspend: 2,
            i02_i03_suspend: 2,
            gpios: [EepromFt4222hGpio::default(); 5],
            gpio_falling_edge: false,
            bcd_disable: true,
            bcd_output_active_low: false,
            bcd_drive: DriveCurrent::Current4mA,
        }
    }
}

/// EEPROM configuration for each GPIO port in the FT4222 device
#[allow(missing_docs)]
#[derive(Debug, Clone, Default, Copy)]
pub struct EepromFt4222hGpio {
    pub drive: DriveCurrent,
    pub slow_slew: bool,
    pub pulldown: bool,
    pub pullup: bool,
    pub open_drain: bool,
    pub suspend: u8,
}

impl From<&EepromFt4222h> for FT_EEPROM_4222H {
    fn from(t: &EepromFt4222h) -> Self {
        FT_EEPROM_4222H {
            common: (&t.common).into(),
            Revision: t.revision as u8,
            I2C_Slave_Address: t.i2c_slave_address as u8,
            SPISuspend: t.spi_suspend as u8,
            SuspendOutPol: t.suspend_out_pol as u8,
            EnableSuspendOut: t.enable_suspend_out as u8,
            Clock_SlowSlew: t.clock_slow_slew as u8,
            Clock_Drive: t.clock_drive as u8,
            IO0_SlowSlew: t.slow_slew[0] as u8,
            IO1_SlowSlew: t.slow_slew[1] as u8,
            IO2_SlowSlew: t.slow_slew[2] as u8,
            IO3_SlowSlew: t.slow_slew[3] as u8,
            IO_Drive: t.io_drive as u8,
            SlaveSelect_PullUp: t.gpios[4].pullup as u8,
            SlaveSelect_PullDown: t.gpios[4].pulldown as u8,
            SlaveSelect_Drive: t.gpios[4].drive as u8,
            SlaveSelect_SlowSlew: t.gpios[4].slow_slew as u8,
            MISO_Suspend: t.miso_suspend as u8,
            SIMO_Suspend: t.simo_suspend as u8,
            IO2_IO3_Suspend: t.i02_i03_suspend as u8,
            SlaveSelect_Suspend: t.gpios[4].suspend as u8,
            GPIO0_Drive: t.gpios[0].drive as u8,
            GPIO1_Drive: t.gpios[1].drive as u8,
            GPIO2_Drive: t.gpios[2].drive as u8,
            GPIO3_Drive: t.gpios[3].drive as u8,
            GPIO0_SlowSlew: t.gpios[0].slow_slew as u8,
            GPIO1_SlowSlew: t.gpios[1].slow_slew as u8,
            GPIO2_SlowSlew: t.gpios[2].slow_slew as u8,
            GPIO3_SlowSlew: t.gpios[3].slow_slew as u8,
            GPIO0_PullDown: t.gpios[0].pulldown as u8,
            GPIO1_PullDown: t.gpios[1].pulldown as u8,
            GPIO2_PullDown: t.gpios[2].pulldown as u8,
            GPIO3_PullDown: t.gpios[3].pulldown as u8,
            GPIO0_PullUp: t.gpios[0].pullup as u8,
            GPIO1_PullUp: t.gpios[1].pullup as u8,
            GPIO2_PullUp: t.gpios[2].pullup as u8,
            GPIO3_PullUp: t.gpios[3].pullup as u8,
            GPIO0_OpenDrain: t.gpios[0].open_drain as u8,
            GPIO1_OpenDrain: t.gpios[1].open_drain as u8,
            GPIO2_OpenDrain: t.gpios[2].open_drain as u8,
            GPIO3_OpenDrain: t.gpios[3].open_drain as u8,
            GPIO0_Suspend: t.gpios[0].suspend as u8,
            GPIO1_Suspend: t.gpios[1].suspend as u8,
            GPIO2_Suspend: t.gpios[2].suspend as u8,
            GPIO3_Suspend: t.gpios[3].suspend as u8,
            FallingEdge: t.gpio_falling_edge as u8,
            BCD_Disable: t.bcd_disable as u8,
            BCD_OutputActiveLow: t.bcd_output_active_low as u8,
            BCD_Drive: t.bcd_drive as u8,
        }
    }
}

impl From<EepromFt4222h> for FT_EEPROM_4222H {
    fn from(t: EepromFt4222h) -> Self {
        Self::from(&t)
    }
}

impl From<FT_EEPROM_4222H> for EepromFt4222h {
    fn from(t: FT_EEPROM_4222H) -> Self {
        EepromFt4222h {
            common: t.common.into(),
            strings: EepromStrings::default(),
            revision: t.Revision,
            i2c_slave_address: t.I2C_Slave_Address,
            spi_suspend: t.SPISuspend,
            suspend_out_pol: t.SuspendOutPol != 0,
            enable_suspend_out: t.EnableSuspendOut != 0,
            clock_slow_slew: t.Clock_SlowSlew != 0,
            clock_drive: t.Clock_Drive.into(),
            slow_slew: [
                t.IO0_SlowSlew != 0,
                t.IO1_SlowSlew != 0,
                t.IO2_SlowSlew != 0,
                t.IO3_SlowSlew != 0,
            ],
            io_drive: t.IO_Drive.into(),
            miso_suspend: t.MISO_Suspend,
            simo_suspend: t.SIMO_Suspend,
            i02_i03_suspend: t.IO2_IO3_Suspend,
            gpios: [
                EepromFt4222hGpio {
                    drive: t.GPIO0_Drive.into(),
                    slow_slew: t.GPIO0_SlowSlew != 0,
                    pulldown: t.GPIO0_PullDown != 0,
                    pullup: t.GPIO0_PullUp != 0,
                    open_drain: t.GPIO0_OpenDrain != 0,
                    suspend: t.GPIO0_Suspend,
                },
                EepromFt4222hGpio {
                    drive: t.GPIO1_Drive.into(),
                    slow_slew: t.GPIO1_SlowSlew != 0,
                    pulldown: t.GPIO1_PullDown != 0,
                    pullup: t.GPIO1_PullUp != 0,
                    open_drain: t.GPIO1_OpenDrain != 0,
                    suspend: t.GPIO1_Suspend,
                },
                EepromFt4222hGpio {
                    drive: t.GPIO2_Drive.into(),
                    slow_slew: t.GPIO2_SlowSlew != 0,
                    pulldown: t.GPIO2_PullDown != 0,
                    pullup: t.GPIO2_PullUp != 0,
                    open_drain: t.GPIO2_OpenDrain != 0,
                    suspend: t.GPIO2_Suspend,
                },
                EepromFt4222hGpio {
                    drive: t.GPIO3_Drive.into(),
                    slow_slew: t.GPIO3_SlowSlew != 0,
                    pulldown: t.GPIO3_PullDown != 0,
                    pullup: t.GPIO3_PullUp != 0,
                    open_drain: t.GPIO3_OpenDrain != 0,
                    suspend: t.GPIO3_Suspend,
                },
                EepromFt4222hGpio {
                    drive: t.SlaveSelect_Drive.into(),
                    slow_slew: t.SlaveSelect_SlowSlew != 0,
                    pulldown: t.SlaveSelect_PullDown != 0,
                    pullup: t.SlaveSelect_PullUp != 0,
                    open_drain: false,
                    suspend: t.SlaveSelect_Suspend,
                },
            ],
            gpio_falling_edge: t.FallingEdge != 0,
            bcd_disable: t.BCD_Disable != 0,
            bcd_output_active_low: t.BCD_OutputActiveLow != 0,
            bcd_drive: t.BCD_Drive.into(),
        }
    }
}
