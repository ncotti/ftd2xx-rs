//! # Types module.
//!
//! It re-import all relevant types from the `types` folder to be used by the
//! user and in other parts of the library.

mod dev_info;
mod dev_type;
mod fterror;
mod version;

pub use dev_info::{DevInfo, FT_DEFAULT_PRODUCT_ID, FT_DEFAULT_VENDOR_ID};
pub use dev_type::DevType;
pub use fterror::FtError;
pub(crate) use fterror::ft_try;
pub use version::Version;
