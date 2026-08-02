//! Implementation for reading and writing the EEPROM

use crate::types::DevType;
use ftd2xx_sys::{
    FT_EEPROM_232B, FT_EEPROM_232H, FT_EEPROM_232HP, FT_EEPROM_232R, FT_EEPROM_233HP,
    FT_EEPROM_2232, FT_EEPROM_2232H, FT_EEPROM_2232HP, FT_EEPROM_2233HP, FT_EEPROM_4222H,
    FT_EEPROM_4232H, FT_EEPROM_4232HP, FT_EEPROM_4233HP, FT_EEPROM_HEADER, FT_EEPROM_PD,
    FT_EEPROM_PD_PDO_mv_ma, FT_EEPROM_X_SERIES,
};

pub struct EepromHeader {
    /// Device type.
    device_type: DevType,
    // Vendor ID.
    vid: u16,
    /// Product ID.
    pid: u16,

    serial_number_enable: bool,

    /// Max power [0;500]
    max_power: u16,

    self_powered: bool,
    remote_wakeup: bool,
    pulldown_enable: bool,

    /// Manufacturer.
    pub manufacturer: String,
    /// Manufacturer ID.
    pub manufacturer_id: String,

    /// Description.
    pub description: String,

    /// Serial number.
    pub serial_number: String,
}

impl EepromHeader {
    fn to_ft_eeprom_header(&self) -> FT_EEPROM_HEADER {
        FT_EEPROM_HEADER {
            deviceType: self.device_type as u32,
            VendorId: self.vid,
            ProductId: self.pid,
            SerNumEnable: self.serial_number_enable as u8,
            MaxPower: self.max_power,
            SelfPowered: self.self_powered as u8,
            RemoteWakeup: self.remote_wakeup as u8,
            PullDownEnable: self.pulldown_enable as u8,
        }
    }
}

pub struct Ft232bEeprom {
    header: EepromHeader,
}

impl Ft232bEeprom {
    fn to_ft_eeprom_232b(&self) -> FT_EEPROM_232B {
        FT_EEPROM_232B {
            common: self.header.to_ft_eeprom_header(),
        }
    }
}

pub struct Ft2232Eeprom {
    pub header: EepromHeader,
    pub cha: Ft2232EepromChannel,
    pub chb: Ft2232EepromChannel,
}

/// FT2232 EEPROM configuration for each of the device's channels (A and B)
#[allow(missing_docs)]
pub struct Ft2232EepromChannel {
    pub is_high_current: bool,
    pub is_fifo: bool,
    pub is_fifo_target: bool,
    pub is_fast_serial: bool,
}

impl Ft2232Eeprom {
    fn to_ft_eeprom_2232(&self) -> FT_EEPROM_2232 {
        FT_EEPROM_2232 {
            common: self.header.to_ft_eeprom_header(),

            // Channel A
            AIsHighCurrent: self.cha.is_high_current as u8,
            AIsFifo: self.cha.is_fifo as u8,
            AIsFifoTar: self.cha.is_fifo_target as u8,
            AIsFastSer: self.cha.is_fast_serial as u8,
            ADriverType: false as u8, // D2XX driver

            // Channel B
            BIsHighCurrent: self.chb.is_high_current as u8,
            BIsFifo: self.chb.is_fifo as u8,
            BIsFifoTar: self.chb.is_fifo_target as u8,
            BIsFastSer: self.chb.is_fast_serial as u8,
            BDriverType: false as u8, // D2XX driver
        }
    }
}

pub struct Ft232rEeprom {
    pub header: EepromHeader,
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
    pub cbus0: u8,
    pub cbus1: u8,
    pub cbus2: u8,
    pub cbus3: u8,
    pub cbus4: u8,
}

impl Ft232rEeprom {
    fn to_ft_eeprom_232r(&self) -> FT_EEPROM_232R {
        FT_EEPROM_232R {
            common: self.header.to_ft_eeprom_header(),
            IsHighCurrent: self.is_high_current as u8,
            UseExtOsc: self.use_external_oscillator as u8,
            InvertTXD: self.invert_txd as u8,
            InvertRXD: self.invert_rxd as u8,
            InvertRTS: self.invert_rts as u8,
            InvertCTS: self.invert_cts as u8,
            InvertDTR: self.invert_dtr as u8,
            InvertDSR: self.invert_dsr as u8,
            InvertDCD: self.invert_dcd as u8,
            InvertRI: self.invert_ri as u8,
            Cbus0: self.cbus0,
            Cbus1: self.cbus1,
            Cbus2: self.cbus2,
            Cbus3: self.cbus3,
            Cbus4: self.cbus4,
            DriverType: true as u8, // D2XX driver
        }
    }
}

pub struct Ft2232hEeprom {
    pub header: EepromHeader,
    pub cha: Ft2232hChannel,
    pub chb: Ft2232hChannel,
    pub power_save_enable: bool,
}

pub struct Ft2232hChannel {
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

impl Ft2232hEeprom {
    fn to_ft_eeprom_2232h(&self) -> FT_EEPROM_2232H {
        FT_EEPROM_2232H {
            common: self.header.to_ft_eeprom_header(),
            ALSlowSlew: self.cha.low_slow_slew as u8,
            ALSchmittInput: self.cha.low_schmitt_input as u8,
            ALDriveCurrent: self.cha.low_drive_current as u8,
            AHSlowSlew: self.cha.high_slow_slew as u8,
            AHSchmittInput: self.cha.high_schmitt_input as u8,
            AHDriveCurrent: self.cha.high_drive_current as u8,
            BLSlowSlew: self.chb.low_slow_slew as u8,
            BLSchmittInput: self.chb.low_schmitt_input as u8,
            BLDriveCurrent: self.chb.low_drive_current as u8,
            BHSlowSlew: self.chb.high_slow_slew as u8,
            BHSchmittInput: self.chb.high_schmitt_input as u8,
            BHDriveCurrent: self.chb.high_drive_current as u8,
            AIsFifo: self.cha.is_fifo as u8,
            AIsFifoTar: self.cha.is_fifo_target as u8,
            AIsFastSer: self.cha.is_fast_serial as u8,
            BIsFifo: self.chb.is_fifo as u8,
            BIsFifoTar: self.chb.is_fifo_target as u8,
            BIsFastSer: self.chb.is_fast_serial as u8,
            PowerSaveEnable: self.power_save_enable as u8,
            ADriverType: false as u8, // D2XX driver
            BDriverType: false as u8, // D2XX driver
        }
    }
}

pub struct Ft4232hEeprom {
    pub header: EepromHeader,
    pub cha: Ft4232hChannel,
    pub chb: Ft4232hChannel,
    pub chc: Ft4232hChannel,
    pub chd: Ft4232hChannel,
}

pub struct Ft4232hChannel {
    slow_slew: bool,
    schmitt_input: bool,
    drive_current: DriveCurrent,
    use_ri_as_txden: bool,
}

impl Ft4232hEeprom {
    fn to_ft_eeprom_4232h(&self) -> FT_EEPROM_4232H {
        FT_EEPROM_4232H {
            common: self.header.to_ft_eeprom_header(),
            ASlowSlew: self.cha.slow_slew as u8,
            ASchmittInput: self.cha.schmitt_input as u8,
            ADriveCurrent: self.cha.drive_current as u8,
            BSlowSlew: self.chb.slow_slew as u8,
            BSchmittInput: self.chb.schmitt_input as u8,
            BDriveCurrent: self.chb.drive_current as u8,
            CSlowSlew: self.chc.slow_slew as u8,
            CSchmittInput: self.chc.schmitt_input as u8,
            CDriveCurrent: self.chc.drive_current as u8,
            DSlowSlew: self.chd.slow_slew as u8,
            DSchmittInput: self.chd.schmitt_input as u8,
            DDriveCurrent: self.chd.drive_current as u8,
            ARIIsTXDEN: self.cha.use_ri_as_txden as u8,
            BRIIsTXDEN: self.chb.use_ri_as_txden as u8,
            CRIIsTXDEN: self.chc.use_ri_as_txden as u8,
            DRIIsTXDEN: self.chd.use_ri_as_txden as u8,
            ADriverType: false as u8,
            BDriverType: false as u8,
            CDriverType: false as u8,
            DDriverType: false as u8,
        }
    }
}

pub struct Ft232hEeprom {
    header: EepromHeader,
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

impl Ft232hEeprom {
    fn to_ft_eeprom_232h(&self) -> FT_EEPROM_232H {
        FT_EEPROM_232H {
            common: self.header.to_ft_eeprom_header(),
            ACSlowSlew: self.ac_slow_slew as u8,
            ACSchmittInput: self.ac_schmitt_input as u8,
            ACDriveCurrent: self.ac_drive_current as u8,
            ADSlowSlew: self.ad_slow_slew as u8,
            ADSchmittInput: self.ad_schmitt_input as u8,
            ADDriveCurrent: self.ad_drive_current as u8,
            Cbus0: self.cbus[0],
            Cbus1: self.cbus[1],
            Cbus2: self.cbus[2],
            Cbus3: self.cbus[3],
            Cbus4: self.cbus[4],
            Cbus5: self.cbus[5],
            Cbus6: self.cbus[6],
            Cbus7: self.cbus[7],
            Cbus8: self.cbus[8],
            Cbus9: self.cbus[9],
            FT1248Cpol: self.ft1248_cpol_high as u8,
            FT1248Lsb: self.ft1248_lsb as u8,
            FT1248FlowControl: self.ft1248_flow_control as u8,
            IsFifo: self.is_fifo as u8,
            IsFifoTar: self.is_fifo_target as u8,
            IsFastSer: self.is_fast_serial as u8,
            IsFT1248: self.is_ft1248 as u8,
            PowerSaveEnable: self.power_save_enable as u8,
            DriverType: false as u8,
        }
    }
}

pub struct FtXSeriesEeprom {
    header: EepromHeader,
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

impl FtXSeriesEeprom {
    fn to_ft_eeprom_x_series(&self) -> FT_EEPROM_X_SERIES {
        FT_EEPROM_X_SERIES {
            common: self.header.to_ft_eeprom_header(),
            ACSlowSlew: self.ac_slow_slew as u8,
            ACSchmittInput: self.ac_schmitt_input as u8,
            ACDriveCurrent: self.ac_drive_current as u8,
            ADSlowSlew: self.ad_slow_slew as u8,
            ADSchmittInput: self.ad_schmitt_input as u8,
            ADDriveCurrent: self.ad_slow_slew as u8,
            Cbus0: self.cbus[0],
            Cbus1: self.cbus[1],
            Cbus2: self.cbus[2],
            Cbus3: self.cbus[3],
            Cbus4: self.cbus[4],
            Cbus5: self.cbus[5],
            Cbus6: self.cbus[6],
            InvertTXD: self.invert_txd as u8,
            InvertRXD: self.invert_rxd as u8,
            InvertRTS: self.invert_rts as u8,
            InvertCTS: self.invert_cts as u8,
            InvertDTR: self.invert_dtr as u8,
            InvertDSR: self.invert_dsr as u8,
            InvertDCD: self.invert_dcd as u8,
            InvertRI: self.invert_ri as u8,
            BCDEnable: self.bdc_enable as u8,
            BCDForceCbusPWREN: self.bcd_force_cbus_pwren as u8,
            BCDDisableSleep: self.bcd_disable_sleep as u8,
            I2CSlaveAddress: self.i2c_slave_address,
            I2CDeviceId: self.i2c_device_id,
            I2CDisableSchmitt: self.i2c_disable_schmitt as u8,
            FT1248Cpol: self.ft1248_cpol as u8,
            FT1248Lsb: self.ft1248_lsb as u8,
            FT1248FlowControl: self.ft1248_flow_control as u8,
            RS485EchoSuppress: self.rs485_echo_suppress as u8,
            PowerSaveEnable: self.power_save_enable as u8,
            DriverType: false as u8,
        }
    }
}

pub struct Ft4222hEeprom {
    header: EepromHeader,
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
    gpios: [Ft4222hGPIO; 4],
    gpio_falling_edge: bool,
    bcd_disable: bool,
    bcd_output_active_low: bool,
    bcd_drive: DriveCurrent,
}

pub struct Ft4222hGPIO {
    pub drive: DriveCurrent,
    pub slow_slew: bool,
    pub pulldown: bool,
    pub pullup: bool,
    pub open_drain: bool,
    pub suspend: u8,
}

impl Ft4222hEeprom {
    fn to_ft_eeprom_4222h(&self) -> FT_EEPROM_4222H {
        FT_EEPROM_4222H {
            common: self.header.to_ft_eeprom_header(),
            Revision: self.revision as u8,
            I2C_Slave_Address: self.i2c_slave_address as u8,
            SPISuspend: self.spi_suspend as u8,
            SuspendOutPol: self.suspend_out_pol as u8,
            EnableSuspendOut: self.enable_suspend_out as u8,
            Clock_SlowSlew: self.clock_slow_slew as u8,
            Clock_Drive: self.clock_drive as u8,
            IO0_SlowSlew: self.slow_slew[0] as u8,
            IO1_SlowSlew: self.slow_slew[1] as u8,
            IO2_SlowSlew: self.slow_slew[2] as u8,
            IO3_SlowSlew: self.slow_slew[3] as u8,
            IO_Drive: self.io_drive as u8,
            SlaveSelect_PullUp: self.ss_pullup as u8,
            SlaveSelect_PullDown: self.ss_pulldown as u8,
            SlaveSelect_Drive: self.ss_drive as u8,
            SlaveSelect_SlowSlew: self.ss_slow_slew as u8,
            MISO_Suspend: self.miso_suspend as u8,
            SIMO_Suspend: self.simo_suspend as u8,
            IO2_IO3_Suspend: self.i02_i03_suspend as u8,
            SlaveSelect_Suspend: self.ss_suspend as u8,
            GPIO0_Drive: self.gpios[0].drive as u8,
            GPIO1_Drive: self.gpios[1].drive as u8,
            GPIO2_Drive: self.gpios[2].drive as u8,
            GPIO3_Drive: self.gpios[3].drive as u8,
            GPIO0_SlowSlew: self.gpios[0].slow_slew as u8,
            GPIO1_SlowSlew: self.gpios[1].slow_slew as u8,
            GPIO2_SlowSlew: self.gpios[2].slow_slew as u8,
            GPIO3_SlowSlew: self.gpios[3].slow_slew as u8,
            GPIO0_PullDown: self.gpios[0].pulldown as u8,
            GPIO1_PullDown: self.gpios[1].pulldown as u8,
            GPIO2_PullDown: self.gpios[2].pulldown as u8,
            GPIO3_PullDown: self.gpios[3].pulldown as u8,
            GPIO0_PullUp: self.gpios[0].pullup as u8,
            GPIO1_PullUp: self.gpios[1].pullup as u8,
            GPIO2_PullUp: self.gpios[2].pullup as u8,
            GPIO3_PullUp: self.gpios[3].pullup as u8,
            GPIO0_OpenDrain: self.gpios[0].open_drain as u8,
            GPIO1_OpenDrain: self.gpios[1].open_drain as u8,
            GPIO2_OpenDrain: self.gpios[2].open_drain as u8,
            GPIO3_OpenDrain: self.gpios[3].open_drain as u8,
            GPIO0_Suspend: self.gpios[0].suspend as u8,
            GPIO1_Suspend: self.gpios[1].suspend as u8,
            GPIO2_Suspend: self.gpios[2].suspend as u8,
            GPIO3_Suspend: self.gpios[3].suspend as u8,
            FallingEdge: self.gpio_falling_edge as u8,
            BCD_Disable: self.bcd_disable as u8,
            BCD_OutputActiveLow: self.bcd_output_active_low as u8,
            BCD_Drive: self.bcd_drive as u8,
        }
    }
}

pub struct EepromPDO {
    mv: [u16; 7],
    ma: [u16; 7],
}

impl EepromPDO {
    fn to_ft_eeprom_pdo(&self) -> FT_EEPROM_PD_PDO_mv_ma {
        FT_EEPROM_PD_PDO_mv_ma {
            PDO1ma: self.ma[0],
            PDO1mv: self.mv[0],
            PDO2ma: self.ma[1],
            PDO2mv: self.mv[1],
            PDO3ma: self.ma[2],
            PDO3mv: self.mv[2],
            PDO4ma: self.ma[3],
            PDO4mv: self.mv[3],
            PDO5ma: self.ma[4],
            PDO5mv: self.mv[4],
            PDO6ma: self.ma[5],
            PDO6mv: self.mv[5],
            PDO7ma: self.ma[6],
            PDO7mv: self.mv[6],
        }
    }
}

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

impl EepromPD {
    fn to_eeprom_pd(&self) -> FT_EEPROM_PD {
        FT_EEPROM_PD {
            srprs: self.srprs as u8,
            sraprs: self.sraprs as u8,
            srrprs: self.srrprs as u8,
            saprs: self.saprs as u8,
            vconns: self.vconns as u8,
            passthru: self.passthru as u8,
            extmcu: self.extmcu as u8,
            pd2en: self.pd2en as u8,
            pd1autoclk: self.pd1autoclk as u8,
            pd2autoclk: self.pd2autoclk as u8,
            useefuse: self.useefuse as u8,
            extvconn: self.extvconn as u8,

            count: self.count,
            srcPin1: self.src_pin[0],
            srcPin2: self.src_pin[1],
            srcPin3: self.src_pin[2],
            srcPin4: self.src_pin[3],
            srcPin5: self.src_pin[4],
            srcPin6: self.src_pin[5],
            srcPin7: self.src_pin[6],

            pd1lden: self.pd1lden,
            pd2lden: self.pd2lden,

            dispin: self.dispin,
            disenbm: self.disenbm,
            disdisbm: self.disdisbm,

            ccselect: self.ccselect,

            iset1: self.iset1,
            iset2: self.iset2,
            iset3: self.iset3,

            extiset: self.extiset as u8,
            isetpd2: self.isetpd2 as u8,
            iseten: self.iseten as u8,

            PDO1_GPIO: self.pdo1_gpio,
            PDO2_GPIO: self.pdo2_gpio,
            PDO3_GPIO: self.pdo3_gpio,
            PDO4_GPIO: self.pdo4_gpio,
            PDO5_GPIO: self.pdo5_gpio,
            PDO6_GPIO: self.pdo6_gpio,
            PDO7_GPIO: self.pdo7_gpio,
            VSET0V_GPIO: self.vset0v_gpio,
            VSAFE5V_GPIO: self.vsafe5v_gpio,

            BM_PDO_Sink: self.bm_pdo_sink.to_ft_eeprom_pdo(),
            BM_PDO_Source: self.bm_pdo_source.to_ft_eeprom_pdo(),
            BM_PDO_Sink_2: self.bm_pdo_sink_2.to_ft_eeprom_pdo(),

            srt: self.srt,
            hrt: self.hrt,
            sct: self.sct,
            dit: self.dit,
            srcrt: self.srcrt,
            trt: self.trt,
            sofft: self.sofft,
            nrt: self.nrt,
            swct: self.swct,
            snkrt: self.snkrt,
            dt: self.dt,
            cnst: self.cnst,
            it: self.it,

            i2caddr: self.i2caddr,
            prou: self.prou,
            trim1: self.trim1,
            trim2: self.trim2,
            extdc: self.extdc as u8,
        }
    }
}

pub struct Ft2233hpEeprom {
    eeprom: Ft2232hEeprom,
    pd: EepromPD,
}

impl Ft2233hpEeprom {
    fn to_eeprom(&self) -> FT_EEPROM_2233HP {
        FT_EEPROM_2233HP {
            ft2232h: self.eeprom.to_ft_eeprom_2232h(),
            pd: self.pd.to_eeprom_pd(),
        }
    }
}

pub struct Ft4233hpEeprom {
    eeprom: Ft4232hEeprom,
    pd: EepromPD,
}

impl Ft4233hpEeprom {
    fn to_eeprom(&self) -> FT_EEPROM_4233HP {
        FT_EEPROM_4233HP {
            ft4232h: self.eeprom.to_ft_eeprom_4232h(),
            pd: self.pd.to_eeprom_pd(),
        }
    }
}

pub struct Ft2232hpEeprom {
    eeprom: Ft2232hEeprom,
    pd: EepromPD,
}

impl Ft2232hpEeprom {
    fn to_eeprom(&self) -> FT_EEPROM_2232HP {
        FT_EEPROM_2232HP {
            ft2232h: self.eeprom.to_ft_eeprom_2232h(),
            pd: self.pd.to_eeprom_pd(),
        }
    }
}

pub struct Ft4232hpEeprom {
    eeprom: Ft4232hEeprom,
    pd: EepromPD,
}

impl Ft4232hpEeprom {
    fn to_eeprom(&self) -> FT_EEPROM_4232HP {
        FT_EEPROM_4232HP {
            ft4232h: self.eeprom.to_ft_eeprom_4232h(),
            pd: self.pd.to_eeprom_pd(),
        }
    }
}

pub struct Ft233hpEeprom {
    eeprom: Ft232hEeprom,
    pd: EepromPD,
}

impl Ft233hpEeprom {
    fn to_eeprom(&self) -> FT_EEPROM_233HP {
        FT_EEPROM_233HP {
            ft232h: self.eeprom.to_ft_eeprom_232h(),
            pd: self.pd.to_eeprom_pd(),
        }
    }
}

pub struct Ft232hpEeprom {
    eeprom: Ft232hEeprom,
    pd: EepromPD,
}

impl Ft232hpEeprom {
    fn to_eeprom(&self) -> FT_EEPROM_232HP {
        FT_EEPROM_232HP {
            ft232h: self.eeprom.to_ft_eeprom_232h(),
            pd: self.pd.to_eeprom_pd(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum DriveCurrent {
    Current4mA = 4,
    Current8mA = 8,
    Current12mA = 12,
    Current16mA = 16,
}
