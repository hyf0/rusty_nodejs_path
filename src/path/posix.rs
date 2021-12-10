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
/// assert_eq!(nodejs_path::basename_impl("/foo/bar/baz/asdf/quux.html"), "quux.html".to_string());
/// ```
pub fn basename_impl(path: &str) -> String {
    parse(path).base
}

/// ```rust
/// assert_eq!(nodejs_path::basename_impl_without_ext("/foo/bar/baz/asdf/quux.html", ".html"), "quux".to_string());
///
/// assert_eq!(nodejs_path::basename_impl_without_ext("/foo/bar/baz/asdf/quux.HTML", ".html"), "quux.HTML".to_string());
/// ```
pub fn basename_impl_without_ext(path: &str, ext: &str) -> String {
    let mut base = parse(path).base;
    if base.ends_with(ext) {
        for _i in 0..ext.chars().collect::<Vec<char>>().len() {
            base.pop();
        }
    }
    base
}

/// ```rust
/// assert_eq!(nodejs_path::basename!("/foo/bar/baz/asdf/quux.html"), "quux.html".to_string());
///
/// assert_eq!(nodejs_path::basename!("/foo/bar/baz/asdf/quux.html", ".html"), "quux".to_string());
///
/// assert_eq!(nodejs_path::basename!("/foo/bar/baz/asdf/quux.HTML", ".html"), "quux.HTML".to_string());
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

/// ```rust
/// assert_eq!(nodejs_path::dirname("/foo/bar/baz/asdf/quux"), "/foo/bar/baz/asdf".to_string());
/// ```
pub fn dirname(path: &str) -> String {
    parse(path).dir
}

/// ```rust
/// assert_eq!(nodejs_path::extname("index.html"), ".html".to_string());
///
/// assert_eq!(nodejs_path::extname("index.coffee.md"), ".md".to_string());
///
/// assert_eq!(nodejs_path::extname("index."), ".".to_string());
///
/// assert_eq!(nodejs_path::extname("index"), "".to_string());
///
/// assert_eq!(nodejs_path::extname(".index.md"), ".md".to_string());
/// ```
pub fn extname(path: &str) -> String {
    parse(path).ext
}

pub fn format(path_object: Parsed) -> String {
    format_inner("/", path_object)
}

pub fn is_absolute(path: &str) -> bool {
    path.chars()
        .into_iter()
        .next()
        .map(|c| c == CHAR_FORWARD_SLASH)
        .unwrap_or(false)
}

pub fn join() {
    todo!()
}

pub fn normalize(path: &str) -> String {
    if path.len() == 0 {
        return ".".to_string();
    }
    todo!()
}

/// # Example
/// ```rust
/// let left = nodejs_path::parse("/home/user/dir/file.txt");
/// assert_eq!(left, nodejs_path::Parsed{
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
            println!(
                "start {} start_dot {} end {} pre_dot_state {} start_part {}",
                start, start_dot, end, pre_dot_state, start_part
            );
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

pub fn relative() {
    todo!()
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
        if resolved_path.len() > 0 {
            resolved_path
        } else {
            ".".to_owned()
        }
    }
}

/// #Example
///
/// ```rust
/// assert_eq!(nodejs_path::resolve!("/foo/bar", "./baz"), "/foo/bar/baz".to_string());
///
/// assert_eq!(nodejs_path::resolve!("/foo/bar", "/tmp/file/"), "/tmp/file".to_string());
///
/// assert_eq!(nodejs_path::resolve!("/home/myself/node", "wwwroot", "static_files/png/", "../gif/image.gif"), "/home/myself/node/wwwroot/static_files/gif/image.gif".to_string());
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
