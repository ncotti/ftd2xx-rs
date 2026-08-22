//! MPSSE (Multi-protocol Synchronous Serial Engine)
//!
//!

use crate::{FtError, FtHandle, classic};

/// Describes an MPSSE data shifting command.
///
/// These commands handle three pins, whose names make reference to the JTAG
/// standard, but can be used for any protocol:
/// * The `TDI` output: this is the "data out" pin.
/// * The `TMS` output: this is the "control" pin.
/// * The `TDO` input: this is the "data in" pin.

/// Writes data bytes into the `TDI` output pin.
///
/// * `handle`: FTDI device handle.
/// * `data`: Bytes to be written, between 1 and 65536.
/// * `falling_edge`: Whether each bit will be written on the clock's falling
/// or rising edge.
/// * `lsb`: Whether each byte will be written in LSB order or MSB order.
pub fn mpsse_write_bytes(
    handle: FtHandle,
    data: Vec<u8>,
    falling_edge: bool,
    lsb: bool,
) -> Result<(), FtError> {
    let op_code: u8 = match (lsb, falling_edge) {
        (false, false) => 0x10,
        (false, true) => 0x11,
        (true, false) => 0x18,
        (true, true) => 0x19,
    };

    if data.is_empty() || data.len() > 65536 {
        return Err(FtError::InvalidArgs);
    }

    let len = data.len() - 1;
    let mut cmd: Vec<u8> = vec![op_code, (len & 0xFF) as u8, ((len >> 8) & 0xFF) as u8];
    cmd.extend(data);

    classic::write(handle, &cmd)?;
    Ok(())
}

/// Writes data bits into the `TDI` output pin.
///
/// * `handle`: FTDI device handle.
/// * `data`: Bits to be written.
/// * `len`: Number of bits to be written, between 1 and 8.
/// * `falling_edge`: Whether each bit will be written on the clock's falling
/// or rising edge.
/// * `lsb`: Whether the bits in the `data` input will be written in LSB order
/// or MSB order.
pub fn mpsse_write_bits(
    handle: FtHandle,
    data: u8,
    len: u8,
    falling_edge: bool,
    lsb: bool,
) -> Result<(), FtError> {
    let op_code: u8 = match (lsb, falling_edge) {
        (false, false) => 0x12,
        (false, true) => 0x13,
        (true, false) => 0x1A,
        (true, true) => 0x1B,
    };

    if len == 0 || len > 8 {
        return Err(FtError::InvalidArgs);
    }
    let cmd: Vec<u8> = vec![op_code, len - 1, data];

    classic::write(handle, &cmd)?;
    Ok(())
}

/// Sends a command to read data bytes from the `TDO` input pin, and may
/// optionally block to read the response.
///
/// * `handle`: FTDI device handle.
/// * `len`: Bytes to be read, from 1 to 65536.
/// * `falling_edge`: Whether each bit will be written on the clock's falling
/// or rising edge.
/// * `lsb`: Whether each byte will be received in LSB order or MSB order.
/// * `blocking`: If `true`, this function will flush the Rx buffer and read
/// as many bytes as requested.
///
/// If `blocking` was set, returns up-to `len` bytes, otherwise an empty vector.
/// To read the response, this function will purge all data in the Rx buffer
/// prior to its call.
pub fn mpsse_read_bytes(
    handle: FtHandle,
    len: usize,
    falling_edge: bool,
    lsb: bool,
    blocking: bool,
) -> Result<Vec<u8>, FtError> {
    let op_code: u8 = match (lsb, falling_edge) {
        (false, false) => 0x20,
        (false, true) => 0x24,
        (true, false) => 0x28,
        (true, true) => 0x2C,
    };

    if len == 0 || len > 65536 {
        return Err(FtError::InvalidArgs);
    }

    let cmd: Vec<u8> = vec![
        op_code,
        ((len - 1) & 0xFF) as u8,
        (((len - 1) >> 8) & 0xFF) as u8,
    ];

    if blocking {
        mpsse_flush(handle)?;
        classic::purge(handle, true, false)?;
    }

    classic::write(handle, &cmd)?;

    let response: Vec<u8> = if blocking {
        mpsse_flush(handle)?;
        classic::read(handle, len)?
    } else {
        Vec::new()
    };

    Ok(response)
}

/// Sends a command to read data bits from the `TDO` input pin, and may
/// optionally block to read the response.
///
/// * `handle`: FTDI device handle.
/// * `len`: Bits to be read, from 1 to 8.
/// * `falling_edge`: Whether each bit will be written on the clock's falling
/// or rising edge.
/// * `lsb`: Whether each bit will be received in LSB order or MSB order.
/// * `blocking`: If `true`, this function will flush the Rx buffer and read
/// a single byte, which should hold all the requested bits.
///
/// If `blocking` was set, returns a single byte which holds the read bits in
/// the order specified by the `lsb` argument; otherwise, the value 0x00.
/// To read the response, this function will purge all data in the Rx buffer
/// prior to its call.
pub fn mpsse_read_bits(
    handle: FtHandle,
    len: u8,
    falling_edge: bool,
    lsb: bool,
    blocking: bool,
) -> Result<u8, FtError> {
    let op_code: u8 = match (lsb, falling_edge) {
        (false, false) => 0x22,
        (false, true) => 0x26,
        (true, false) => 0x2A,
        (true, true) => 0x2E,
    };

    if len == 0 || len > 8 {
        return Err(FtError::InvalidArgs);
    }
    let cmd: Vec<u8> = vec![op_code, len - 1];

    if blocking {
        mpsse_flush(handle)?;
        classic::purge(handle, true, false)?;
    }

    classic::write(handle, &cmd)?;

    let response: u8 = if blocking {
        mpsse_flush(handle)?;
        classic::read(handle, 1)?
            .get(0)
            .copied()
            .unwrap_or_default()
    } else {
        0
    };

    Ok(response)
}

/// Writes data bytes into the `TDI` output pin and reads data bytes from the
/// `TDO` input pin in opposite clock edges.
///
/// * `handle`: FTDI device handle.
/// * `data`: Bytes to be written, between 1 and 65536.
/// * `read_on_falling_edge`: Whether each bit will be read on the clock's
/// falling or rising edge. The write operation will be performed in the
/// opposite clock edge, in the same clock period.
/// * `lsb`: Whether each byte will be written in LSB order or MSB order.
/// * `blocking`: If `true`, this function will flush the Rx buffer and read
/// as many bytes as requested.
///
/// If `blocking` was set, returns up-to `data.len()` bytes, otherwise an
/// empty vector. To read the response, this function will purge all data
/// in the Rx buffer prior to its call.
pub fn mpsse_rw_bytes(
    handle: FtHandle,
    data: Vec<u8>,
    read_on_falling_edge: bool,
    lsb: bool,
    blocking: bool,
) -> Result<Vec<u8>, FtError> {
    let op_code: u8 = match (lsb, read_on_falling_edge) {
        (false, false) => 0x31,
        (false, true) => 0x34,
        (true, false) => 0x39,
        (true, true) => 0x3C,
    };

    if data.is_empty() || data.len() > 65536 {
        return Err(FtError::InvalidArgs);
    }

    let len = data.len();
    let mut cmd: Vec<u8> = vec![
        op_code,
        ((len - 1) & 0xFF) as u8,
        (((len - 1) >> 8) & 0xFF) as u8,
    ];
    cmd.extend(data);

    if blocking {
        mpsse_flush(handle)?;
        classic::purge(handle, true, false)?;
    }

    classic::write(handle, &cmd)?;

    let response: Vec<u8> = if blocking {
        mpsse_flush(handle)?;
        classic::read(handle, len)?
    } else {
        Vec::new()
    };

    Ok(response)
}

/// Writes data bits into the `TDI` output pin and reads data bits from the
/// `TDO` input pin in opposite clock edges.
///
/// * `handle`: FTDI device handle.
/// * `data`: Bits to be written, between 1 and 8.
/// * `read_on_falling_edge`: Whether each bit will be read on the clock's
/// falling or rising edge. The write operation will be performed in the
/// opposite clock edge, in the same clock period.
/// * `lsb`: Whether each bit will be received in LSB order or MSB order.
/// * `blocking`: If `true`, this function will flush the Rx buffer and read
/// a single byte, which should hold all the requested bits.
///
/// If `blocking` was set, returns a single byte which holds the read bits in
/// the order specified by the `lsb` argument; otherwise, the value 0x00.
/// To read the response, this function will purge all data in the Rx buffer
/// prior to its call.
pub fn mpsse_rw_bits(
    handle: FtHandle,
    data: u8,
    len: u8,
    read_on_falling_edge: bool,
    lsb: bool,
    blocking: bool,
) -> Result<u8, FtError> {
    let op_code: u8 = match (lsb, read_on_falling_edge) {
        (false, false) => 0x33,
        (false, true) => 0x36,
        (true, false) => 0x3B,
        (true, true) => 0x3E,
    };

    if len == 0 || len > 8 {
        return Err(FtError::InvalidArgs);
    }
    let cmd: Vec<u8> = vec![op_code, len - 1, data];

    if blocking {
        mpsse_flush(handle)?;
        classic::purge(handle, true, false)?;
    }

    classic::write(handle, &cmd)?;

    let response: u8 = if blocking {
        mpsse_flush(handle)?;
        classic::read(handle, 1)?
            .get(0)
            .copied()
            .unwrap_or_default()
    } else {
        0
    };

    Ok(response)
}

/// Writes control bits to the `TMS` output pin.
///
/// * `handle`: FTDI device handle.
/// * `data`: Bits to be written. Data is always written in LSB order. The MSB
/// bit sets the electrical state of the `TDI` pin, and is held static for the
/// duration of the `TMS` clocking.
/// * `len`: Number of bits to be written, between 1 and 7.
/// * `falling_edge`: Whether each bit will be written on the clock's falling
/// or rising edge.
pub fn mpsse_write_tms(
    handle: FtHandle,
    data: u8,
    len: u8,
    falling_edge: bool,
) -> Result<(), FtError> {
    let op_code: u8 = match falling_edge {
        false => 0x4A,
        true => 0x4B,
    };

    if len == 0 || len > 7 {
        return Err(FtError::InvalidArgs);
    }

    let cmd: Vec<u8> = vec![op_code, len - 1, data];

    classic::write(handle, &cmd)?;
    Ok(())
}

/// Writes control bits into the `TMS` output pin and reads data bits from the
/// `TDO` input pin.
///
/// * `handle`: FTDI device handle.
/// * `data`: Bits to be written. Data is always written and read in LSB order.
/// The MSB bit sets the electrical state of the `TDI` pin, and is held
/// static for the duration of the `TMS` clocking.
/// * `len`: Number of bits to be written, between 1 and 7.
/// * `write_on_falling_edge`: Whether each bit will be written on the clock's
/// falling or rising edge on the `TMS` pin.
/// * `read_on_falling_edge`: Whether each bit will be read on the clock's
/// falling or rising edge on the `TDO` pin.
/// * `blocking`: If `true`, this function will flush the Rx buffer and read
/// a single byte, which should hold all the requested bits.
///
/// If `blocking` was set, returns a single byte which holds the read bits in
/// LSB order; otherwise, the value 0x00.
/// To read the response, this function will purge all data in the Rx buffer
/// prior to its call.
pub fn mpsse_rw_tms(
    handle: FtHandle,
    data: u8,
    len: u8,
    write_on_falling_edge: bool,
    read_on_falling_edge: bool,
    blocking: bool,
) -> Result<u8, FtError> {
    let op_code: u8 = match (read_on_falling_edge, write_on_falling_edge) {
        (false, false) => 0x6A,
        (false, true) => 0x6B,
        (true, false) => 0x6E,
        (true, true) => 0x6F,
    };

    if len == 0 || len > 7 {
        return Err(FtError::InvalidArgs);
    }

    let cmd: Vec<u8> = vec![op_code, len - 1, data];

    if blocking {
        mpsse_flush(handle)?;
        classic::purge(handle, true, false)?;
    }

    classic::write(handle, &cmd)?;

    let response: u8 = if blocking {
        mpsse_flush(handle)?;
        classic::read(handle, 1)?
            .get(0)
            .copied()
            .unwrap_or_default()
    } else {
        0
    };

    Ok(response)
}

/// Sets the initial value and direction of the MPSSE pins.
///
/// * `handle`: FT device handle.
/// * `value`: Electrical value set for output pins.
/// * `direction`: For each bit, a `1` corresponds to an output.
///
/// The pinout of these devices is as follows:
///
/// ```txt
/// 15 | GPIOH7   GPIOH6   GPIOH5   GPIOH4   GPIOH3   GPIOH2   GPIOH1   GPIOH0   | 8
/// 7  | GPIOL3   GPIOL2   GPIOL1   GPIOL0   TMS(out) TDO(in)  TDI(out) TCK(out) | 0
/// ```
///
/// Some devices may only have 8 pins instead of 15. The direction of the
/// TCK, TDI, TDO and TMS pins is forced by this function, regardless of the
/// value of the four LSB bits in `direction`.
///
/// The initial value of TCK dictates the order of all other operations that
/// depend on the rising or falling edge of the clock:
///
/// * If `TCK == 0`, then a clock cycle entails a rising edge at time "t = 0",
/// followed by a falling edge at "t = T / 2".
/// * If `TCK == 1`, then a clock cycle entails a falling edge at time "t = 0",
/// followed by a rising edge at "t = T / 2".
pub fn mpsse_write_pins(handle: FtHandle, value: u16, direction: u16) -> Result<(), FtError> {
    let direction = direction | 0b1011;
    let direction = direction & (!(1 << 2));

    let cmd_low: Vec<u8> = vec![0x80, (value & 0xFF) as u8, (direction & 0xFF) as u8];
    let cmd_high: Vec<u8> = vec![
        0x82,
        ((value >> 8) & 0xFF) as u8,
        ((direction >> 8) & 0xFF) as u8,
    ];

    classic::write(handle, &cmd_low)?;
    classic::write(handle, &cmd_high)?;
    Ok(())
}

/// Reads the electrical state of the MPSSE interface pins.
///
/// * `handle`: FT device handle
/// * `blocking`: If `true`, this function will flush the Rx buffer and read
/// the electrical state of the pins.
///
/// If blocking, returns the electrical state of the possible 16 pins
/// in the MPSSE interface. Otherwise, just "0x0000" is returned.
pub fn mpsse_read_pins(handle: FtHandle, blocking: bool) -> Result<u16, FtError> {
    let cmd_read_low: Vec<u8> = vec![0x81];
    let cmd_read_high: Vec<u8> = vec![0x83];

    if blocking {
        mpsse_flush(handle)?;
        classic::purge(handle, true, false)?;
    }

    classic::write(handle, &cmd_read_low)?;
    classic::write(handle, &cmd_read_high)?;

    let response: u16 = if blocking {
        mpsse_flush(handle)?;
        let response_vec = classic::read(handle, 2)?;
        (((response_vec.get(1).copied().unwrap_or_default() & 0xFF) as u16) << 8)
            | ((response_vec.get(0).copied().unwrap_or_default() & 0xFF) as u16)
    } else {
        0
    };

    Ok(response)
}

/// Internally connects the TDI and TDO pins.
///
/// * `handle`: FT device handle.
/// * `enable`: Whether to connect the TDI and TDO pins, or not.
pub fn mpsse_loopback(handle: FtHandle, enable: bool) -> Result<(), FtError> {
    let op_code: Vec<u8> = if enable { vec![0x84] } else { vec![0x85] };
    classic::write(handle, &op_code)?;
    Ok(())
}

/// Sets the clock frequency for the `TCK` pin.
///
/// * `handle`: FT device handle.
/// * `freq`: Frequency in Hz, between 91.553 Hz and 30 MHz.
///
/// The equation for calculating the frequency can be either of the following:
///
/// $freq = 12MHz / (1 + X * 2)$
/// $freq = 60MHz / (1 + X * 2)$
///
/// Where X is an u16 value.
///
/// This function will evaluate the two functions, see which one yields the
/// less error, and set the frequency accordingly.
///
/// Returns the actual frequency set.
pub fn mpsse_set_clock_frequency(handle: FtHandle, freq: f32) -> Result<f32, FtError> {
    let freq: f32 = if freq <= 0.0 { 1.0 } else { freq };

    let x1: u16 = (((12e6 / freq) - 1.0) / 2.0).round() as u16;
    let x2: u16 = (((60e6 / freq) - 1.0) / 2.0).round() as u16;

    let actual_freq1: f32 = 12e6 / (1.0 + (x1 as f32) * 2.0);
    let actual_freq2: f32 = 60e6 / (1.0 + (x2 as f32) * 2.0);

    let freq_error1: f32 = (actual_freq1 - freq).abs();
    let freq_error2: f32 = (actual_freq2 - freq).abs();

    let (f, x, clock_divider) = if freq_error1 < freq_error2 {
        (actual_freq1, x1, 0x8B as u8)
    } else {
        (actual_freq2, x2, 0x8A)
    };

    let clock_divider: Vec<u8> = vec![clock_divider];
    classic::write(handle, &clock_divider)?;

    let freq_cmd: Vec<u8> = vec![0x86, (x & 0xFF) as u8, ((x >> 8) & 0xFF) as u8];
    classic::write(handle, &freq_cmd)?;

    Ok(f)
}

/// Flushes the Rx queue buffer back to the PC.
pub fn mpsse_flush(handle: FtHandle) -> Result<(), FtError> {
    let command: Vec<u8> = vec![0x87];
    classic::write(handle, &command)?;
    Ok(())
}

/// Causes the MPSSE controller to wait until GPIOL1, i.e., pin 5, is either
/// high or low before processing the next instruction.
///
/// * `handle`: FT device handle.
/// * `on_high`: Whether to wait until the GPIOL1 is high, or low.
pub fn mpsse_wait(handle: FtHandle, on_high: bool) -> Result<(), FtError> {
    let cmd: Vec<u8> = match on_high {
        true => {
            vec![0x88]
        }
        false => {
            vec![0x89]
        }
    };
    classic::write(handle, &cmd)?;
    Ok(())
}

/// If enabled, the data will be valid on both clock edges, at the cost of
/// slower maximum speeds. (TODO, check).
///
/// * `handle`: FT device handle.
/// * `enable`: Enable or disable 3 phase data clocking.
pub fn mpsse_3_phase_data_clocking(handle: FtHandle, enable: bool) -> Result<(), FtError> {
    let cmd: Vec<u8> = match enable {
        true => {
            vec![0x8C]
        }
        false => {
            vec![0x8D]
        }
    };
    classic::write(handle, &cmd)?;
    Ok(())
}

/// Toggles the clock signal TCK `len` times.
///
/// * `handle`: FT device handle.
/// * `len`: Number of clock cycles to toggle, from 1 to 524288.
pub fn mpsse_write_clk(handle: FtHandle, len: u32) -> Result<(), FtError> {
    if len == 0 || len > 524288 {
        return Err(FtError::InvalidArgs);
    }

    if len >= 8 {
        let cmd: Vec<u8> = vec![
            0x8F,
            ((len - 1 / 8) & 0xFF) as u8,
            (((len - 1 / 8) >> 8) & 0xFF) as u8,
        ];
        classic::write(handle, &cmd)?;
    }

    if len % 8 != 0 {
        let cmd: Vec<u8> = vec![0x8E, ((len - 1) % 8) as u8];
        classic::write(handle, &cmd)?;
    }

    Ok(())
}

/// This function toggles the TMS clock pin at most `len` times, or until
/// the `GPIOL1` pin is set to either high or low.
///
/// * `handle`: FT device handle.
/// * `len`: Maximum number of clock cycles to toggle, from 0 to 524288.
/// This function will wait for at most `len` amount of clock cycles
/// (or infinitely if `len == 0`), or until GPIOL1 is either high, or low,
/// depending on the `wait_on_high` argument.
/// * `wait_on_high`: The clock toggling will stop until the GPIOL1 pin is
/// either high or low.
pub fn mpsse_write_clk_timeout(
    handle: FtHandle,
    len: u32,
    wait_on_high: bool,
) -> Result<(), FtError> {
    let cmd: Vec<u8> = if len == 0 || len > 524288 {
        if wait_on_high { vec![0x94] } else { vec![0x95] }
    } else if wait_on_high {
        vec![
            0x9C,
            (((len - 1) / 8) & 0xFF) as u8,
            ((((len - 1) / 8) >> 8) & 0xFF) as u8,
        ]
    } else {
        vec![
            0x9D,
            (((len - 1) / 8) & 0xFF) as u8,
            ((((len - 1) / 8) >> 8) & 0xFF) as u8,
        ]
    };

    classic::write(handle, &cmd)?;
    Ok(())
}

/// Enable or disable adaptative clocking (TODO)
pub fn mpsse_adaptive_clocking(handle: FtHandle, enable: bool) -> Result<(), FtError> {
    let cmd: Vec<u8> = if enable { vec![0x96] } else { vec![0x97] };

    classic::write(handle, &cmd)?;
    Ok(())
}
