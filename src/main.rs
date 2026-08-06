use ftd2xx_rs::EepromFt4232h;
use ftd2xx_rs::scan;
use ftd2xx_rs::*;

fn main() {
    let version: Version = get_library_version().unwrap();
    println!("{}", version);

    let (vid, pid) = get_vid_pid().unwrap();
    println!("{}, {}", vid, pid);

    set_vid_pid(0xAAAA, 0xFFFF).unwrap();

    let (vid, pid) = get_vid_pid().unwrap();
    println!("{}, {}", vid, pid);

    let device_number = create_device_info_list().unwrap();
    let device_list = get_device_info_list(device_number).unwrap();

    println!("Devices connected: {device_number}");
    println!("{:?}", device_list[0]);

    let some_device = get_device_info_detail(2).unwrap();

    println!("{:?}", some_device);

    let handle = open(0).unwrap();

    let device_infos: Vec<DevInfo> = scan().unwrap();
    for info in device_infos {
        println!("{}", info)
    }

    let eeprom: EepromFt4232h = classic::eeprom_read(handle).unwrap();

    println!("{:?}", eeprom);
}
