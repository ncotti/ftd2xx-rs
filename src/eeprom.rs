//! # EEPROM module
//!
//! Contains the common trait `Eeprom`, required for all EEPROMs.
//! Also, It brings all modules inside the `eeprom` folder into scope.

mod eeprom_header;
mod eeprom_pd;
mod ft2232;
mod ft2232h;
mod ft232b;
mod ft232h;
mod ft232r;
mod ft4222h;
pub mod ft4232h;
mod ftxseries;

use std::fmt;

pub use crate::eeprom::eeprom_header::EepromHeader;
pub use crate::eeprom::eeprom_pd::{EepromPD, EepromPDO};
use crate::types::DevType;

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

    // fn read(ft_handle: FtHandle) -> Result<Self, FtError>;
    // fn write(&self) -> Result<(), FtError>;
    // fn erase(&self) -> Result<(), FtError>;

    /// Returns all strings that can be stored in the Eeprom as
    /// immutable references. This function is not meant to be called
    /// externally, but rather to be utilized by the getter methods.
    fn strings(&self) -> &EepromStrings;

    /// Returns all strings that can be stored in the Eeprom as
    /// mutable references. This function is not meant to be called
    /// externally, but rather to be utilized by the setter methods.
    fn string_mut(&mut self) -> &mut EepromStrings;

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
