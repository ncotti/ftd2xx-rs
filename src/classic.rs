//! Implements safe Rust wrappers around all D2XX classic functions as
//! described in section 3 of the D2XX Programmer's Guide.

use crate::{
    fterror::{FtError, ft_try},
    types::DeviceInfo,
};
use ftd2xx_sys::*;

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
pub fn get_device_info_detail() -> Result<(), FtError> {
    Ok(())
}

/// 3.6 FT_ListDevices
pub fn list_devices() -> Result<(), FtError> {
    Ok(())
}
