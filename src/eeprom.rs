//! Implementation for reading and writing the EEPROM
//! 
//! 
//! 


mod ft232b;
mod ft232h;
mod ft232r;
mod ft2232;
mod ft2232h;
mod ft4222h;
pub mod ft4232h;
mod ftxseries;



use ftd2xx_sys::{FT_EEPROM_HEADER, FT_EEPROM_PD, FT_EEPROM_PD_PDO_mv_ma};
use crate::types::{DevType, FT_DEFAULT_PRODUCT_ID, FT_DEFAULT_VENDOR_ID};



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
