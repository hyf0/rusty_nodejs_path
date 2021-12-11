// Align to https://github.com/nodejs/node/blob/master/lib/path.js

/// `nodejs_path::posix` provides access to POSIX specific implementations of the path methods.
pub mod posix;
/// `nodejs_path::win32` provides access to  Windows-specific implementations of the path methods.
pub mod win32;

pub(crate) mod shared;
