#![warn(missing_docs)]

//! Crate doc

use ftd2xx_sys::*;

pub mod classic;
pub mod device_type;
pub mod fterror;
pub mod types;

pub use classic::*;
pub use device_type::*;
pub use fterror::FtError;
pub use types::Version;

/// Get the libftd2xx.so or libftd2xx.a library version.
pub fn get_library_version() -> Result<Version, FtError> {
    let mut version: u32 = 0;
    let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };

    if status != FT_OK {
        return Err(FtError::try_from(status).unwrap());
    }
    Ok(Version::new(version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_get_library_version() {
        let version = get_library_version().unwrap();
        let re = Regex::new(r"^v\d{1,2}\.\d{1,2}\.\d{1,2}$").unwrap();
        println!("{}", version);
        assert!(re.is_match(&version.to_string()));
    }
}
