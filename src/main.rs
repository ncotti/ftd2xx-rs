use ftd2xx_rs::*;

fn main() {
    let version: Version = get_library_version().unwrap();
    println!("{}", version);
}
