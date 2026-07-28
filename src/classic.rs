//! Implements safe Rust wrappers around all D2XX classic functions as
//! described in section 3 of the D2XX Programmer's Guide.

use crate::{
    fterror::{FtError, ft_try},
    types::DeviceInfo,
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