use std::{borrow::Cow, ops::Add};

use crate::Parsed;

use super::shared::{
    format_inner, is_posix_path_separator, normalize_string, CHAR_DOT, CHAR_FORWARD_SLASH,
};

/// Provides the platform-specific path segment separator:
/// - `\` on Windows
/// - `/` on POSIX
#[allow(non_upper_case_globals)]
pub const sep: char = '/';
/// Provides the platform-specific path delimiter:
/// - `;` for Windows
/// - `:` for POSIX
#[allow(non_upper_case_globals)]
pub const delimiter: char = ':';

///
/// ```rust
/// assert_eq!(&nodejs_path::basename_impl("/foo/bar/baz/asdf/quux.html"), "quux.html");
/// ```
#[inline]
pub fn basename_impl(path: &str) -> String {
    basename_impl_without_ext(path, "")
}

/// ```rust
/// assert_eq!(&nodejs_path::basename_impl_without_ext("/foo/bar/baz/asdf/quux.html", ".html"), "quux");
///
/// assert_eq!(&nodejs_path::basename_impl_without_ext("/foo/bar/baz/asdf/quux.HTML", ".html"), "quux.HTML");
///
/// assert_eq!(&nodejs_path::basename_impl_without_ext("aaa/bbb", "bbb"), "bbb");
/// ```
pub fn basename_impl_without_ext(path: &str, ext: &str) -> String {
    let mut start = 0;
    let mut end = -1;
    let mut matched_slash = true;

    let path = path.chars().collect::<Vec<char>>();
    let ext = ext.chars().collect::<Vec<char>>();

    if ext.len() > 0 && ext.len() <= path.len() {
        if ext == path {
            return "".to_owned();
        }
        let mut ext_idx = ext.len() as i32 - 1;
        let mut first_non_slash_end = -1;
        let mut i = path.len() as i32 - 1;
        while i >= 0 {
            let code = path.get(i as usize).unwrap();

            if code == &CHAR_FORWARD_SLASH {
                // If we reached a path separator that was not part of a set of path
                // separators at the end of the string, stop now
                if !matched_slash {
                    start = i + 1;
                    break;
                }
            } else {
                if first_non_slash_end == -1 {
                    // We saw the first non-path separator, remember this index in case
                    // we need it if the extension ends up not matching
                    matched_slash = false;
                    first_non_slash_end = i + 1;
                }
                if ext_idx >= 0 {
                    // Try to match the explicit extension
                    if code == ext.get(ext_idx as usize).unwrap() {
                        ext_idx -= 1;
                        if ext_idx == -1 {
                            // We matched the extension, so mark this as the end of our path
                            // component
                            end = i;
                        }
                    } else {
                        // Extension does not match, so our result is the entire path
                        // component
                        ext_idx = -1;
                        end = first_non_slash_end;
                    }
                }
            }

            i -= 1;
        }

        if start == end {
            end = first_non_slash_end
        } else if end == -1 {
            end = path.len() as i32
        }

        return path[start as usize..end as usize].iter().collect();
    }

    let mut i = path.len() as i32 - 1;
    while i >= 0 {
        if path.get(i as usize).unwrap() == &CHAR_FORWARD_SLASH {
            // If we reached a path separator that was not part of a set of path
            // separators at the end of the string, stop now
            if !matched_slash {
                start = i + 1;
                break;
            }
        } else if end == -1 {
            // We saw the first non-path separator, mark this as the end of our
            // path component
            matched_slash = false;
            end = i + 1;
        }

        i -= 1;
    }

    if end == -1 {
        return "".to_owned();
    }

    return path[start as usize..end as usize].iter().collect();
}

/// Returns the last portion of a path, similar to the Unix basename command. Trailing directory separators are ignored.
/// ```rust
/// assert_eq!(&nodejs_path::basename!("/foo/bar/baz/asdf/quux.html"), "quux.html");
///
/// assert_eq!(&nodejs_path::basename!("/foo/bar/baz/asdf/quux.html", ".html"), "quux");
///
/// assert_eq!(&nodejs_path::basename!("/foo/bar/baz/asdf/quux.HTML", ".html"), "quux.HTML");
/// ```

#[macro_export]
macro_rules! basename {
    (  $x:expr  ) => {{
        $crate::posix::basename_impl($x)
    }};
    (  $x:expr, $y:expr  ) => {{
        $crate::posix::basename_impl_without_ext($x, $y)
    }};
}
pub use basename;

/// Returns the directory name of a path, similar to the Unix dirname command. Trailing directory separators are ignored,
/// ```rust
/// assert_eq!(&nodejs_path::dirname("/foo/bar/baz/asdf/quux"), "/foo/bar/baz/asdf");
/// ```
pub fn dirname(path: &str) -> String {
    if path.len() == 0 {
        ".".to_owned()
    } else {
        let path = path.chars().collect::<Vec<char>>();
        let has_root = path
            .iter()
            .next()
            .map(|c| c == &CHAR_FORWARD_SLASH)
            .unwrap_or(false);
        let mut end = -1;
        let mut matched_slash = true;

        let mut i = path.len() as i32 - 1;
        while i >= 1 {
            if path
                .get(i as usize)
                .map(|c| c == &CHAR_FORWARD_SLASH)
                .unwrap_or(false)
            {
                if !matched_slash {
                    end = i;
                    break;
                }
            } else {
                // We saw the first non-path separator
                matched_slash = false;
            }

            i -= 1;
        }

        if end == -1 {
            if has_root {
                "/".to_owned()
            } else {
                ".".to_owned()
            }
        } else if has_root && end == 1 {
            "//".to_owned()
        } else {
            path[0..end as usize].iter().collect()
        }
    }
}
/// Returns the extension of the path, from the last occurrence of the . (period) character to end of string in the last portion of the path. If there is no . in the last portion of the path, or if there are no . characters other than the first character of the basename of path, an empty string is returned.
/// ```rust
/// assert_eq!(&nodejs_path::extname("index.html"), ".html");
///
/// assert_eq!(&nodejs_path::extname("index.coffee.md"), ".md");
///
/// assert_eq!(&nodejs_path::extname("index."), ".");
///
/// assert_eq!(&nodejs_path::extname("index"), "");
///
/// assert_eq!(&nodejs_path::extname(".index.md"), ".md");
/// ```
pub fn extname(path: &str) -> String {
    parse(path).ext
}

/// Returns a path string from an object. This is the opposite of nodejs_path::parse().

pub fn format(path_object: Parsed) -> String {
    format_inner("/", path_object)
}

/// The method determines if path is an absolute path. If the given path is a zero-length string, false will be returned.
/// #Example
/// ```rust
/// assert_eq!(nodejs_path::posix::is_absolute("/foo/bar"), true);
/// assert_eq!(nodejs_path::posix::is_absolute("/baz/.."), true);
/// assert_eq!(nodejs_path::posix::is_absolute("qux/"), false);
/// assert_eq!(nodejs_path::posix::is_absolute("."), false);  
/// assert_eq!(nodejs_path::posix::is_absolute(""), false);  
/// ```
pub fn is_absolute(path: &str) -> bool {
    path.bytes().next().map(|c| c == b'/').unwrap_or(false)
}

/// The method joins all given path segments together using the platform-specific separator as a delimiter, then normalizes the resulting path.
///
/// Zero-length path segments are ignored. If the joined path string is a zero-length string then '.' will be returned, representing the current working directory.
/// ```rust
/// assert_eq!(nodejs_path::posix::join!("/foo", "bar", "baz/asdf", "quux", ".."), "/foo/bar/baz/asdf");
/// ```
#[macro_export]
macro_rules! join {
    ( $( $x:expr ),* ) => {
      {
        $crate::posix::join_impl(&[
          $(
            $x,
          )*
        ])
      }
    };
  }
pub use join;

pub fn join_impl(args: &[&str]) -> String {
    if args.len() == 0 {
        ".".to_owned()
    } else {
        // let length =
        let joined = args
            .iter()
            .filter_map(|&arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(arg))
                }
            })
            .reduce(|mut pre, cur| {
                pre = pre.add("/");
                pre = pre.add(cur);
                pre
            });
        match joined {
            Some(joined) => normalize(&joined),
            None => ".".to_string(),
        }
    }
}

/// The path.normalize() method normalizes the given path, resolving '..' and '.' segments.
///
/// When multiple, sequential path segment separation characters are found (e.g. / on POSIX and either \ or / on Windows), they are replaced by a single instance of the platform-specific path segment /// separator (/ on POSIX and \ on Windows). Trailing separators are preserved.
///
/// If the path is a zero-length string, '.' is returned, representing the current working directory.
///
/// ```rust
/// assert_eq!(nodejs_path::posix::normalize("/foo/bar//baz/asdf/quux/.."), "/foo/bar/baz/asdf");
/// ```
pub fn normalize(path: &str) -> String {
    if path.len() == 0 {
        return ".".to_owned();
    } else {
        let is_absolute = is_absolute(path);
        let trailing_separator = path
            .chars()
            .last()
            .map(|c| c == CHAR_FORWARD_SLASH)
            .unwrap_or(false);
        let mut consecutive_dd = 0;
        // let mut path = normalize_string(path, !is_absolute, &'/', &is_posix_path_separator);
        let mut path_stack = vec![];
        path.split("/")
            .filter(|seg| !seg.is_empty())
            .for_each(|seg| {
                match seg {
                    "." => {}
                    ".." => {
                        // path_stack.pop();
                        if consecutive_dd == path_stack.len() {
                            path_stack.push(seg);
                            consecutive_dd += 1;
                        } else {
                            path_stack.pop();
                        }
                    }
                    other => {
                        path_stack.push(other);
                    }
                }
            });
        let mut normalized_path = if is_absolute {
            // if is absolute path, whatever how many times .. used, it is just the same as /
            path_stack
                .iter()
                .position(|&str| str != "..")
                .map(|item| path_stack[item..].join("/"))
                .unwrap_or("".to_string())
        } else {
            path_stack.join("/")
        };
        // if path.len() == 0 {
        //     if is_absolute {
        //         return "/".to_owned();
        //     } else if trailing_separator {
        //         return "./".to_owned();
        //     } else {
        //         return ".".to_owned();
        //     }
        // }
        // if is_ab
        if is_absolute {
            normalized_path = "/".to_string() + &normalized_path;
        }

        if normalized_path.is_empty() {
            normalized_path.push('.');
        }

        if trailing_separator && normalized_path != "/" {
            normalized_path.push('/');
        }

        normalized_path
    }
}

/// # Example
/// ```rust
/// assert_eq!(nodejs_path::parse("/home/user/dir/file.txt"), nodejs_path::Parsed{
///   root: "/".to_string(),
///   dir: "/home/user/dir".to_string(),
///   base: "file.txt".to_string(),
///   ext: ".txt".to_string(),
///   name: "file".to_string(),
/// })
/// ```
///
/// ```plain
/// ┌─────────────────────┬────────────┐
/// │          dir        │    base    │
/// ├──────┬              ├──────┬─────┤
/// │ root │              │ name │ ext │
/// "  /    home/user/dir / file  .txt "
/// └──────┴──────────────┴──────┴─────┘
/// (All spaces in the "" line should be ignored. They are purely for formatting.)
/// ```
pub fn parse(path: &str) -> Parsed {
    let path = path.chars().collect::<Vec<char>>();
    let mut ret = Parsed::default();
    if path.len() == 0 {
        ret
    } else {
        let is_absolute = path.get(0).map(|c| c == &CHAR_FORWARD_SLASH).unwrap();

        let start;
        if is_absolute {
            ret.root = "/".to_owned();
            start = 1;
        } else {
            start = 0;
        }
        let mut start_dot = -1;
        let mut start_part = 0;
        let mut end = -1;
        let mut matched_slash = true;
        let mut i = (path.len() - 1) as i32;

        // Track the state of characters (if any) we see before our first dot and
        // after any path separator we find
        let mut pre_dot_state = 0;

        // Get non-dir info
        while i >= start {
            let code = *path.get(i as usize).unwrap();
            if code == CHAR_FORWARD_SLASH {
                // If we reached a path separator that was not part of a set of path
                // separators at the end of the string, stop now
                if !matched_slash {
                    start_part = i + 1;
                    // i -= 1;
                    break;
                }
                i -= 1;
                continue;
            }
            if end == -1 {
                // We saw the first non-path separator, mark this as the end of our
                // extension
                matched_slash = false;
                end = i + 1;
            }
            if code == CHAR_DOT {
                // If this is our first dot, mark it as the start of our extension
                if start_dot == -1 {
                    start_dot = i;
                } else if pre_dot_state != 1 {
                    pre_dot_state = 1;
                }
            } else if start_dot != -1 {
                // We saw a non-dot and non-path separator before our dot, so we should
                // have a good chance at having a non-empty extension
                pre_dot_state = -1;
            }

            i -= 1;
        }

        if end != -1 {
            let start = if start_part == 0 && is_absolute {
                1
            } else {
                start_part
            };
            if start_dot == -1 ||
                // We saw a non-dot character immediately before the dot
                pre_dot_state == 0 ||
                // The (right-most) trimmed path component is exactly '..'
                (pre_dot_state == 1 &&
                start_dot == end - 1 &&
                start_dot == start_part + 1)
            {
                ret.base = path[start as usize..end as usize].iter().collect();
                ret.name = ret.base.clone();
            } else {
                ret.name = path[start as usize..start_dot as usize].iter().collect();
                ret.base = path[start as usize..end as usize].iter().collect();
                ret.ext = path[start_dot as usize..end as usize].iter().collect();
            }
        }

        if start_part > 0 {
            ret.dir = path[0..(start_part - 1) as usize].iter().collect();
        } else if is_absolute {
            ret.dir = "/".to_owned();
        }

        ret
    }
}

///
/// method returns the relative path from from to to based on the current working directory. If from and to each resolve to the same path (after calling resolve() on each), a zero-length string is returned.
/// ```rust
/// assert_eq!(nodejs_path::posix::relative("/data/orandea/test/aaa", "/data/orandea/impl/bbb"), "../../impl/bbb");
/// ```
pub fn relative(from: &str, to: &str) -> String {
    if from == to {
        "".to_owned()
    } else {
        let from = resolve!(&from).chars().collect::<Vec<char>>();
        let to = resolve!(&to).chars().collect::<Vec<char>>();

        if from == to {
            "".to_owned()
        } else {
            let from_start = 1;
            let from_end = from.len() as i32;
            let from_len = from_end - from_start;
            let to_start = 1;
            let to_len = to.len() as i32 - to_start;

            // Compare paths to find the longest common path from root
            let length = if from_len < to_len { from_len } else { to_len };

            let mut last_common_sep = -1;
            let mut i = 0;

            while i < length {
                let from_code = from.get((from_start + i) as usize).unwrap();
                if from_code != to.get((to_start + i) as usize).unwrap() {
                    break;
                } else if from_code == &CHAR_FORWARD_SLASH {
                    last_common_sep = i;
                }
                i += 1;
            }

            if i == length {
                if to_len > length {
                    if to.get((to_start + i) as usize).unwrap() == &CHAR_FORWARD_SLASH {
                        // We get here if `from` is the exact base path for `to`.
                        // For example: from='/foo/bar'; to='/foo/bar/baz'
                        return to[(to_start + i + 1) as usize..to.len()].iter().collect();
                        // return StringPrototypeSlice(to, toStart + i + 1);
                    }
                    if i == 0 {
                        // We get here if `from` is the root
                        // For example: from='/'; to='/foo'
                        return to[(to_start + i) as usize..to.len()].iter().collect();
                    }
                } else if from_len > length {
                    if from.get((from_start + i) as usize).unwrap() == &CHAR_FORWARD_SLASH {
                        // We get here if `to` is the exact base path for `from`.
                        // For example: from='/foo/bar/baz'; to='/foo/bar'
                        last_common_sep = i;
                    } else if i == 0 {
                        // We get here if `to` is the root.
                        // For example: from='/foo/bar'; to='/'
                        last_common_sep = 0;
                    }
                }
            }

            let mut out = "".to_owned();
            // Generate the relative path based on the path difference between `to`
            // and `from`.
            let mut i = from_start + last_common_sep + 1;
            while i <= from_end {
                if i == from_end || from.get(i as usize).unwrap() == &CHAR_FORWARD_SLASH {
                    if out.len() == 0 {
                        out.push_str("..")
                    } else {
                        out.push_str("/..")
                    }
                    // out += out.length === 0 ? '..' : '/..';
                }
                i += 1;
            }

            // Lastly, append the rest of the destination (`to`) path that comes after
            // the common path parts.
            format!(
                "{}{}",
                &out,
                &to[(to_start + last_common_sep) as usize..to.len()]
                    .iter()
                    .collect::<String>()
            )
            // return `${out}${StringPrototypeSlice(to, toStart + lastCommonSep)}`;
        }
    }
}

pub fn resolve_impl(args: &[&str]) -> String {
    let mut resolved_path = "".to_owned();
    let mut resolved_absolute = false;

    let mut i = args.len() as i32 - 1;

    while i >= -1 && !resolved_absolute {
        let path = if i >= 0 {
            args.get(i.clone() as usize).unwrap().to_string()
        } else {
            posix_cwd()
        };

        // Skip empty entries
        if path.len() == 0 {
            i -= 1;
            continue;
        }

        resolved_path = format!("{}/{}", path, resolved_path);
        resolved_absolute = path
            .chars()
            .next()
            .map(|c| c == CHAR_FORWARD_SLASH)
            .unwrap_or(false);

        i -= 1;
    }

    // At this point the path should be resolved to a full absolute path, but
    // handle relative paths to be safe (might happen when process.cwd() fails)

    // Normalize the path
    resolved_path = normalize_string(
        &resolved_path,
        !resolved_absolute,
        &sep,
        &is_posix_path_separator,
    );

    if resolved_absolute {
        "/".to_owned() + &resolved_path
    } else {
        if !resolved_path.is_empty() {
            resolved_path
        } else {
            ".".to_owned()
        }
    }
}

/// Resolves a sequence of paths or path segments into an absolute path.
///
/// ```rust
/// assert_eq!(&nodejs_path::resolve!("/foo/bar", "./baz"), "/foo/bar/baz");
///
/// assert_eq!(&nodejs_path::resolve!("/foo/bar", "/tmp/file/"), "/tmp/file");
///
/// assert_eq!(&nodejs_path::resolve!("/home/myself/node", "wwwroot", "static_files/png/", "../gif/image.gif"), "/home/myself/node/wwwroot/static_files/gif/image.gif");
///
/// assert_eq!(nodejs_path::resolve!("."), std::env::current_dir().unwrap().to_str().unwrap().to_owned());
///
/// assert_eq!(nodejs_path::resolve!(), std::env::current_dir().unwrap().to_str().unwrap().to_owned());
/// ```
#[macro_export]
macro_rules! resolve {
    ( $( $x:expr ),* ) => {
      {
        $crate::posix::resolve_impl(&[
          $(
            $x,
          )*
        ])
      }
    };
  }
pub use resolve;

pub fn to_namespaced_path() {}

fn posix_cwd() -> String {
    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    if cfg!(target_os = "windows") {
        // Converts Windows' backslash path separators to POSIX forward slashes
        // and truncates any drive indicator
        // const regexp = /\\/g;
        // return () => {
        //   const cwd = StringPrototypeReplace(process.cwd(), regexp, '/');
        //   return StringPrototypeSlice(cwd, StringPrototypeIndexOf(cwd, '/'));
        // };
        return cwd
            .chars()
            .map(|c| if c == '\\' { '/' } else { c })
            .take_while(|c| c != &'/')
            .collect();
    }

    // We're already on POSIX, no need for any transformations
    return cwd;
}
