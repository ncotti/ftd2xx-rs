//! Bit bang

use crate::DevInfo;
use crate::Device;
use crate::FtError;
use crate::classic;

use crate::BitMode;

/// Establishes the pin direction as either input or output
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PinDir {
    /// GPIO input
    Input = 0,
    /// GPIO output
    Output = 1,
}

/// An FTDI device configured in asynchronous bit-bang mode
pub struct BitBanger {
    device: Device,
    pin_dirs: [PinDir; 8],
}

impl TryFrom<u32> for BitBanger {
    type Error = FtError;

    fn try_from(value: u32) -> Result<Self, FtError> {
        let device = Device::try_from(value)?;
        classic::set_bit_mode(device.handle, 0, BitMode::AsyncBitBang)?;
        classic::set_baud_rate(device.handle, 9600)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<&str> for BitBanger {
    type Error = FtError;

    fn try_from(description: &str) -> Result<Self, FtError> {
        let device = Device::try_from(description)?;
        classic::set_bit_mode(device.handle, 0, BitMode::AsyncBitBang)?;
        classic::set_baud_rate(device.handle, 9600)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<String> for BitBanger {
    type Error = FtError;

    fn try_from(description: String) -> Result<Self, FtError> {
        let device = Device::try_from(description)?;
        classic::set_bit_mode(device.handle, 0, BitMode::AsyncBitBang)?;
        classic::set_baud_rate(device.handle, 9600)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<DevInfo> for BitBanger {
    type Error = FtError;

    fn try_from(info: DevInfo) -> Result<Self, FtError> {
        let device = Device::try_from(info)?;
        classic::set_bit_mode(device.handle, 0, BitMode::AsyncBitBang)?;
        classic::set_baud_rate(device.handle, 9600)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<&DevInfo> for BitBanger {
    type Error = FtError;

    fn try_from(info: &DevInfo) -> Result<Self, FtError> {
        let device = Device::try_from(info)?;
        classic::set_bit_mode(device.handle, 0, BitMode::AsyncBitBang)?;
        classic::set_baud_rate(device.handle, 9600)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl BitBanger {
    /// Will scan for devices, and attempt to connect to the first device
    /// found, if any.
    pub fn new() -> Result<Self, FtError> {
        let device = Self::try_from(0)?;
        Ok(device)
    }

    /// Reads a GPIO pin. Returns "1" if high, or "0" if low.
    ///
    /// The value read is only updated after the next baud rate tick.
    /// If the pin was defined as an output, the value set will be returned.
    pub fn read(&self, pin: u8) -> Result<u8, FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        let gpio_byte = self.read_all()?;
        Ok((gpio_byte >> pin) & 0x01)
    }

    /// Reads all GPIO pins defined as inputs.
    ///
    /// Returns an u8, where each bit corresponds to the electrical value of
    /// the pin (MSB is pin 7, LSB is pin 0).
    /// If a pin is defined as an output, the value set will be returned.
    pub fn read_all(&self) -> Result<u8, FtError> {
        classic::restart_in_task(self.device.handle)?;
        let byte = classic::get_bit_mode(self.device.handle)?;
        Ok(byte)
    }

    /// Writes a GPIO pin.
    ///
    /// The value will be written after the next baud rate tick.
    /// An error will be returned if trying to write a GPIO which was set as
    /// input.
    pub fn write(&self, pin: u8, value: u8) -> Result<(), FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }

        if self.pin_dirs[pin as usize] == PinDir::Input {
            return Err(FtError::WriteGPIOInput);
        }

        let mut pin_values = self.read_all()?;

        if value != 0 {
            pin_values |= 1 << pin;
        } else {
            pin_values &= !(1 << pin);
        }

        let pin_values: Vec<u8> = vec![pin_values];
        let bytes_written = classic::write(self.device.handle, &pin_values)?;
        if bytes_written != 1 {
            return Err(FtError::IOError);
        }
        Ok(())
    }

    /// Writes all GPIO pins.
    ///
    /// It receives an 8-bit values, where each bit maps to a GPIO pin. If a
    /// pin was not set as an output, then its value will be ignored.
    pub fn write_all(&self, value: u8) -> Result<(), FtError> {
        let pin_values: Vec<u8> = vec![value];
        let bytes_written = classic::write(self.device.handle, &pin_values)?;
        if bytes_written != 1 {
            return Err(FtError::IOError);
        }
        Ok(())
    }

    /// Sets a GPIO pin as an input.
    pub fn set_input(&mut self, pin: u8) -> Result<(), FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        self.pin_dirs[pin as usize] = PinDir::Input;
        classic::set_bit_mode(self.device.handle, self.get_umask(), BitMode::AsyncBitBang)?;
        Ok(())
    }

    /// Sets a GPIO pin as an output.
    pub fn set_output(&mut self, pin: u8) -> Result<(), FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        self.pin_dirs[pin as usize] = PinDir::Output;
        classic::set_bit_mode(self.device.handle, self.get_umask(), BitMode::AsyncBitBang)?;
        Ok(())
    }

    /// Returns the current pin direction (Input or Output).
    pub fn get_pin_dir(&self, pin: u8) -> Result<PinDir, FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        Ok(self.pin_dirs[pin as usize])
    }

    /// UMASK is an 8-bit value, where a "1" means that the pin is set as an
    /// output and a "0" means that the pin is set as an input.
    fn get_umask(&mut self) -> u8 {
        let umask = ((self.pin_dirs[7] as u8) << 7)
            | ((self.pin_dirs[6] as u8) << 6)
            | ((self.pin_dirs[5] as u8) << 5)
            | ((self.pin_dirs[4] as u8) << 4)
            | ((self.pin_dirs[3] as u8) << 3)
            | ((self.pin_dirs[2] as u8) << 2)
            | ((self.pin_dirs[1] as u8) << 1)
            | ((self.pin_dirs[0] as u8) << 0);

        umask
    }

    /// Sets the baud rate. Writes and reads are updated whenever a baud rate
    /// tick is issued. It accepts any value for baud rate, not only standard
    /// ones.
    pub fn set_baud_rate(&self, baud_rate: u32) -> Result<(), FtError> {
        classic::set_baud_rate(self.device.handle, baud_rate)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// This test requires the eight pins from channel A and channel B to be
    /// connected between each other.
    #[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_io() -> Result<(), FtError> {
        let mut cha = BitBanger::try_from(0)?;
        let mut chb = BitBanger::try_from(1)?;

        for i in 0..8 {
            cha.set_input(i)?;
            chb.set_output(i)?;
        }

        chb.write_all(0b10101010)?;
        let input_value = cha.read_all()?;
        assert!(input_value == 0b10101010);

        chb.write(0, 1)?;
        let input_value = cha.read_all()?;
        assert!(input_value == 0b10101011);

        chb.write(3, 0)?;
        let input_value = cha.read_all()?;
        assert!(input_value == 0b10100011);

        cha.write(0, 1).expect_err("Inputs shouldn't be written");
        Ok(())
    }
}
