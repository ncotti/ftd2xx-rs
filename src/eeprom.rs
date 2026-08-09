//! # EEPROM module
//!
//! Contains the common trait `Eeprom`, required for all EEPROMs.
//! Also, It brings all modules inside the `eeprom` folder into scope.

mod eeprom_header;
mod eeprom_pd;
// mod ft2232;
// mod ft2232h;
// mod ft232b;
// mod ft232h;
// mod ft232r;
// mod ft4222h;
pub mod ft4232h;
//mod ftxseries;

use std::fmt;

pub use crate::eeprom::eeprom_header::EepromHeader;
pub use crate::eeprom::eeprom_pd::{EepromPD, EepromPDO};
use crate::types::DevType;
use crate::{FtError, FtHandle, classic};

/// Basic EEPROM trait which must be implemented for all devices' EEPROMs.
/// An Eeprom device must define the following tratis:
/// * Sized: A known size at compile time.
/// * Default: A way to create a default valued configuration.
/// * From<FtEeprom>: Values from the `FT_EEPROM_XXX` type should be able
/// * Clone: To duplicate an Eeprom configuration.
/// to be converted into this struct.
pub trait Eeprom: Sized + Default + From<Self::FtEeprom> + Clone {
    /// Holds the reference `FT_EEPROM_XXX` type that this struct
    /// must be able to be converted into, either consuming the value or by
    /// reference.
    type FtEeprom: for<'a> From<&'a Self> + for<'a> From<Self>;

    /// Reads the EEPROM
    fn read(ft_handle: FtHandle) -> Result<Self, FtError> {
        classic::eeprom_read(ft_handle)
    }

    /// Writes to the EEPROM
    fn write(&self) -> Result<(), FtError> {
        classic::eeprom_program(self.handle(), self)
    }

    /// Erases the contents of the EEPROM
    /// TODO, check if the eeprom ends up having all zeros or what
    fn erase(&self) -> Result<(), FtError> {
        classic::erase_ee(self.handle())
    }

    /// Reads user area.
    fn read_user_area(&self) -> Result<Vec<u8>, FtError> {
        let bytes_read = classic::ee_ua_read(self.handle())?;
        Ok(bytes_read)
    }

    /// Writes to user area.
    fn write_user_area(&self, bytes: &Vec<u8>) -> Result<(), FtError> {
        classic::ee_ua_write(self.handle(), bytes)?;
        Ok(())
    }

    /// Return amount of bytes available in user area.
    fn get_user_area_size(&self) -> Result<usize, FtError> {
        classic::ee_ua_size(self.handle())
    }

    /// Returns all strings that can be stored in the Eeprom as
    /// immutable references. This function is not meant to be called
    /// externally, but rather to be utilized by the getter methods.
    fn strings(&self) -> &EepromStrings;

    /// Returns all strings that can be stored in the Eeprom as
    /// mutable references. This function is not meant to be called
    /// externally, but rather to be utilized by the setter methods.
    fn string_mut(&mut self) -> &mut EepromStrings;

    /// Returns a copy of the device's FT handle. This function is not meant
    /// to be called externally.
    fn handle(&self) -> FtHandle;

    /// Returns a mutable reference of the handle.
    fn handle_mut(&mut self) -> &mut FtHandle;

    /// Returns the manufacturer string.
    fn get_manufacturer(&self) -> &String {
        &self.strings().manufacturer
    }

    /// Returns the manufacturer ID string.
    fn get_manufacturer_id(&self) -> &String {
        &self.strings().manufacturer_id
    }

    /// Returns the description string.
    fn get_description(&self) -> &String {
        &self.strings().description
    }

    /// Returns the serial number string.
    fn get_serial_number(&self) -> &String {
        &self.strings().serial_number
    }

    /// Sets the manufacturer string. To write it to the EEPROM, use the
    /// `write()` method.
    fn set_manufacturer(&mut self, manufacturer: impl Into<String>) {
        self.string_mut().manufacturer = manufacturer.into();
    }

    /// Sets the manufacturer ID string. To write it to the EEPROM, use the
    /// `write()` method.
    fn set_manufacturer_id(&mut self, manufacturer_id: impl Into<String>) {
        self.string_mut().manufacturer_id = manufacturer_id.into();
    }

    /// Sets the description string. To write it to the EEPROM, use the
    /// `write()` method.
    fn set_description(&mut self, description: impl Into<String>) {
        self.string_mut().description = description.into();
    }

    /// Sets the serial number string. To write it to the EEPROM, use the
    /// `write()` method.
    fn set_serial_number(&mut self, serial_number: impl Into<String>) {
        self.string_mut().serial_number = serial_number.into();
    }
}

/// These are all the possible Strings that are held in FT EEPROMs.
#[derive(Default, Debug, Clone)]
pub struct EepromStrings {
    /// Manufacturer.
    pub manufacturer: String,
    /// Manufacturer ID.
    pub manufacturer_id: String,
    /// Short description.
    pub description: String,
    /// Serial number.
    pub serial_number: String,
}

impl fmt::Display for EepromStrings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Manufacturer: {}, ID: {}",
            self.manufacturer, self.manufacturer_id
        )?;
        writeln!(f, "Serial number: {}", self.serial_number)?;
        write!(f, "Description: {}", self.description)
    }
}

/// Drive current for each I/O pin, i.e., the maximum allowed current for each
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

impl Default for DriveCurrent {
    fn default() -> Self {
        DriveCurrent::Current4mA
    }
}
