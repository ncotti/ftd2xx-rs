//! Doc
//!

use crate::eeprom::{DriveCurrent, EepromHeader};
use ftd2xx_sys::FT_EEPROM_4222H;

/// EEPROM configuration for an FT4222H device.
#[allow(missing_docs)]
pub struct EepromFt4222h {
    common: EepromHeader,
    revision: u8,
    i2c_slave_address: u8,
    spi_suspend: u8,
    suspend_out_pol: bool,
    enable_suspend_out: bool,
    clock_slow_slew: bool,
    clock_drive: DriveCurrent,
    slow_slew: [bool; 4],
    io_drive: DriveCurrent,
    ss_pullup: bool,
    ss_pulldown: bool,
    ss_drive: DriveCurrent,
    ss_slow_slew: bool,
    miso_suspend: u8,
    simo_suspend: u8,
    i02_i03_suspend: u8,
    ss_suspend: u8,
    gpios: [EepromFt4222hGpio; 4],
    gpio_falling_edge: bool,
    bcd_disable: bool,
    bcd_output_active_low: bool,
    bcd_drive: DriveCurrent,
}

/// EEPROM configuration for each GPIO port in the FT4222 device
#[allow(missing_docs)]
pub struct EepromFt4222hGpio {
    pub drive: DriveCurrent,
    pub slow_slew: bool,
    pub pulldown: bool,
    pub pullup: bool,
    pub open_drain: bool,
    pub suspend: u8,
}

impl From<EepromFt4222h> for FT_EEPROM_4222H {
    fn from(t: EepromFt4222h) -> Self {
        FT_EEPROM_4222H {
            common: t.common.into(),
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
            SlaveSelect_PullUp: t.ss_pullup as u8,
            SlaveSelect_PullDown: t.ss_pulldown as u8,
            SlaveSelect_Drive: t.ss_drive as u8,
            SlaveSelect_SlowSlew: t.ss_slow_slew as u8,
            MISO_Suspend: t.miso_suspend as u8,
            SIMO_Suspend: t.simo_suspend as u8,
            IO2_IO3_Suspend: t.i02_i03_suspend as u8,
            SlaveSelect_Suspend: t.ss_suspend as u8,
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
