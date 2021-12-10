// Align to https://nodejs.org/docs/latest-v16.x/api/path.html

mod path;

#[cfg(target_family = "unix")]
pub use path::posix::*;
#[cfg(target_family = "windows")]
pub use path::win32::*;

pub use path::posix;
pub use path::win32;

pub use path::shared::Parsed;
