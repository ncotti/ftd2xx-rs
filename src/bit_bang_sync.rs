//! Bit bang

use crate::DevInfo;
use crate::Device;
use crate::FtError;
use crate::classic;

use crate::BitMode;
use crate::bit_bang::PinDir;

/// An FTDI device configured in synchronous bit-bang mode
pub struct BitBangerSync {
    device: Device,
    pin_dirs: [PinDir; 8],
}

impl TryFrom<u32> for BitBangerSync {
    type Error = FtError;

    fn try_from(value: u32) -> Result<Self, FtError> {
        let device = Device::try_from(value)?;
        classic::set_bit_mode(device.handle, 0, BitMode::SyncBitBang)?;
        classic::set_baud_rate(device.handle, 115200)?;
        classic::purge(device.handle, true, true)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<&str> for BitBangerSync {
    type Error = FtError;

    fn try_from(description: &str) -> Result<Self, FtError> {
        let device = Device::try_from(description)?;
        classic::set_bit_mode(device.handle, 0, BitMode::SyncBitBang)?;
        classic::set_baud_rate(device.handle, 115200)?;
        classic::purge(device.handle, true, true)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<String> for BitBangerSync {
    type Error = FtError;

    fn try_from(description: String) -> Result<Self, FtError> {
        let device = Device::try_from(description)?;
        classic::set_bit_mode(device.handle, 0, BitMode::SyncBitBang)?;
        classic::set_baud_rate(device.handle, 115200)?;
        classic::purge(device.handle, true, true)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<DevInfo> for BitBangerSync {
    type Error = FtError;

    fn try_from(info: DevInfo) -> Result<Self, FtError> {
        let device = Device::try_from(info)?;
        classic::set_bit_mode(device.handle, 0, BitMode::SyncBitBang)?;
        classic::set_baud_rate(device.handle, 115200)?;
        classic::purge(device.handle, true, true)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl TryFrom<&DevInfo> for BitBangerSync {
    type Error = FtError;

    fn try_from(info: &DevInfo) -> Result<Self, FtError> {
        let device = Device::try_from(info)?;
        classic::set_bit_mode(device.handle, 0, BitMode::SyncBitBang)?;
        classic::set_baud_rate(device.handle, 115200)?;
        classic::purge(device.handle, true, true)?;
        Ok(Self {
            device: device,
            pin_dirs: [PinDir::Input; 8],
        })
    }
}

impl BitBangerSync {
    /// Will scan for devices, and attempt to connect to the first device
    /// found, if any.
    pub fn new() -> Result<Self, FtError> {
        let device = Self::try_from(0)?;
        Ok(device)
    }

    /// Returns the amount of available bytes to be read from the RXQueue.
    pub fn read_qtty(&self) -> Result<u32, FtError> {
        let bytes_to_be_read = classic::get_queue_status(self.device.handle)?;
        Ok(bytes_to_be_read)
    }

    /// Reads all GPIO pins defined as inputs, as much times as `bytes_to_read`
    /// is possible.
    ///
    /// If `bytes_to_read == 0`, it will read as much data as possible.
    ///
    /// Returns an u8 vector, where each bit corresponds to the electrical
    /// value of the pin (MSB is pin 7, LSB is pin 0).
    /// If a pin is defined as an output, the value set will be returned.
    /// The size of the output might be less (or zero), depending on the
    /// available data.
    pub fn read_all(&self, bytes_to_read: u32) -> Result<Vec<u8>, FtError> {
        let available_bytes = self.read_qtty()?;

        // If zero, read as much data as available
        let bytes_to_read = if bytes_to_read == 0 {
            available_bytes
        } else {
            bytes_to_read
        };

        // If more data than available was requested, read the current
        // electrical state of the pins and write that to the channel. That
        // way, the read queue will be filled.
        if bytes_to_read > available_bytes {
            let bytes_to_write = (bytes_to_read - available_bytes) as usize;
            let pin_values = vec![classic::get_bit_mode(self.device.handle)?; bytes_to_write];
            self.write_all(&pin_values)?;
        }

        let bytes_read = classic::read(self.device.handle, bytes_to_read)?;
        Ok(bytes_read)
    }

    /// Reads all GPIO pins defined as inputs, as much times as `bytes_to_read`
    /// is possible.
    /// The value read is only updated after the next baud rate tick.
    /// If the pin was defined as an output, the value set will be returned.
    pub fn read(&self, pin: u8, bytes_to_read: u32) -> Result<Vec<u8>, FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        let mut gpio_bytes = self.read_all(bytes_to_read)?;
        for byte in gpio_bytes.iter_mut() {
            *byte = (*byte >> pin) & 0x01;
        }
        Ok(gpio_bytes)
    }

    /// Writes all GPIO pins.
    ///
    /// It receives an 8-bit values, where each bit maps to a GPIO pin. If a
    /// pin was not set as an output, then its value will be ignored.
    pub fn write_all(&self, bytes: &Vec<u8>) -> Result<u32, FtError> {
        let bytes_written = classic::write(self.device.handle, &bytes)?;
        Ok(bytes_written)
    }

    /// Writes a GPIO pin.
    ///
    /// The value will be written after the next baud rate tick.
    /// An error will be returned if trying to write a GPIO which was set as
    /// input.
    pub fn write(&self, pin: u8, bytes: &Vec<u8>) -> Result<u32, FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }

        if self.pin_dirs[pin as usize] == PinDir::Input {
            return Err(FtError::WriteGPIOInput);
        }

        let mut pin_values = vec![classic::get_bit_mode(self.device.handle)?; bytes.len()];

        for (byte, pin_value) in std::iter::zip(bytes.iter(), pin_values.iter_mut()) {
            if *byte != 0 {
                *pin_value |= 1 << pin;
            } else {
                *pin_value &= !(1 << pin);
            }
        }

        let bytes_written = self.write_all(&pin_values)?;
        Ok(bytes_written)
    }

    /// Sets a GPIO pin as an input.
    pub fn set_input(&mut self, pin: u8) -> Result<(), FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        self.pin_dirs[pin as usize] = PinDir::Input;
        classic::set_bit_mode(self.device.handle, self.get_umask(), BitMode::SyncBitBang)?;
        Ok(())
    }

    /// Sets a GPIO pin as an output.
    pub fn set_output(&mut self, pin: u8) -> Result<(), FtError> {
        if pin > 7 {
            return Err(FtError::InvalidArgs);
        }
        self.pin_dirs[pin as usize] = PinDir::Output;
        classic::set_bit_mode(self.device.handle, self.get_umask(), BitMode::SyncBitBang)?;
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

    use core::time;
    use std::thread::sleep;

    use super::*;

    /// This test requires the eight pins from channel A and channel B to be
    /// connected between each other.
    //#[cfg(feature = "test-ft4232h")]
    #[test]
    fn test_io() -> Result<(), FtError> {
        let mut cha = BitBangerSync::try_from(0)?;
        let mut chb = BitBangerSync::try_from(1)?;

        // We will interleave inputs and outputs
        for i in 0..4 {
            cha.set_input(i)?;
            chb.set_output(i)?;
        }

        for i in 4..8 {
            cha.set_output(i)?;
            chb.set_input(i)?;
        }

        // There should be no values to be read at the start
        assert!(cha.read_qtty()? == 0);
        assert!(chb.read_qtty()? == 0);

        // Write all zeros in both devices
        let zeros: Vec<u8> = vec![0x00];
        cha.write_all(&zeros)?;
        chb.write_all(&zeros)?;
        sleep(time::Duration::from_secs(1));

        // Since we wrote 1 byte, there should be one value to be read
        // on both channels.
        // Discard it, since we can't make assumptions on the initial state.
        assert!(cha.read_all(1)?.len() == 1);
        assert!(chb.read_all(1)?.len() == 1);

        // Write a series of bytes in channel A
        let sequence: Vec<u8> = vec![0xAA, 0xBB, 0xCC];
        assert!(cha.write_all(&sequence)? == sequence.len() as u32);
        sleep(time::Duration::from_secs(1));

        assert!(cha.read_qtty()? == 3);
        let bytes_read = cha.read_all(0)?;
        assert!(bytes_read.len() == 3);

        // Since the read operations are always "one byte" behind, we expect
        // the first value to be all zeros
        assert!(bytes_read[0] == 0x00);
        assert!(bytes_read[1] == 0xA0);
        assert!(bytes_read[2] == 0xB0);

        // Let's force reading that last byte
        let bytes_read = cha.read_all(1)?;
        assert!(bytes_read[0] == 0xC0);

        // Now, let's repeat with channel B, which should see the "0x0C"
        // written from channel A.
        assert!(chb.read_qtty()? == 0);
        let sequence: Vec<u8> = vec![1, 0, 1];
        assert!(chb.write(0, &sequence)? == 3);
        assert!(chb.write(1, &sequence)? == 3);
        sleep(time::Duration::from_secs(1));

        assert!(chb.read_qtty()? == 6);

        // Read synch from channel B
        let bytes_read = chb.read_all(4)?;
        assert!(bytes_read[0] == 0xC0);
        assert!(bytes_read[1] == 0xC1);
        assert!(bytes_read[2] == 0xC0);
        assert!(bytes_read[3] == 0xC1);

        assert!(chb.read_qtty()? == 2);
        let bytes_read = chb.read(1, 3)?;
        assert!(bytes_read[0] == 0x1);
        assert!(bytes_read[1] == 0x0);
        assert!(bytes_read[2] == 0x1);

        // Check that both channel A and B have the right values
        let bytes_read = cha.read_all(1)?;
        assert!(bytes_read[0] == 0xC3);

        let bytes_read = chb.read_all(1)?;
        assert!(bytes_read[0] == 0xC3);

        Ok(())
    }
}
