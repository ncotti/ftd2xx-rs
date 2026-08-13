//! Doc

use ftd2xx_sys::d2xx::{FT_EEPROM_PD, FT_EEPROM_PD_PDO_mv_ma};

/// Common Power Delivery Output (PDO) currents and voltages.
#[derive(Debug, Clone, Default)]
pub struct EepromPDO {
    /// Voltage delivered from power pins [0;51100]mV
    mv: [u16; 7],
    /// Current delivered from power pins [0;10230]mA
    ma: [u16; 7],
}

impl From<&EepromPDO> for FT_EEPROM_PD_PDO_mv_ma {
    fn from(t: &EepromPDO) -> Self {
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

impl From<EepromPDO> for FT_EEPROM_PD_PDO_mv_ma {
    fn from(t: EepromPDO) -> Self {
        Self::from(&t)
    }
}

/// Common Power Delivery (PD) configuration. Power delivery devices have a "P"
/// at the end of their name.
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
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

impl From<&EepromPD> for FT_EEPROM_PD {
    fn from(t: &EepromPD) -> Self {
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

            BM_PDO_Sink: (&t.bm_pdo_sink).into(),
            BM_PDO_Source: (&t.bm_pdo_source).into(),
            BM_PDO_Sink_2: (&t.bm_pdo_sink_2).into(),

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

impl From<EepromPD> for FT_EEPROM_PD {
    fn from(t: EepromPD) -> Self {
        Self::from(&t)
    }
}
