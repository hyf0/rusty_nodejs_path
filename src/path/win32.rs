use crate::Parsed;

use super::shared::{
    is_path_separator, CHAR_COLON, CHAR_DOT, CHAR_LOWERCASE_A, CHAR_LOWERCASE_Z, CHAR_UPPERCASE_A,
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

pub fn parse(raw_path: &str) -> Parsed {
    let mut ret = Parsed::default();
    if raw_path.len() == 0 {
        return ret;
    }
    let path = raw_path.chars().collect::<Vec<char>>();
    let len = path.len();
    let mut root_end = 0;
    let mut code = path.get(0).unwrap();
    if len == 1 {
        if is_path_separator(code) {
            // `path` contains just a path separator, exit early to avoid
            // unnecessary work
            ret.dir = raw_path.to_string();
            ret.root = ret.dir.clone();
            return ret;
        }
        ret.name = raw_path.to_string();
        ret.base = ret.name.clone();
        return ret;
    }
    // Try to match a root
    if is_path_separator(code) {
        // Possible UNC root

        root_end = 1;
        if is_path_separator(path.get(1).unwrap()) {
            // Matched double path separator at beginning
            let mut j = 2;
            let mut last = j;
            // Match 1 or more non-path separators
            while j < len && !is_path_separator(path.get(j).unwrap()) {
                j += 1;
            }

            if j < len && j != last {
                last = j;
                while j < len && !is_path_separator(path.get(j).unwrap()) {
                    j += 1;
                }
                if j == len {
                    root_end = j;
                } else if j != last {
                    root_end = j + 1;
                }
            }
        } else if is_windows_device_root(*code) && path.get(1).unwrap() == &CHAR_COLON {
            if len <= 2 {
                ret.dir = raw_path.to_string();
                ret.root = ret.dir.clone();
                return ret;
            }
            root_end = 2;
            if is_path_separator(path.get(2).unwrap()) {
                if len == 3 {
                    ret.dir = raw_path.to_string();
                    ret.root = ret.dir.clone();
                    return ret;
                }
                root_end = 3;
            }
        }

        if root_end > 0 {
            ret.root = path[0..root_end].into_iter().collect();
        }
    }

    let mut startDot: i32 = -1;
    let mut startPart = root_end;
    let mut end: i32 = -1;
    let mut matchedSlash = true;
    let mut i = path.len() - 1;

    // Track the state of characters (if any) we see before our first dot and
    // after any path separator we find
    let mut preDotState = 0;

    // Get non-dir info
    while i >= root_end {
        code = path.get(i).unwrap();
        if (is_path_separator(code)) {
            // If we reached a path separator that was not part of a set of path
            // separators at the end of the string, stop now
            if (!matchedSlash) {
                startPart = i + 1;
                break;
            }
            i -= 1;
            continue;
        }
        if (end == -1) {
            // We saw the first non-path separator, mark this as the end of our
            // extension
            matchedSlash = false;
            end = i as i32 + 1;
        }
        if (code == &CHAR_DOT) {
            // If this is our first dot, mark it as the start of our extension
            if (startDot == -1) {
                startDot = i as i32;
            } else if (preDotState != 1) {
                preDotState = 1;
            }
        } else if (startDot != -1) {
            // We saw a non-dot and non-path separator before our dot, so we should
            // have a good chance at having a non-empty extension
            preDotState = -1;
        }
    }

    if (end != -1) {
        if (startDot == -1 ||
          // We saw a non-dot character immediately before the dot
          preDotState == 0 ||
          // The (right-most) trimmed path component is exactly '..'
          (preDotState == 1 &&
           startDot == end - 1 &&
           startDot == startPart as i32 + 1))
        {
            ret.name = path[startPart..end as usize].into_iter().collect();
            ret.base = ret.name.clone();
        } else {
            ret.name = path[startPart..startDot as usize].into_iter().collect();
            ret.base = path[startPart..end as usize].into_iter().collect();
            ret.ext = path[startDot as usize..end as usize].into_iter().collect();
        }
    }

    // If the directory is the root, use the entire root as the `dir` including
    // the trailing slash if any (`C:\abc` -> `C:\`). Otherwise, strip out the
    // trailing slash (`C:\abc\def` -> `C:\abc`).
    if (startPart > 0 && startPart != root_end) {
        ret.dir = path[0..startPart - 1].into_iter().collect();
    } else {
        ret.dir = ret.root.clone();
    }

    return ret;
}
