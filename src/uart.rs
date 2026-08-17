//! FTDI device as UART

use crate::BitMode;
use crate::BitsPerWord;
use crate::Device;
use crate::FlowControl;
use crate::FtError;
use crate::Parity;
use crate::StopBits;
use crate::classic;

/// FTDI device used as UART
pub struct Uart {
    device: Device,
    bits_per_word: BitsPerWord,
    parity: Parity,
    stop_bits: StopBits,
    read_timeout_ms: u32,
    write_timeout_ms: u32,
}

impl TryFrom<u32> for Uart {
    type Error = FtError;

    fn try_from(value: u32) -> Result<Self, FtError> {
        let device = Device::try_from(value)?;
        classic::set_bit_mode(device.handle, 0, BitMode::Reset)?;
        classic::set_baud_rate(device.handle, 115200)?;
        classic::set_data_characteristics(
            device.handle,
            BitsPerWord::Bits8,
            StopBits::StopBits1,
            Parity::None,
        )?;
        classic::clr_dtr(device.handle)?;
        classic::clr_rts(device.handle)?;
        classic::set_flow_control(device.handle, FlowControl::None, 0, 0)?;
        classic::set_timeouts(device.handle, 1000, 1000)?;
        classic::purge(device.handle, true, true)?;
        Ok(Self {
            device: device,
            bits_per_word: BitsPerWord::Bits8,
            parity: Parity::None,
            stop_bits: StopBits::StopBits1,
            read_timeout_ms: 1000,
            write_timeout_ms: 1000,
        })
    }
}

impl Uart {
    /// Returns the amount of bytes in the Rx queue, i.e., that have been
    /// received and can be read without waiting.
    pub fn len(&self) -> Result<usize, FtError> {
        let read_bytes = classic::get_queue_status(self.device.handle)?;
        Ok(read_bytes)
    }

    /// Reads bytes from the UART.
    ///
    /// * `size`: number of bytes to read.
    ///     * If `size == 0`, all available bytes will be read
    ///     * If `0 < size <= Bytes in Rx queue`, then  `size` bytes will be
    ///     * read instantly.
    ///     * If `Bytes in Rx queue > size`, then this function will block for
    ///     at most `read_timeout_ms` milliseconds, and then return the
    ///     available amount of bytes, which could be less than specified.
    ///
    /// Returns the bytes read.
    pub fn read(&self, size: usize) -> Result<Vec<u8>, FtError> {
        let size = if size == 0 { self.len()? } else { size };

        let data = classic::read(self.device.handle, size)?;
        Ok(data)
    }

    /// Writes bytes to UART
    pub fn write(&self, data: &Vec<u8>) -> Result<usize, FtError> {
        let bytes_written = classic::write(self.device.handle, data)?;
        Ok(bytes_written)
    }

    /// Sets the UART baud rate.
    pub fn set_baud_rate(&self, baud_rate: u32) -> Result<(), FtError> {
        classic::set_baud_rate(self.device.handle, baud_rate)?;
        Ok(())
    }

    /// Sets the read timeout, in milliseconds.
    pub fn set_read_timeout(&mut self, timeout: u32) -> Result<(), FtError> {
        self.read_timeout_ms = timeout;
        classic::set_timeouts(
            self.device.handle,
            self.read_timeout_ms,
            self.write_timeout_ms,
        )?;
        Ok(())
    }

    /// Sets the write timeout, in milliseconds.
    pub fn set_write_timeout(&mut self, timeout: u32) -> Result<(), FtError> {
        self.write_timeout_ms = timeout;
        classic::set_timeouts(
            self.device.handle,
            self.read_timeout_ms,
            self.write_timeout_ms,
        )?;
        Ok(())
    }

    /// Sets the type of parity bits per transaction.
    pub fn set_parity(&mut self, parity: Parity) -> Result<(), FtError> {
        self.parity = parity;
        classic::set_data_characteristics(
            self.device.handle,
            self.bits_per_word,
            self.stop_bits,
            self.parity,
        )?;
        Ok(())
    }

    /// Sets the number of data bits per transaction.
    pub fn set_bits_per_word(&mut self, bits_per_word: BitsPerWord) -> Result<(), FtError> {
        self.bits_per_word = bits_per_word;
        classic::set_data_characteristics(
            self.device.handle,
            self.bits_per_word,
            self.stop_bits,
            self.parity,
        )?;
        Ok(())
    }

    /// Sets the number of stop bits per transaction.
    pub fn set_stop_bits(&mut self, stop_bits: StopBits) -> Result<(), FtError> {
        self.stop_bits = stop_bits;
        classic::set_data_characteristics(
            self.device.handle,
            self.bits_per_word,
            self.stop_bits,
            self.parity,
        )?;
        Ok(())
    }

    /// Enables XON XOFF flow control.
    /// Standard values are XON=0x11; XOFF=0x13.
    pub fn enable_xon_xoff(&mut self, xon_char: u8, xoff_char: u8) -> Result<(), FtError> {
        classic::set_flow_control(
            self.device.handle,
            FlowControl::XonXoff,
            xon_char,
            xoff_char,
        )?;
        Ok(())
    }

    /// Enables RTS CTS flow control.
    pub fn enable_rts_cts(&self) -> Result<(), FtError> {
        classic::set_flow_control(self.device.handle, FlowControl::RtsCts, 0, 0)?;
        Ok(())
    }

    /// Enables DTR DSR flow control.
    pub fn enable_dtr_dsr(&self) -> Result<(), FtError> {
        classic::set_flow_control(self.device.handle, FlowControl::DtrDsr, 0, 0)?;
        Ok(())
    }

    /// Disables any flow control method.
    pub fn disable_flow_control(&self) -> Result<(), FtError> {
        classic::set_flow_control(self.device.handle, FlowControl::None, 0, 0)?;
        Ok(())
    }

    /// TODO: this function would wait until "CTS" is asserted, and then write
    /// The same with the XON_XOFF or DTR_DSR, not implemented yet
    pub fn rts_cts(&self) -> Result<(), FtError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::thread::sleep;

    use super::*;

    /// This test requires that all pins from channel C and D to be
    /// connected as such:
    /// Channel C (Master)  Channel D (Slave)
    /// (0)TXD      ->      (1)RXD
    /// (1)RXD      <-      (0)TXD
    /// (2)RTS#     ->      (3)CTS#
    /// (3)CTS#     <-      (2)RTS#
    /// (4)DTR#     ->      (5)DSR#
    /// (5)DSR#     <-      (4)DTR#
    /// (6)DCD#     <-      (6)DCD#
    /// (7)RI#      <-      (7)RI#
    #[test]
    fn test_uart_read_write() -> Result<(), FtError> {
        let uart_a = Uart::try_from(2)?;
        let uart_b = Uart::try_from(3)?;

        assert!(uart_a.len()? == 0);
        assert!(uart_b.len()? == 0);

        let sequence: Vec<u8> = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        assert!(uart_a.write(&sequence)? == 6);

        sleep(time::Duration::from_millis(1000));

        assert!(uart_b.len()? == 6);
        let response = uart_b.read(0)?;
        assert!(response.len() == 6);
        assert!(sequence == response);
        Ok(())
    }
}
