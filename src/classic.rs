//! Implements safe Rust wrappers around all D2XX classic functions as
//! described in section 3 of the D2XX Programmer's Guide.

use crate::{
    fterror::{FtError, ft_try}, types::{DeviceInfo, FlowControl, UartInfo},
};
use ftd2xx_sys::*;

use std::ffi::c_void;

/// 3.1 FT_SetVIDPID
pub fn set_vid_pid(vid: u16, pid: u16) -> Result<(), FtError> {
    let vid: u32 = vid.into();
    let pid: u32 = pid.into();
    ft_try!(FT_SetVIDPID(vid, pid));
    Ok(())
}

/// 3.2 FT_GetVIDPID
pub fn get_vid_pid() -> Result<(u16, u16), FtError> {
    let mut vid: u32 = 0;
    let mut pid: u32 = 0;
    ft_try!(FT_GetVIDPID(&mut vid, &mut pid));
    let vid: u16 = vid as u16;
    let pid: u16 = pid as u16;
    Ok((vid, pid))
}
/// 3.3 FT_CreateDeviceInfoList
pub fn create_device_info_list(number_of_devices: u32) -> Result<(), FtError> {
    let mut number_of_devices: u32 = number_of_devices;
    ft_try!(FT_CreateDeviceInfoList(&mut number_of_devices));
    Ok(())
}

/// 3.4 FT_GetDeviceInfoList
pub fn get_device_info_list(number_of_devices: u32) -> Result<Vec<DeviceInfo>, FtError> {
    let mut number_of_devices: u32 = number_of_devices;
    let mut ft_devices_info: Vec<FT_DEVICE_LIST_INFO_NODE> = vec![
        FT_DEVICE_LIST_INFO_NODE {
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
    ft_try!(FT_GetDeviceInfoList(
        ft_devices_info.as_mut_ptr(),
        &mut number_of_devices
    ));

    let mut devices_info: Vec<DeviceInfo> = Vec::new();

    for dev in ft_devices_info {
        devices_info.push(DeviceInfo::new(dev));
    }

    Ok(devices_info)
}

/// 3.5 FT_GetDeviceInfoDetail
pub fn get_device_info_detail(device_index: u32) -> Result<DeviceInfo, FtError> {
    let mut flags: u32 = 0;
    let mut ft_type: u32 = 0;
    let mut id: u32 = 0;
    let mut locid: u32 = 0;
    let mut serial_number: [i8; 16] = [0; 16];
    let mut description: [i8; 64] = [0; 64];
    let mut ft_handle: FT_HANDLE = std::ptr::null_mut();

    let p_serial_number: *mut c_void = serial_number.as_mut_ptr().cast();
    let p_description: *mut c_void = description.as_mut_ptr().cast();

    ft_try!(FT_GetDeviceInfoDetail(
        device_index,
        &mut flags,
        &mut ft_type,
        &mut id,
        &mut locid,
        p_serial_number,
        p_description,
        &mut ft_handle
    ));

    let ft_device_info = FT_DEVICE_LIST_INFO_NODE {
        Flags: flags,
        Type: ft_type,
        ID: id,
        LocId: locid,
        SerialNumber: serial_number,
        Description: description,
        ftHandle: ft_handle,
    };

    let device_info = DeviceInfo::new(ft_device_info);

    Ok(device_info)
}

/// TODO 3.6 FT_ListDevices TODO
pub fn list_devices() -> Result<(), FtError> {
    Ok(())
}

/// 3.7 FT_Open
pub fn open(device_index: u32) -> Result<FT_HANDLE, FtError> {
    let mut ft_handle: FT_HANDLE = std::ptr::null_mut();
    ft_try!(FT_Open(device_index as i32, &mut ft_handle));
    Ok(ft_handle)
}

/// 3.8 FT_OpenEx, with flag FT_OPEN_BY_SERIAL_NUMBER
pub fn open_ex_by_serial_number(serial_number: [i8; 16]) -> Result<FT_HANDLE, FtError> {
    let mut ft_handle: FT_HANDLE = std::ptr::null_mut();
    let mut serial_number = serial_number;
    let p_serial_number: *mut c_void = serial_number.as_mut_ptr().cast();
    ft_try!(FT_OpenEx(
        p_serial_number,
        FT_OPEN_BY_SERIAL_NUMBER,
        &mut ft_handle
    ));
    Ok(ft_handle)
}

/// 3.8 FT_OpenEx, with flag FT_OPEN_BY_DESCRIPTION
pub fn open_ex_by_description(description: &str) -> Result<FT_HANDLE, FtError> {
    let mut ft_handle: FT_HANDLE = std::ptr::null_mut();
    let p_description: *mut c_void = description.as_ptr() as *mut c_void;
    ft_try!(FT_OpenEx(
        p_description,
        FT_OPEN_BY_DESCRIPTION,
        &mut ft_handle
    ));
    Ok(ft_handle)
}

/// 3.8 FT_OpenEx, with flag FT_OPEN_BY_LOCATION
pub fn open_ex_by_location(location_id: u32) -> Result<FT_HANDLE, FtError> {
    let mut ft_handle: FT_HANDLE = std::ptr::null_mut();
    let mut location_id = location_id;
    let p_location_id: *mut c_void = (&mut location_id as *mut u32).cast();
    ft_try!(FT_OpenEx(
        p_location_id,
        FT_OPEN_BY_LOCATION,
        &mut ft_handle
    ));
    Ok(ft_handle)
}

/// 3.9 FT_Close
pub fn close(ft_handle: FT_HANDLE) -> Result<(), FtError> {
    ft_try!(FT_Close(ft_handle));
    Ok(())
}

/// 3.10 FT_Read
pub fn read(ft_handle: FT_HANDLE, bytes_to_read:u32) -> Result<Vec<u8>, FtError> {
    let mut bytes_read: u32 = 0;
    let mut bytes: Vec<u8> = Vec::new();
    bytes.reserve_exact(bytes_to_read as usize);
    ft_try!(FT_Read(ft_handle, bytes.as_mut_ptr().cast(), bytes_to_read, &mut bytes_read));
    Ok(bytes)
}

/// 3.11 FT_Write
pub fn write(ft_handle: FT_HANDLE, data: &mut Vec<u8>) -> Result<u32, FtError> {
    let mut bytes_written: u32 = 0;
    let bytes_to_be_written: u32 = data.len() as u32;
    ft_try!(FT_Write(ft_handle, data.as_mut_ptr().cast(), bytes_to_be_written, &mut bytes_written));
    Ok(bytes_written)
}

/// 3.12 FT_SetBaudRate
pub fn set_baud_rate(ft_handle: FT_HANDLE, baud_rate:u32) -> Result<(), FtError> {
    ft_try!(FT_SetBaudRate(ft_handle, baud_rate));
    Ok(())
}

/// 3.13 FT_SetDivisor
pub fn set_divisor(ft_handle: FT_HANDLE, divisor: u16) -> Result<(), FtError> {
    ft_try!(FT_SetDivisor(ft_handle, divisor));
    Ok(())
}

/// 3.14 FT_SetDataCharacteristics
pub fn set_data_characteristics(ft_handle: FT_HANDLE, uart_info: UartInfo) -> Result<(), FtError> {
    ft_try!(FT_SetDataCharacteristics(ft_handle, uart_info.bits as u8, uart_info.stop_bits as u8, uart_info.parity as u8));
    Ok(())
}

/// 3.15 FT_SetTimeouts
pub fn set_timeouts(ft_handle: FT_HANDLE, read_timeout: u32, write_timeout: u32) -> Result<(), FtError> {
    ft_try!(FT_SetTimeouts(ft_handle, read_timeout, write_timeout));
    Ok(())
}

/// 3.16 FT_SetFlowControl
pub fn set_flow_control(ft_handle: FT_HANDLE, flow_control: FlowControl, xon_char: u8, xoff_char: u8) -> Result<(), FtError> {
    ft_try!(FT_SetFlowControl(ft_handle, flow_control as u16, xon_char, xoff_char));
    Ok(())
}

/// 3.17 FT_SetDtr
pub fn set_dtr(ft_handle: FT_HANDLE) -> Result<(), FtError> {
    ft_try!(FT_SetDtr(ft_handle));
    Ok(())
}

/// 3.18 FT_ClrDtr
pub fn clr_dtr(ft_handle: FT_HANDLE) -> Result<(), FtError> {
    ft_try!(FT_ClrDtr(ft_handle));
    Ok(())
}

/// 3.19 FT_SetRts
pub fn set_rts(ft_handle: FT_HANDLE) -> Result<(), FtError> {
    ft_try!(FT_SetRts(ft_handle));
    Ok(())
}

/// 3.20 FT_ClrRts
pub fn clr_rts(ft_handle: FT_HANDLE) -> Result<(), FtError> {
    ft_try!(FT_ClrRts(ft_handle));
    Ok(())
}

/// 3.21 FT_GetModemStatus
pub fn get_modem_status(ft_handle: FT_HANDLE) -> Result<u32, FtError> {
    let mut modem_status:u32 = 0;
    ft_try!(FT_GetModemStatus(ft_handle, &mut modem_status));
    Ok(modem_status)
}

// Note, 3.22 and 3.23 are Windows only and therefore not included

/// 3.24 FT_GetQueueStatus
pub fn get_queue_status(ft_handle: FT_HANDLE) -> Result<u32, FtError> {
    let mut bytes_in_rx_queue: u32 = 0;
    ft_try!(FT_GetQueueStatus(ft_handle, &mut bytes_in_rx_queue));
    Ok(bytes_in_rx_queue)
}

/// 3.25 FT_GetDeviceInfo
pub fn get_device_info(ft_handle: FT_HANDLE) -> Result<DeviceInfo, FtError> {

    let mut flags: u32 = 0;
    let mut ft_type: u32 = 0;
    let mut id: u32 = 0;
    let mut locid: u32 = 0;
    let mut serial_number: [i8; 16] = [0; 16];
    let mut description: [i8; 64] = [0; 64];
    let mut dummy_void_ptr: FT_HANDLE = std::ptr::null_mut();

    ft_try!(FT_GetDeviceInfo(ft_handle, &mut ft_type, &mut id, serial_number.as_mut_ptr(), description.as_mut_ptr(), dummy_void_ptr));

    let ft_device_info = FT_DEVICE_LIST_INFO_NODE {
        Flags: flags,
        Type: ft_type,
        ID: id,
        LocId: locid,
        SerialNumber: serial_number,
        Description: description,
        ftHandle: ft_handle,
    };

    let device_info = DeviceInfo::new(ft_device_info);
    Ok(device_info)
}


