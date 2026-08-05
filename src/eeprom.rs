//! Implementation for reading and writing the EEPROM

use crate::types::{DevType, FT_DEFAULT_PRODUCT_ID, FT_DEFAULT_VENDOR_ID};
use ftd2xx_sys::{
    FT_EEPROM_232B, FT_EEPROM_232H, FT_EEPROM_232HP, FT_EEPROM_232R, FT_EEPROM_233HP,
    FT_EEPROM_2232, FT_EEPROM_2232H, FT_EEPROM_2232HP, FT_EEPROM_2233HP, FT_EEPROM_4222H,
    FT_EEPROM_4232H, FT_EEPROM_4232HP, FT_EEPROM_4233HP, FT_EEPROM_HEADER, FT_EEPROM_PD,
    FT_EEPROM_PD_PDO_mv_ma, FT_EEPROM_X_SERIES,
};

#[derive(Default, Debug)]
struct EepromStrings {
    /// Manufacturer.
    pub manufacturer: String,
    /// Manufacturer ID.
    pub manufacturer_id: String,
    /// Short description.
    pub description: String,
    /// Serial number.
    pub serial_number: String,
}

pub trait Eeprom: Sized + Default + From<Self::FtEeprom> {
    type FtEeprom: for<'a> From<Self>;

    // fn read(ft_handle: FtHandle) -> Result<Self, FtError>;
    // fn write(&self) -> Result<(), FtError>;
    // fn erase(&self) -> Result<(), FtError>;

    fn strings(&self) -> &EepromStrings;

    fn string_mut(&mut self) -> &mut EepromStrings;

    fn get_manufacturer(&self) -> &str {
        &self.strings().manufacturer
    }

    fn get_manufacturer_id(&self) -> &str {
        &self.strings().manufacturer_id
    }

    fn get_description(&self) -> &str {
        &self.strings().description
    }

    fn get_serial_number(&self) -> &str {
        &self.strings().serial_number
    }

    fn set_manufacturer(&mut self, manufacturer: String) {
        self.string_mut().manufacturer = manufacturer;
    }

    fn set_manufacturer_id(&mut self, manufacturer_id: String) {
        self.string_mut().manufacturer_id = manufacturer_id;
    }

    fn set_description(&mut self, description: String) {
        self.string_mut().description = description;
    }

    fn set_serial_number(&mut self, serial_number: String) {
        self.string_mut().serial_number = serial_number;
    }
}

/// Common EEPROM header used for all devices
#[derive(Debug)]
pub struct EepromHeader {
    /// Device type. This field is nor read or written to the EEPROM, but
    /// rather used by the FTD2XX library to know which EEPROM layout is
    /// handling.
    device_type: DevType,

    /// Vendor ID.
    pub vid: u16,
    /// Product ID.
    pub pid: u16,

    /// If `true`, the `serial_number`` will be announced when the USB device is
    /// connected, and can be retrieved from the `scan()` function.
    /// If `false`, it won't be visible.
    pub serial_number_enable: bool,

    /// Max power drawn from the USB interface. Valid values are: [0;500]
    /// TODO, check, actual power: 2mA * value, this is current.
    pub max_power: u16,

    /// If `true`, the device will be "self_powered", meaning that it won't
    /// draw any current from the USB interface, but rather use its own
    /// external power source. When set to `true`, the `max_power` should be
    /// set to 0mA.
    pub self_powered: bool,

    /// Enable remote wake-up. TODO
    pub remote_wakeup: bool,

    /// If `true`, the device's IO pins will be connected to an internal
    /// pulldown resistor when the in USB suspend mode.
    pub pulldown_enable: bool,
}

impl From<EepromHeader> for FT_EEPROM_HEADER {
    fn from(t: EepromHeader) -> Self {
        FT_EEPROM_HEADER {
            deviceType: t.device_type as u32,
            VendorId: t.vid,
            ProductId: t.pid,
            SerNumEnable: t.serial_number_enable as u8,
            MaxPower: t.max_power,
            SelfPowered: t.self_powered as u8,
            RemoteWakeup: t.remote_wakeup as u8,
            PullDownEnable: t.pulldown_enable as u8,
        }
    }
}

impl From<FT_EEPROM_HEADER> for EepromHeader {
    fn from(t: FT_EEPROM_HEADER) -> Self {
        EepromHeader {
            device_type: DevType::from(t.deviceType as u8),
            vid: t.VendorId,
            pid: t.ProductId,
            serial_number_enable: t.SerNumEnable != 0,
            max_power: t.MaxPower,
            self_powered: t.SelfPowered != 0,
            remote_wakeup: t.RemoteWakeup != 0,
            pulldown_enable: t.PullDownEnable != 0,
        }
    }
}

impl EepromHeader {
    /// Create a new EEPROM Header configuration with sensible default values.
    fn new(device_type: DevType) -> Self {
        EepromHeader {
            device_type: device_type,
            vid: FT_DEFAULT_VENDOR_ID,
            pid: FT_DEFAULT_PRODUCT_ID,
            serial_number_enable: true,
            max_power: 250,
            self_powered: false,
            remote_wakeup: false,
            pulldown_enable: true,
        }
    }
}

/// EEPROM configuration for an FT232B device.
pub struct EepromFt232b {
    /// EEPROM common configuration
    common: EepromHeader,
}

impl From<EepromFt232b> for FT_EEPROM_232B {
    fn from(t: EepromFt232b) -> Self {
        FT_EEPROM_232B {
            common: t.common.into(),
        }
    }
}

/// EEPROM configuration for an FT2232 device.
pub struct EepromFt2232 {
    /// EEPROM common configuration
    pub common: EepromHeader,
    /// Channel A configuration
    pub cha: EepromFt2232Channel,
    /// Channel B configuration
    pub chb: EepromFt2232Channel,
}

/// FT2232 EEPROM configuration for each of the device's channels (A and B)
#[allow(missing_docs)]
pub struct EepromFt2232Channel {
    pub is_high_current: bool,
    pub is_fifo: bool,
    pub is_fifo_target: bool,
    pub is_fast_serial: bool,
}

impl From<EepromFt2232> for FT_EEPROM_2232 {
    fn from(t: EepromFt2232) -> Self {
        FT_EEPROM_2232 {
            common: t.common.into(),

            // Channel A
            AIsHighCurrent: t.cha.is_high_current as u8,
            AIsFifo: t.cha.is_fifo as u8,
            AIsFifoTar: t.cha.is_fifo_target as u8,
            AIsFastSer: t.cha.is_fast_serial as u8,
            ADriverType: false as u8, // D2XX driver

            // Channel B
            BIsHighCurrent: t.chb.is_high_current as u8,
            BIsFifo: t.chb.is_fifo as u8,
            BIsFifoTar: t.chb.is_fifo_target as u8,
            BIsFastSer: t.chb.is_fast_serial as u8,
            BDriverType: false as u8, // D2XX driver
        }
    }
}

/// EEPROM configuration for an FT232R device.
#[allow(missing_docs)]
pub struct EepromFt232r {
    pub common: EepromHeader,
    pub is_high_current: bool,
    pub use_external_oscillator: bool,
    pub invert_txd: bool,
    pub invert_rxd: bool,
    pub invert_rts: bool,
    pub invert_cts: bool,
    pub invert_dtr: bool,
    pub invert_dsr: bool,
    pub invert_dcd: bool,
    pub invert_ri: bool,
    pub cbus: [u8; 5],
}

impl From<EepromFt232r> for FT_EEPROM_232R {
    fn from(t: EepromFt232r) -> Self {
        FT_EEPROM_232R {
            common: t.common.into(),
            IsHighCurrent: t.is_high_current as u8,
            UseExtOsc: t.use_external_oscillator as u8,
            InvertTXD: t.invert_txd as u8,
            InvertRXD: t.invert_rxd as u8,
            InvertRTS: t.invert_rts as u8,
            InvertCTS: t.invert_cts as u8,
            InvertDTR: t.invert_dtr as u8,
            InvertDSR: t.invert_dsr as u8,
            InvertDCD: t.invert_dcd as u8,
            InvertRI: t.invert_ri as u8,
            Cbus0: t.cbus[0],
            Cbus1: t.cbus[1],
            Cbus2: t.cbus[2],
            Cbus3: t.cbus[3],
            Cbus4: t.cbus[4],
            DriverType: true as u8, // D2XX driver
        }
    }
}

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

/// EEPROM configuration for an FT4232H device.
#[allow(missing_docs)]
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

/// Common Power Delivery Output (PDO) currents and voltages.
pub struct EepromPDO {
    /// Voltage delivered from power pins [0;51100]mV
    mv: [u16; 7],
    /// Current delivered from power pins [0;10230]mA
    ma: [u16; 7],
}

impl From<EepromPDO> for FT_EEPROM_PD_PDO_mv_ma {
    fn from(t: EepromPDO) -> Self {
        FT_EEPROM_PD_PDO_mv_ma {
            PDO1ma: t.ma[0],
            PDO1mv: t.mv[0],
            PDO2ma: t.ma[1],
            PDO2mv: t.mv[1],
            PDO3ma: t.ma[2],
            PDO3mv: t.mv[2],
            PDO4ma: t.ma[3],
            PDO4mv: t.mv[3],
            PDO5ma: t.ma[4],
            PDO5mv: t.mv[4],
            PDO6ma: t.ma[5],
            PDO6mv: t.mv[5],
            PDO7ma: t.ma[6],
            PDO7mv: t.mv[6],
        }
    }
}

/// Common Power Delivery (PD) configuration. Power delivery devices have a "P"
/// at the end of their name.
#[allow(missing_docs)]
pub struct EepromPD {
    srprs: bool,
    sraprs: bool,
    srrprs: bool,
    saprs: bool,
    vconns: bool,
    passthru: bool,
    extmcu: bool,
    pd2en: bool,
    pd1autoclk: bool,
    pd2autoclk: bool,
    useefuse: bool,
    extvconn: bool,

    count: u8,
    src_pin: [u8; 7],
    pd1lden: u8,
    pd2lden: u8,
    dispin: u8,
    disenbm: u8,
    disdisbm: u8,
    ccselect: u8,

    iset1: u8,
    iset2: u8,
    iset3: u8,
    extiset: bool,
    isetpd2: bool,
    iseten: bool,

    pdo1_gpio: [u8; 7],
    pdo2_gpio: [u8; 7],
    pdo3_gpio: [u8; 7],
    pdo4_gpio: [u8; 7],
    pdo5_gpio: [u8; 7],
    pdo6_gpio: [u8; 7],
    pdo7_gpio: [u8; 7],
    vset0v_gpio: [u8; 7],
    vsafe5v_gpio: [u8; 7],

    bm_pdo_sink: EepromPDO,
    bm_pdo_source: EepromPDO,
    bm_pdo_sink_2: EepromPDO,

    srt: u8,
    hrt: u8,
    sct: u8,
    dit: u8,
    srcrt: u16,
    trt: u16,
    sofft: u16,
    nrt: u16,
    swct: u16,
    snkrt: u16,
    dt: u8,
    cnst: u8,
    it: u16,

    i2caddr: u8,
    prou: u32,
    trim1: u32,
    trim2: u32,
    extdc: bool,
}

impl From<EepromPD> for FT_EEPROM_PD {
    fn from(t: EepromPD) -> Self {
        FT_EEPROM_PD {
            srprs: t.srprs as u8,
            sraprs: t.sraprs as u8,
            srrprs: t.srrprs as u8,
            saprs: t.saprs as u8,
            vconns: t.vconns as u8,
            passthru: t.passthru as u8,
            extmcu: t.extmcu as u8,
            pd2en: t.pd2en as u8,
            pd1autoclk: t.pd1autoclk as u8,
            pd2autoclk: t.pd2autoclk as u8,
            useefuse: t.useefuse as u8,
            extvconn: t.extvconn as u8,

            count: t.count,
            srcPin1: t.src_pin[0],
            srcPin2: t.src_pin[1],
            srcPin3: t.src_pin[2],
            srcPin4: t.src_pin[3],
            srcPin5: t.src_pin[4],
            srcPin6: t.src_pin[5],
            srcPin7: t.src_pin[6],

            pd1lden: t.pd1lden,
            pd2lden: t.pd2lden,

            dispin: t.dispin,
            disenbm: t.disenbm,
            disdisbm: t.disdisbm,

            ccselect: t.ccselect,

            iset1: t.iset1,
            iset2: t.iset2,
            iset3: t.iset3,

            extiset: t.extiset as u8,
            isetpd2: t.isetpd2 as u8,
            iseten: t.iseten as u8,

            PDO1_GPIO: t.pdo1_gpio,
            PDO2_GPIO: t.pdo2_gpio,
            PDO3_GPIO: t.pdo3_gpio,
            PDO4_GPIO: t.pdo4_gpio,
            PDO5_GPIO: t.pdo5_gpio,
            PDO6_GPIO: t.pdo6_gpio,
            PDO7_GPIO: t.pdo7_gpio,
            VSET0V_GPIO: t.vset0v_gpio,
            VSAFE5V_GPIO: t.vsafe5v_gpio,

            BM_PDO_Sink: t.bm_pdo_sink.into(),
            BM_PDO_Source: t.bm_pdo_source.into(),
            BM_PDO_Sink_2: t.bm_pdo_sink_2.into(),

            srt: t.srt,
            hrt: t.hrt,
            sct: t.sct,
            dit: t.dit,
            srcrt: t.srcrt,
            trt: t.trt,
            sofft: t.sofft,
            nrt: t.nrt,
            swct: t.swct,
            snkrt: t.snkrt,
            dt: t.dt,
            cnst: t.cnst,
            it: t.it,

            i2caddr: t.i2caddr,
            prou: t.prou,
            trim1: t.trim1,
            trim2: t.trim2,
            extdc: t.extdc as u8,
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

/// Drive current of each I/O pin, i.e., the maximum allowed current for each
/// pin to source/sink, in mA.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum DriveCurrent {
    Current4mA = 4,
    Current8mA = 8,
    Current12mA = 12,
    Current16mA = 16,
    CurrentInvalid = 0,
}

impl From<u8> for DriveCurrent {
    fn from(value: u8) -> Self {
        match value {
            4 => Self::Current4mA,
            8 => Self::Current8mA,
            12 => Self::Current12mA,
            16 => Self::Current16mA,
            _ => Self::CurrentInvalid,
        }
    }
}
