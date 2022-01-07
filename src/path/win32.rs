use super::shared::{
    is_path_separator, CHAR_COLON, CHAR_LOWERCASE_A, CHAR_LOWERCASE_Z, CHAR_UPPERCASE_A,
    CHAR_UPPERCASE_Z,
};

/// Provides the platform-specific path segment separator:
/// - `\` on Windows
/// - `/` on POSIX
#[allow(non_upper_case_globals)]
pub const sep: char = '\\';
/// Provides the platform-specific path delimiter:
/// - `;` for Windows
/// - `:` for POSIX
#[allow(non_upper_case_globals)]
pub const delimiter: char = ';';

/// The method determines if path is an absolute path. If the given path is a zero-length string, false will be returned.
/// 
/// #Example
/// ```rust
/// assert_eq!(nodejs_path::win32::is_absolute("//server"), true);
/// assert_eq!(nodejs_path::win32::is_absolute("\\\\server"), true);
/// assert_eq!(nodejs_path::win32::is_absolute("C:/foo/.."), true);
/// assert_eq!(nodejs_path::win32::is_absolute("C:\\foo\\.."), true);  
/// assert_eq!(nodejs_path::win32::is_absolute("bar\\baz"), false);  
/// assert_eq!(nodejs_path::win32::is_absolute("bar/baz"), false);  
/// assert_eq!(nodejs_path::win32::is_absolute("."), false);  
/// ```
pub fn is_absolute(path: &str) -> bool {
    if path.is_empty() {
        false
    } else {
        let path_len = path.len();
        let mut path = path.chars();
        let idx0 = path.next();
        idx0.clone().map_or(false, |c| is_path_separator(&c))
            || (path_len > 2 && idx0.map_or(false, |c| is_windows_device_root(c)) && {
                let idx1 = path.next();
                let idx2 = path.next();
                idx1.map_or(false, |c| c == CHAR_COLON)
                    && idx2.map_or(false, |c| is_path_separator(&c))
            })
    }
}

fn is_windows_device_root(code: char) -> bool {
    (code >= CHAR_UPPERCASE_A && code <= CHAR_UPPERCASE_Z)
        || (code >= CHAR_LOWERCASE_A && code <= CHAR_LOWERCASE_Z)
}
