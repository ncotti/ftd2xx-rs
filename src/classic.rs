//! Implements safe Rust wrappers around all D2XX classic functions as
//! described in section 3 of the D2XX Programmer's Guide.

use crate::{
    FtHandle,
    types::{DevInfo, FtError, Version, ft_try},
    utils::string_to_i8_array,
};

use crate::eeprom::Eeprom;

use ftd2xx_sys::d2xx::{
    self, FT_BITMODE_ASYNC_BITBANG, FT_BITMODE_CBUS_BITBANG, FT_BITMODE_FAST_SERIAL,
    FT_BITMODE_MCU_HOST, FT_BITMODE_MPSSE, FT_BITMODE_RESET, FT_BITMODE_SYNC_BITBANG,
    FT_BITMODE_SYNC_FIFO,
};

use std::ffi::c_void;

use crate::utils;

/// Describes an UART interface
pub struct UartInfo {
    /// Parity bit
    pub parity: Parity,
    /// DAta bits
    pub bits: BitsPerWord,
    /// Stop bits
    pub stop_bits: StopBits,
}

/// Parity bit
#[repr(u8)]
#[allow(missing_docs)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4,
}

/// Bits per word in UART transaction
#[repr(u8)]
pub enum BitsPerWord {
    /// Seven bits
    Bits7 = 7,
    /// Eight bits
    Bits8 = 8,
}

/// Number of stop bits per UART transaction
#[repr(u8)]
pub enum StopBits {
    /// One stop bit
    StopBits1 = 0,
    /// Two stop bits
    StopBits2 = 2,
}

#[repr(u16)]
#[allow(missing_docs)]
pub enum FlowControl {
    None = d2xx::FT_FLOW_NONE as u16,
    RtsCts = d2xx::FT_FLOW_RTS_CTS as u16,
    DtrDsr = d2xx::FT_FLOW_DTR_DSR as u16,
    XonXoff = d2xx::FT_FLOW_XON_XOFF as u16,
}

#[allow(missing_docs)]
pub struct EventCause {
    pub rx_char: bool,
    pub modem_status: bool,
    pub line_status: bool,
}

#[repr(u8)]
#[allow(missing_docs)]
pub enum BitMode {
    Reset = FT_BITMODE_RESET as u8,
    AsyncBitBang = FT_BITMODE_ASYNC_BITBANG as u8,
    MPSSE = FT_BITMODE_MPSSE as u8,
    SyncBitBang = FT_BITMODE_SYNC_BITBANG as u8,
    MCUHostBusEmulation = FT_BITMODE_MCU_HOST as u8,
    FastOptoIsolatedSerial = FT_BITMODE_FAST_SERIAL as u8,
    CBUSBitBang = FT_BITMODE_CBUS_BITBANG as u8,
    SingleChannelSync245FIFOMode = FT_BITMODE_SYNC_FIFO as u8,
}

/// 3.1 FT_SetVIDPID
#[cfg(not(target_os = "windows"))]
pub fn set_vid_pid(vid: u16, pid: u16) -> Result<(), FtError> {
    let vid: u32 = vid.into();
    let pid: u32 = pid.into();
    ft_try!(d2xx::FT_SetVIDPID(vid, pid));
    Ok(())
}

/// 3.2 FT_GetVIDPID
#[cfg(not(target_os = "windows"))]
pub fn get_vid_pid() -> Result<(u16, u16), FtError> {
    let mut vid: u32 = 0;
    let mut pid: u32 = 0;
    ft_try!(d2xx::FT_GetVIDPID(&mut vid, &mut pid));
    let vid: u16 = vid as u16;
    let pid: u16 = pid as u16;
    Ok((vid, pid))
}
/// 3.3 FT_CreateDeviceInfoList
pub fn create_device_info_list() -> Result<u32, FtError> {
    let mut number_of_devices: u32 = 0;
    ft_try!(d2xx::FT_CreateDeviceInfoList(&mut number_of_devices));
    Ok(number_of_devices)
}

/// 3.4 FT_GetDeviceInfoList
pub fn get_device_info_list(number_of_devices: u32) -> Result<Vec<DevInfo>, FtError> {
    if number_of_devices == 0 {
        return Err(FtError::DeviceNotFound);
    }

    let mut number_of_devices: u32 = number_of_devices;
    let mut ft_devices_info: Vec<d2xx::FT_DEVICE_LIST_INFO_NODE> = vec![
        d2xx::FT_DEVICE_LIST_INFO_NODE {
            Flags: 0,
            Type: 0,
            ID: 0,
            LocId: 0,
            SerialNumber: [0; 16],
            Description: [0; 64],
            ftHandle: std::ptr::null_mut(),
        };
        usize::try_from(number_of_devices)
            .unwrap()
    ];

    ft_try!(d2xx::FT_GetDeviceInfoList(
        ft_devices_info.as_mut_ptr(),
        &mut number_of_devices
    ));

    let mut devices_info: Vec<DevInfo> = Vec::new();

    for dev in ft_devices_info {
        devices_info.push(DevInfo::from(dev));
    }

    Ok(devices_info)
}

/// 3.5 FT_GetDeviceInfoDetail
pub fn get_device_info_detail(device_index: u32) -> Result<DevInfo, FtError> {
    let mut flags: u32 = 0;
    let mut ft_type: u32 = 0;
    let mut id: u32 = 0;
    let mut locid: u32 = 0;
    let mut serial_number: [i8; 16] = [0; 16];
    let mut description: [i8; 64] = [0; 64];
    let mut ft_handle: FtHandle = std::ptr::null_mut();

    let p_serial_number: *mut c_void = serial_number.as_mut_ptr().cast();
    let p_description: *mut c_void = description.as_mut_ptr().cast();

    ft_try!(d2xx::FT_GetDeviceInfoDetail(
        device_index,
        &mut flags,
        &mut ft_type,
        &mut id,
        &mut locid,
        p_serial_number,
        p_description,
        &mut ft_handle
    ));

    let ft_device_info = d2xx::FT_DEVICE_LIST_INFO_NODE {
        Flags: flags,
        Type: ft_type,
        ID: id,
        LocId: locid,
        SerialNumber: serial_number,
        Description: description,
        ftHandle: ft_handle,
    };

    let device_info = DevInfo::from(ft_device_info);

    Ok(device_info)
}

// 3.6 FT_ListDevices is not implemented, use 3.4 or 3.5 instead.

/// 3.7 FT_Open
pub fn open(device_index: u32) -> Result<FtHandle, FtError> {
    let mut ft_handle: FtHandle = std::ptr::null_mut();
    ft_try!(d2xx::FT_Open(device_index as i32, &mut ft_handle));
    Ok(ft_handle)
}

/// 3.8 FT_OpenEx, with flag FT_OPEN_BY_SERIAL_NUMBER
pub fn open_ex_by_serial_number(serial_number: [i8; 16]) -> Result<FtHandle, FtError> {
    let mut ft_handle: FtHandle = std::ptr::null_mut();
    let mut serial_number = serial_number;
    let p_serial_number: *mut c_void = serial_number.as_mut_ptr().cast();
    ft_try!(d2xx::FT_OpenEx(
        p_serial_number,
        d2xx::FT_OPEN_BY_SERIAL_NUMBER,
        &mut ft_handle
    ));
    Ok(ft_handle)
}

/// 3.8 FT_OpenEx, with flag FT_OPEN_BY_DESCRIPTION
pub fn open_ex_by_description(description: &str) -> Result<FtHandle, FtError> {
    let mut ft_handle: FtHandle = std::ptr::null_mut();
    let p_description: *mut c_void = description.as_ptr() as *mut c_void;
    ft_try!(d2xx::FT_OpenEx(
        p_description,
        d2xx::FT_OPEN_BY_DESCRIPTION,
        &mut ft_handle
    ));
    Ok(ft_handle)
}

/// 3.8 FT_OpenEx, with flag FT_OPEN_BY_LOCATION
pub fn open_ex_by_location(location_id: u32) -> Result<FtHandle, FtError> {
    let mut ft_handle: FtHandle = std::ptr::null_mut();
    let location_id = location_id;
    ft_try!(d2xx::FT_OpenEx(
        location_id as *mut c_void,
        d2xx::FT_OPEN_BY_LOCATION,
        &mut ft_handle
    ));
    Ok(ft_handle)
}

/// 3.9 FT_Close
pub fn close(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_Close(ft_handle));
    Ok(())
}

/// 3.10 FT_Read
pub fn read(ft_handle: FtHandle, bytes_to_read: u32) -> Result<Vec<u8>, FtError> {
    let mut bytes_read: u32 = 0;
    let mut bytes: Vec<u8> = Vec::new();
    bytes.reserve_exact(bytes_to_read as usize);
    ft_try!(d2xx::FT_Read(
        ft_handle,
        bytes.as_mut_ptr().cast(),
        bytes_to_read,
        &mut bytes_read
    ));
    unsafe { bytes.set_len(bytes_read as usize) };
    Ok(bytes)
}

/// 3.11 FT_Write
pub fn write(ft_handle: FtHandle, data: &Vec<u8>) -> Result<u32, FtError> {
    let mut bytes_written: u32 = 0;
    let bytes_to_be_written: u32 = data.len() as u32;
    let mut bytes = data.clone();
    ft_try!(d2xx::FT_Write(
        ft_handle,
        bytes.as_mut_ptr().cast(),
        bytes_to_be_written,
        &mut bytes_written
    ));
    Ok(bytes_written)
}

/// 3.12 FT_SetBaudRate
pub fn set_baud_rate(ft_handle: FtHandle, baud_rate: u32) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetBaudRate(ft_handle, baud_rate));
    Ok(())
}

/// 3.13 FT_SetDivisor
pub fn set_divisor(ft_handle: FtHandle, divisor: u16) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetDivisor(ft_handle, divisor));
    Ok(())
}

/// 3.14 FT_SetDataCharacteristics
pub fn set_data_characteristics(ft_handle: FtHandle, uart_info: UartInfo) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetDataCharacteristics(
        ft_handle,
        uart_info.bits as u8,
        uart_info.stop_bits as u8,
        uart_info.parity as u8
    ));
    Ok(())
}

/// 3.15 FT_SetTimeouts
pub fn set_timeouts(
    ft_handle: FtHandle,
    read_timeout: u32,
    write_timeout: u32,
) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetTimeouts(ft_handle, read_timeout, write_timeout));
    Ok(())
}

/// 3.16 FT_SetFlowControl
pub fn set_flow_control(
    ft_handle: FtHandle,
    flow_control: FlowControl,
    xon_char: u8,
    xoff_char: u8,
) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetFlowControl(
        ft_handle,
        flow_control as u16,
        xon_char,
        xoff_char
    ));
    Ok(())
}

/// 3.17 FT_SetDtr
pub fn set_dtr(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetDtr(ft_handle));
    Ok(())
}

/// 3.18 FT_ClrDtr
pub fn clr_dtr(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_ClrDtr(ft_handle));
    Ok(())
}

/// 3.19 FT_SetRts
pub fn set_rts(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetRts(ft_handle));
    Ok(())
}

/// 3.20 FT_ClrRts
pub fn clr_rts(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_ClrRts(ft_handle));
    Ok(())
}

/// 3.21 FT_GetModemStatus
pub fn get_modem_status(ft_handle: FtHandle) -> Result<u32, FtError> {
    let mut modem_status: u32 = 0;
    ft_try!(d2xx::FT_GetModemStatus(ft_handle, &mut modem_status));
    Ok(modem_status)
}

// Note, 3.22 and 3.23 are not supported. Use 3.21 instead.

/// 3.24 FT_GetQueueStatus
pub fn get_queue_status(ft_handle: FtHandle) -> Result<u32, FtError> {
    let mut bytes_in_rx_queue: u32 = 0;
    ft_try!(d2xx::FT_GetQueueStatus(ft_handle, &mut bytes_in_rx_queue));
    Ok(bytes_in_rx_queue)
}

/// 3.25 FT_GetDeviceInfo
pub fn get_device_info(ft_handle: FtHandle) -> Result<DevInfo, FtError> {
    let flags: u32 = 0;
    let mut ft_type: u32 = 0;
    let mut id: u32 = 0;
    let locid: u32 = 0;
    let mut serial_number: [i8; 16] = [0; 16];
    let mut description: [i8; 64] = [0; 64];
    let dummy_void_ptr: FtHandle = std::ptr::null_mut();

    ft_try!(d2xx::FT_GetDeviceInfo(
        ft_handle,
        &mut ft_type,
        &mut id,
        serial_number.as_mut_ptr(),
        description.as_mut_ptr(),
        dummy_void_ptr
    ));

    let ft_device_info = d2xx::FT_DEVICE_LIST_INFO_NODE {
        Flags: flags,
        Type: ft_type,
        ID: id,
        LocId: locid,
        SerialNumber: serial_number,
        Description: description,
        ftHandle: ft_handle,
    };

    let device_info = DevInfo::from(ft_device_info);
    Ok(device_info)
}

/// 3.26 FT_GetDriverVersion
pub fn get_driver_version(ft_handle: FtHandle) -> Result<Version, FtError> {
    let mut version: u32 = 0;
    ft_try!(d2xx::FT_GetDriverVersion(ft_handle, &mut version));
    Ok(Version::new(version))
}

/// 3.27 FT_GetLibraryVersion
pub fn get_library_version() -> Result<Version, FtError> {
    let mut version: u32 = 0;
    ft_try!(d2xx::FT_GetLibraryVersion(&mut version));
    Ok(Version::new(version))
}

/// 3.28 FT_GetComPortNumber
#[cfg(target_os = "windows")]
pub fn get_com_port_number(ft_handle: FtHandle) -> Result<u32, FtError> {
    let mut com_port: u32 = 0;
    ft_try!(d2xx::FT_GetComPortNumber(ft_handle, &mut com_port));
    Ok(com_port)
}

/// 3.29 FT_GetStatus
pub fn get_status(ft_handle: FtHandle) -> Result<(u32, u32, u32), FtError> {
    let mut bytes_in_rx_queue: u32 = 0;
    let mut bytes_in_tx_queue: u32 = 0;
    let mut status: u32 = 0;
    ft_try!(d2xx::FT_GetStatus(
        ft_handle,
        &mut bytes_in_rx_queue,
        &mut bytes_in_tx_queue,
        &mut status
    ));
    Ok((bytes_in_rx_queue, bytes_in_tx_queue, status))
}

/// 3.30 FT_SetEventNotification
pub fn set_event_notification(
    ft_handle: FtHandle,
    event_cause: EventCause,
    event_handle: &mut d2xx::EVENT_HANDLE,
) -> Result<(), FtError> {
    let event_cause = (event_cause.rx_char as u32) << 0
        & (event_cause.modem_status as u32) << 1
        & (event_cause.line_status as u32) << 2;

    ft_try!(d2xx::FT_SetEventNotification(
        ft_handle,
        event_cause,
        (event_handle as *mut d2xx::EVENT_HANDLE).cast()
    ));
    Ok(())
}

/// 3.31 FT_SetChars
pub fn set_chars(
    ft_handle: FtHandle,
    event_char: u8,
    error_char: u8,
    event_en: bool,
    error_en: bool,
) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetChars(
        ft_handle,
        event_char,
        event_en as u8,
        error_char,
        error_en as u8
    ));
    Ok(())
}

/// 3.32 FT_SetBreakOn
pub fn set_break_on(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetBreakOn(ft_handle));
    Ok(())
}

/// 3.33 FT_SetBreakOff
pub fn set_break_off(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetBreakOff(ft_handle));
    Ok(())
}

/// 3.34 FT_Purge
pub fn purge(ft_handle: FtHandle, purge_rx: bool, purge_tx: bool) -> Result<(), FtError> {
    let purge_mask: u32 = ((purge_tx as u32) << 1) | ((purge_rx as u32) << 1);
    ft_try!(d2xx::FT_Purge(ft_handle, purge_mask));
    Ok(())
}

/// 3.35 FT_ResetDevice
pub fn reset_device(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_ResetDevice(ft_handle));
    Ok(())
}

/// 3.36 FT_ResetPort
#[cfg(target_os = "windows")]
pub fn reset_port(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_ResetPort(ft_handle));
    Ok(())
}

/// 3.37 FT_CyclePort
#[cfg(target_os = "windows")]
pub fn cycle_port(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_CyclePort(ft_handle));
    Ok(())
}

/// 3.38 FT_Rescan
#[cfg(target_os = "windows")]
pub fn rescan() -> Result<(), FtError> {
    ft_try!(d2xx::FT_Rescan());
    Ok(())
}

/// 3.39 FT_Reload
#[cfg(target_os = "windows")]
pub fn reload(vid: u16, pid: u16) -> Result<(), FtError> {
    ft_try!(d2xx::FT_Reload(vid, pid));
    Ok(())
}

/// 3.40 FT_SetResetPipeRetryCount
#[cfg(target_os = "windows")]
pub fn set_reset_pipe_retry_count(ft_handle: FtHandle, retry_count: u32) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetResetPipeRetryCount(ft_handle, retry_count));
    Ok(())
}

/// 3.41 FT_StopInTask
pub fn stop_in_task(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_StopInTask(ft_handle));
    Ok(())
}

/// 3.42 FT_RestartInTask
pub fn restart_in_task(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_RestartInTask(ft_handle));
    Ok(())
}

/// 3.43 FT_RestartInTask
pub fn set_deadman_timeout(ft_handle: FtHandle, timeout: u32) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetDeadmanTimeout(ft_handle, timeout));
    Ok(())
}

/// 4.1 FT_ReadEE
pub fn read_ee(ft_handle: FtHandle, offset: u32) -> Result<u16, FtError> {
    let mut value: u16 = 0;
    ft_try!(d2xx::FT_ReadEE(ft_handle, offset, &mut value));
    Ok(value)
}

/// 4.2 FT_WriteEE
pub fn write_ee(ft_handle: FtHandle, offset: u32, value: u16) -> Result<(), FtError> {
    ft_try!(d2xx::FT_WriteEE(ft_handle, offset, value));
    Ok(())
}

/// 4.3 FT_EraseEE
pub fn erase_ee(ft_handle: FtHandle) -> Result<(), FtError> {
    ft_try!(d2xx::FT_EraseEE(ft_handle));
    Ok(())
}

// 4.4 FT_EE_Read, 4.5 FT_EE_Read, 4.6 FT_EE_Program and 4.7 FT_EE_ProgramEx
// are not implemented. Use 4.11 FT_EEPROM_Read and 4.12 FT_EEPROM_Program
// instead.

/// 4.8 FT_EE_UASize
pub fn ee_ua_size(ft_handle: FtHandle) -> Result<usize, FtError> {
    let mut size: u32 = 0;
    ft_try!(d2xx::FT_EE_UASize(ft_handle, &mut size));
    Ok(size as usize)
}

/// 4.9 FT_EE_UARead
pub fn ee_ua_read(ft_handle: FtHandle) -> Result<Vec<u8>, FtError> {
    let mut bytes_read: u32 = 0;
    let mut bytes: Vec<u8> = Vec::new();
    bytes.reserve_exact(1024);
    ft_try!(d2xx::FT_EE_UARead(
        ft_handle,
        bytes.as_mut_ptr(),
        1024,
        &mut bytes_read
    ));
    unsafe { bytes.set_len(bytes_read as usize) };
    Ok(bytes)
}

/// 4.10 FT_EE_UAWrite
pub fn ee_ua_write(ft_handle: FtHandle, bytes: &Vec<u8>) -> Result<(), FtError> {
    let data_len: u32 = bytes.len() as u32;
    let mut bytes = bytes.clone();
    ft_try!(d2xx::FT_EE_UAWrite(ft_handle, bytes.as_mut_ptr(), data_len));
    Ok(())
}

/// 4.11 FT_EEPROM_Read
pub fn eeprom_read<T: Eeprom>(ft_handle: FtHandle) -> Result<T, FtError> {
    let mut manufacturer: [i8; 64] = [0; 64];
    let mut manufacturer_id: [i8; 64] = [0; 64];
    let mut description: [i8; 64] = [0; 64];
    let mut serial_number: [i8; 64] = [0; 64];

    let p_manufacturer: *mut i8 = manufacturer.as_mut_ptr();
    let p_manufacturer_id: *mut i8 = manufacturer_id.as_mut_ptr();
    let p_description: *mut i8 = description.as_mut_ptr();
    let p_serial_number: *mut i8 = serial_number.as_mut_ptr();

    let mut ft_eeprom: T::FtEeprom = T::default().into();
    let p_ft_eeprom: *mut c_void = (&mut ft_eeprom as *mut T::FtEeprom).cast();

    ft_try!(d2xx::FT_EEPROM_Read(
        ft_handle,
        p_ft_eeprom,
        std::mem::size_of::<T::FtEeprom>() as u32,
        p_manufacturer,
        p_manufacturer_id,
        p_description,
        p_serial_number
    ));

    let mut eeprom: T = T::from(ft_eeprom);
    eeprom.set_manufacturer(utils::i8_array_to_string(&manufacturer));
    eeprom.set_manufacturer_id(utils::i8_array_to_string(&manufacturer_id));
    eeprom.set_description(utils::i8_array_to_string(&description));
    eeprom.set_serial_number(utils::i8_array_to_string(&serial_number));
    *eeprom.handle_mut() = ft_handle;

    Ok(eeprom)
}

/// 4.12 FT_EEPROM_Program TODO
pub fn eeprom_program<T: Eeprom>(ft_handle: FtHandle, eeprom: &T) -> Result<(), FtError> {
    let mut manufacturer: [i8; 64] = string_to_i8_array(eeprom.get_manufacturer());
    let mut manufacturer_id: [i8; 64] = string_to_i8_array(eeprom.get_manufacturer_id());
    let mut description: [i8; 64] = string_to_i8_array(eeprom.get_description());
    let mut serial_number: [i8; 64] = string_to_i8_array(eeprom.get_serial_number());

    let mut ft_eeprom: T::FtEeprom = eeprom.into();
    let p_ft_eeprom: *mut c_void = (&mut ft_eeprom as *mut T::FtEeprom).cast();

    ft_try!(d2xx::FT_EEPROM_Program(
        ft_handle,
        p_ft_eeprom,
        std::mem::size_of::<T::FtEeprom>() as u32,
        manufacturer.as_mut_ptr(),
        manufacturer_id.as_mut_ptr(),
        description.as_mut_ptr(),
        serial_number.as_mut_ptr()
    ));
    Ok(())
}

/// 5.1 FT_SetLatencyTimer
pub fn set_latency_timer(ft_handle: FtHandle, timer: u8) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetLatencyTimer(ft_handle, timer));
    Ok(())
}

/// 5.2 FT_GetLatencyTimer
pub fn get_latency_timer(ft_handle: FtHandle) -> Result<u8, FtError> {
    let mut timer: u8 = 0;
    ft_try!(d2xx::FT_GetLatencyTimer(ft_handle, &mut timer));
    Ok(timer)
}

/// 5.3 FT_SetBitMode
pub fn set_bit_mode(ft_handle: FtHandle, mask: u8, bit_mode: BitMode) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetBitMode(ft_handle, mask, bit_mode as u8));
    Ok(())
}

/// 5.4 FT_GetBitMode
pub fn get_bit_mode(ft_handle: FtHandle) -> Result<u8, FtError> {
    let mut bit_mode: u8 = 0;
    ft_try!(d2xx::FT_GetBitMode(ft_handle, &mut bit_mode));
    // TODO BitMode::from(u8)
    Ok(bit_mode)
}

/// 5.5 FT_SetUSBParameters
pub fn set_usb_parameters(
    ft_handle: FtHandle,
    in_transfer_size: u32,
    out_transfer_size: u32,
) -> Result<(), FtError> {
    ft_try!(d2xx::FT_SetUSBParameters(
        ft_handle,
        in_transfer_size,
        out_transfer_size
    ));
    Ok(())
}
