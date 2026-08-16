//! FT errors. These error codes match the `FT_STATUS` return values from
//! the FTD2XX library functions.

use ftd2xx_sys::d2xx;
use std::convert::TryFrom;
use thiserror::Error;

/// FT_STATUS error codes as an enum.
#[allow(missing_docs)]
#[derive(Error, Debug, PartialEq)]
#[repr(u32)]
pub enum FtError {
    #[error("FT_INVALID_HANDLE")]
    InvalidHandle = d2xx::FT_INVALID_HANDLE as u32,
    #[error("FT_DEVICE_NOT_FOUND")]
    DeviceNotFound = d2xx::FT_DEVICE_NOT_FOUND as u32,
    #[error("FT_DEVICE_NOT_OPENED")]
    DeviceNotOpened = d2xx::FT_DEVICE_NOT_OPENED as u32,
    #[error("FT_IO_ERROR")]
    IOError = d2xx::FT_IO_ERROR as u32,
    #[error("FT_INSUFFICIENT_RESOURCES")]
    InsufficientResources = d2xx::FT_INSUFFICIENT_RESOURCES as u32,
    #[error("FT_INVALID_PARAMETER")]
    InvalidParameter = d2xx::FT_INVALID_PARAMETER as u32,
    #[error("FT_INVALID_BAUD_RATE")]
    InvalidBaudRate = d2xx::FT_INVALID_BAUD_RATE as u32,
    #[error("FT_DEVICE_NOT_OPENED_FOR_ERASE")]
    DeviceNotOpenedForErase = d2xx::FT_DEVICE_NOT_OPENED_FOR_ERASE as u32,
    #[error("FT_DEVICE_NOT_OPENED_FOR_WRITE")]
    DeviceNotOpenedForWrite = d2xx::FT_DEVICE_NOT_OPENED_FOR_WRITE as u32,
    #[error("FT_FAILED_TO_WRITE_DEVICE")]
    FailedToWriteDevice = d2xx::FT_FAILED_TO_WRITE_DEVICE as u32,
    #[error("FT_EEPROM_READ_FAILED")]
    EepromReadFailed = d2xx::FT_EEPROM_READ_FAILED as u32,
    #[error("FT_EEPROM_WRITE_FAILED")]
    EepromWriteFailed = d2xx::FT_EEPROM_WRITE_FAILED as u32,
    #[error("FT_EEPROM_ERASE_FAILED")]
    EepromEraseFailed = d2xx::FT_EEPROM_ERASE_FAILED as u32,
    #[error("FT_EEPROM_NOT_PRESENT")]
    EepromNotPresent = d2xx::FT_EEPROM_NOT_PRESENT as u32,
    #[error("FT_EEPROM_NOT_PROGRAMMED")]
    EepromNotProgramed = d2xx::FT_EEPROM_NOT_PROGRAMMED as u32,
    #[error("FT_INVALID_ARGS")]
    InvalidArgs = d2xx::FT_INVALID_ARGS as u32,
    #[error("FT_INVAFT_NOT_SUPPORTEDLID_HANDLE")]
    NotSupported = d2xx::FT_NOT_SUPPORTED as u32,
    #[error("FT_OTHER_ERROR")]
    OtherError = d2xx::FT_OTHER_ERROR as u32,

    // The following errors are custom made
    #[error("Trying to write a GPIO input")]
    WriteGPIOInput = 19 as u32,
}

impl TryFrom<u32> for FtError {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Err("FT_OK is not an error."),
            1 => Ok(FtError::InvalidHandle),
            2 => Ok(FtError::DeviceNotFound),
            3 => Ok(FtError::DeviceNotOpened),
            4 => Ok(FtError::IOError),
            5 => Ok(FtError::InsufficientResources),
            6 => Ok(FtError::InvalidParameter),
            7 => Ok(FtError::InvalidBaudRate),
            8 => Ok(FtError::DeviceNotOpenedForErase),
            9 => Ok(FtError::DeviceNotOpenedForWrite),
            10 => Ok(FtError::FailedToWriteDevice),
            11 => Ok(FtError::EepromReadFailed),
            12 => Ok(FtError::EepromWriteFailed),
            13 => Ok(FtError::EepromEraseFailed),
            14 => Ok(FtError::EepromNotPresent),
            15 => Ok(FtError::EepromNotProgramed),
            16 => Ok(FtError::InvalidArgs),
            17 => Ok(FtError::NotSupported),
            18 => Ok(FtError::OtherError),
            _ => Err("Unknown FT error code value"),
        }
    }
}

/// This macro will try to execute the "FT_X" function inside an `unsafe{}`
/// statement. If it returns anything other than a FT_OK status code, it will
/// return from the functions it was called with an `Err(FtError)`.
macro_rules! ft_try {
    ($expr:expr) => {{
        let status = unsafe { $expr };
        if status != d2xx::FT_OK {
            return Err(FtError::try_from(status).unwrap());
        }
    }};
}

pub(crate) use ft_try;
