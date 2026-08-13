//! FT Device type enumeration.

use ftd2xx_sys::d2xx;
use std::convert::From;

/// FT device types
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DevType {
    DevBM = d2xx::FT_DEVICE_BM as u8,
    DevAM = d2xx::FT_DEVICE_AM as u8,
    Dev100AX = d2xx::FT_DEVICE_100AX as u8,
    DevUnknown = d2xx::FT_DEVICE_UNKNOWN as u8,
    Dev2232C = d2xx::FT_DEVICE_2232C as u8,
    Dev232R = d2xx::FT_DEVICE_232R as u8,
    Dev2232H = d2xx::FT_DEVICE_2232H as u8,
    Dev4232H = d2xx::FT_DEVICE_4232H as u8,
    Dev232H = d2xx::FT_DEVICE_232H as u8,
    DevXSeries = d2xx::FT_DEVICE_X_SERIES as u8,
    Dev4222H0 = d2xx::FT_DEVICE_4222H_0 as u8,
    Dev4222H12 = d2xx::FT_DEVICE_4222H_1_2 as u8,
    Dev4222H3 = d2xx::FT_DEVICE_4222H_3 as u8,
    Dev4222Prog = d2xx::FT_DEVICE_4222_PROG as u8,
    Dev900 = d2xx::FT_DEVICE_900 as u8,
    Dev930 = d2xx::FT_DEVICE_930 as u8,
    DevUMFTPD3A = d2xx::FT_DEVICE_UMFTPD3A as u8,
    Dev2233HP = d2xx::FT_DEVICE_2233HP as u8,
    Dev4233HP = d2xx::FT_DEVICE_4233HP as u8,
    Dev2232HP = d2xx::FT_DEVICE_2232HP as u8,
    Dev4232HP = d2xx::FT_DEVICE_4232HP as u8,
    Dev233HP = d2xx::FT_DEVICE_233HP as u8,
    Dev232HP = d2xx::FT_DEVICE_232HP as u8,
    Dev2232HA = d2xx::FT_DEVICE_2232HA as u8,
    Dev4232HA = d2xx::FT_DEVICE_4232HA as u8,
}

impl From<u8> for DevType {
    fn from(device: u8) -> Self {
        match device {
            0 => DevType::DevBM,
            1 => DevType::DevAM,
            2 => DevType::Dev100AX,
            3 => DevType::DevUnknown,
            4 => DevType::Dev2232C,
            5 => DevType::Dev232R,
            6 => DevType::Dev2232H,
            7 => DevType::Dev4232H,
            8 => DevType::Dev232H,
            9 => DevType::DevXSeries,
            10 => DevType::Dev4222H0,
            11 => DevType::Dev4222H12,
            12 => DevType::Dev4222H3,
            13 => DevType::Dev4222Prog,
            14 => DevType::Dev900,
            15 => DevType::Dev930,
            16 => DevType::DevUMFTPD3A,
            17 => DevType::Dev2233HP,
            18 => DevType::Dev4233HP,
            19 => DevType::Dev2232HP,
            20 => DevType::Dev4232HP,
            21 => DevType::Dev233HP,
            22 => DevType::Dev232HP,
            23 => DevType::Dev2232HA,
            24 => DevType::Dev4232HA,
            _ => DevType::DevUnknown,
        }
    }
}
