//! # Description
//!
//! The default operation of the path module varies based on the operating system on which crate is compiling. Specifically, when compiling on a Windows operating system, the path module will assume that Windows-style paths are being used.
//! So using `nodejs_path::basename()` might yield different results on POSIX and Windows:
//!
//! On POSIX:
//! ```no_run
//! nodejs_path::basename("C:\\temp\\myfile.html");
//! // Returns: "C:\\temp\\myfile.html"
//! ```
//! On Windows:
//! ```no_run
//! nodejs_path::basename("C:\\temp\\myfile.html");
//! Returns: "myfile.html"
//! ```
//! To achieve consistent results when working with Windows file paths on any operating system, use [`nodejs_path::win32`](win32):
//! On POSIX and Windows:
//! ```ignore
//! assert_eq!(&nodejs_path::win32::basename!("C:\\temp\\myfile.html"), "myfile.html")
//! ```
//! To achieve consistent results when working with POSIX file paths on any operating system, use [`nodejs_path::posix`](posix):
//! On POSIX and Windows:
//! ```rust
//! assert_eq!(&nodejs_path::posix::basename!("/tmp/myfile.html"), "myfile.html")
//! ```
//!
//! # Porting
//! Rust doesn't support variadic function. So, variadic functions of Node.js must be ported to macros.
//! - path.basename          => [`nodejs_path::basename!`](self::basename)
//! - path.delimiter         => [`nodejs_path::delimiter`](self::delimiter)
//! - path.dirname           => [`nodejs_path::dirname`](self::dirname)
//! - path.extname           => [`nodejs_path::extname`](self::extname)
//! - path.format            => [`nodejs_path::format`](self::format)
//! - path.isAbsolute        => [`nodejs_path::is_absolute`](self::is_absolute)
//! - path.join              => [`nodejs_path::join!`](self::join)
//! - path.normalize         => [`nodejs_path::normalize`](self::normalize)
//! - path.parse             => [`nodejs_path::parse`](self::parse)
//! - path.relative          => [`nodejs_path::relative!`](self::relative)
//! - path.resolve           => [`nodejs_path::resolve!`](self::resolve)
//! - path.sep               => [`nodejs_path::sep`](self::sep)
//! - path.toNamespacedPath  => [`nodejs_path::to_namespaced_path`](self::to_namespaced_path)

// Align to https://nodejs.org/docs/latest-v16.x/api/path.html

mod path;

#[cfg(target_family = "unix")]
pub use path::posix::*;
#[cfg(target_family = "windows")]
pub use path::win32::*;

pub use path::posix;
pub use path::win32;

pub use path::shared::Parsed;
