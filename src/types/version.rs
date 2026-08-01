//! FTD2XX library version

use std::fmt;

/// Holds the current library version, as v.<major>.<minor>.<build>.
#[derive(Debug, Copy, Clone)]
#[allow(missing_docs)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
}

impl Version {
    /// Creates a new version struct from a "version number".
    /// E.g., version "v3.01.15 == v<major>.<minor>.<build>" is expected to be
    /// the number 0x00030115.
    pub fn new(version: u32) -> Self {
        Self {
            major: ((((version >> 20) & 0xf) * 10) + ((version >> 16) & 0xf)) as u8,
            minor: ((((version >> 12) & 0xf) * 10) + ((version >> 8) & 0xf)) as u8,
            build: ((((version >> 4) & 0xf) * 10) + ((version >> 0) & 0xf)) as u8,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.build)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version_conversion() {
        let version_number: u32 = 0x030115;
        let version = Version::new(version_number);
        assert!(version.major == 03);
        assert!(version.minor == 01);
        assert!(version.build == 15);
    }
}
