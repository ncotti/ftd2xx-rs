use ftd2xx_rs::*;

fn main() {
    let version: Version = get_library_version().unwrap();
    println!("{}", version);

    let (vid, pid) = get_vid_pid().unwrap();
    println!("{}, {}", vid, pid);

    set_vid_pid(0xAAAA, 0xFFFF).unwrap();

    let (vid, pid) = get_vid_pid().unwrap();
    println!("{}, {}", vid, pid);

    create_device_info_list(5).unwrap();
    let device_list = get_device_info_list(5).unwrap();

    println!("{:?}", device_list);
}
