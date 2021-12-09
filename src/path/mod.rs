// Align to https://github.com/nodejs/node/blob/master/lib/path.js

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
    pub dir: String,
    pub root: String,
    pub base: String,
    pub name: String,
    pub ext: String,
}

impl Default for Parsed {
    fn default() -> Self {
        Self {
            dir: "".to_owned(),
            root: "".to_owned(),
            base: "".to_owned(),
            name: "".to_owned(),
            ext: "".to_owned(),
        }
    }
}

fn normalize_string(
    path: &str,
    allow_above_root: bool,
    separator: &char,
    is_path_separator: &dyn Fn(&char) -> bool,
) -> String {
    let path = path.chars().collect::<Vec<char>>();

    let mut res: Vec<char> = Vec::new();
    let mut last_segment_length = 0;
    let mut last_slash = -1;
    let mut dots = 0;
    let mut code = ' ';

    {
        let mut i = 0;
        let path_len = path.len();

        while i <= path_len {
            if i < path_len {
                code = *path.get(i).unwrap();
            } else if is_path_separator(&code) {
                break;
            } else {
                code = CHAR_FORWARD_SLASH
            }

            if is_path_separator(&code) {
                if last_slash == i as i32 - 1 || dots == 1 {
                    // noop
                } else if dots == 2 {
                    if res.len() < 2
                        || last_segment_length != 2
                        || res.get(res.len() - 1).unwrap() != &CHAR_DOT
                        || res.get(res.len() - 2).unwrap() != &CHAR_DOT
                    {
                        if res.len() > 2 {
                            let last_slash_index =
                                last_index_of(&res, separator).map_or(-1, |s| s as i32);
                            if last_slash_index == -1 {
                                res = vec![];
                                last_segment_length = 0
                            } else {
                                res = res[0..last_slash_index as usize].to_vec();
                                last_segment_length = res.len() as i32
                                    - 1
                                    - last_index_of(&res, separator).map_or(-1, |s| s as i32);
                            }
                            last_slash = i as i32;
                            dots = 0;

                            i += 1;
                            continue;
                        } else if res.len() != 0 {
                            res = vec![];
                            last_segment_length = 0;
                            last_slash = i as i32;
                            dots = 0;

                            i += 1;
                            continue;
                        }
                    }
                    if allow_above_root {
                        if res.len() > 0 {
                            res.push(*separator);
                        }
                        res.push('.');
                        res.push('.');
                        last_segment_length = 2;
                    }
                } else {
                    if res.len() > 0 {
                        res.push(*separator)
                    }
                    path[(last_slash + 1) as usize..i as usize]
                        .iter()
                        .for_each(|c| res.push(*c));
                    last_segment_length = i as i32 - last_slash - 1;
                }
                last_slash = i as i32;
                dots = 0;
            } else if code == CHAR_DOT && dots != -1 {
                dots += 1;
            } else {
                dots = -1;
            }

            i += 1;
        }
    }

    res.into_iter().collect()
}

fn last_index_of(vec: &Vec<char>, tar: &char) -> Option<usize> {
    vec.iter()
        .enumerate()
        .rev()
        .find_map(|(idx, c)| if c == tar { Some(idx) } else { None })
}

pub mod win32 {
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
}

pub mod posix {
    use super::{
        format_inner, is_posix_path_separator, normalize_string, CHAR_DOT, CHAR_FORWARD_SLASH,
    };
    use crate::Parsed;

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
    /// assert_eq!(nodejs_path::basename("/foo/bar/baz/asdf/quux.html"), "quux.html".to_string());
    /// ```
    pub fn basename(path: &str) -> String {
        parse(path).base
    }

    /// ```rust
    /// assert_eq!(nodejs_path::basename_without_ext("/foo/bar/baz/asdf/quux.html", ".html"), "quux".to_string());
    ///
    /// assert_eq!(nodejs_path::basename_without_ext("/foo/bar/baz/asdf/quux.HTML", ".html"), "quux.HTML".to_string());
    /// ```
    pub fn basename_without_ext(path: &str, ext: &str) -> String {
        let mut base = parse(path).base;
        if base.ends_with(ext) {
            for _i in 0..ext.chars().collect::<Vec<char>>().len() {
                base.pop();
            }
        }
        base
    }
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

    pub fn resolve_with_array(args: &[&str]) -> String {
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
          $crate::posix::resolve_with_array(&[
            $(
              $x,
            )*
          ])
        }
      };
    }
    pub use resolve;

    pub fn to_namespaced_path() {}
}

fn format_inner(sep: &str, path_object: Parsed) -> String {
    let root = path_object.root.clone();
    let dir = if !is_empty(&path_object.dir) {
        path_object.dir
    } else {
        path_object.root
    };
    let base = if !is_empty(&path_object.base) {
        path_object.base
    } else {
        format!("{}{}", &path_object.name, &path_object.ext)
    };

    if is_empty(&dir) {
        return base;
    }

    if dir == root {
        format!("{}{}", dir, base)
    } else {
        format!("{}{}{}", dir, sep, base)
    }
}

#[inline]
fn is_empty(s: &str) -> bool {
    s.len() == 0
}

// Alphabet chars.
// const CHAR_UPPERCASE_A: char = 65; /* A */
// const CHAR_LOWERCASE_A: char = 97; /* a */
// const CHAR_UPPERCASE_Z: char = 90; /* Z */
// const CHAR_LOWERCASE_Z: char = 122; /* z */
// const CHAR_UPPERCASE_C: char = 67; /* C */
// const CHAR_LOWERCASE_B: char = 98; /* b */
// const CHAR_LOWERCASE_E: char = 101; /* e */
// const CHAR_LOWERCASE_N: char = 110; /* n */
// // Non-alphabetic chars.
const CHAR_DOT: char = '.'; /* . */
const CHAR_FORWARD_SLASH: char = '/'; /* / */
// const CHAR_BACKWARD_SLASH: char = 92; /* \ */
// const CHAR_VERTICAL_LINE: char = 124; /* | */
// const CHAR_COLON: char = 58; /* : */
// const CHAR_QUESTION_MARK: char = 63; /* ? */
// const CHAR_UNDERSCORE: char = 95; /* _ */
// const CHAR_LINE_FEED: char = 10; /* \n */
// const CHAR_CARRIAGE_RETURN: char = 13; /* \r */
// const CHAR_TAB: char = 9; /* \t */
// const CHAR_FORM_FEED: char = 12; /* \f */
// const CHAR_EXCLAMATION_MARK: char = 33; /* ! */
// const CHAR_HASH: char = 35; /* # */
// const CHAR_SPACE: char = 32; /*   */
// const CHAR_NO_BREAK_SPACE: char = 160; /* \u00A0 */
// const CHAR_ZERO_WIDTH_NOBREAK_SPACE: char = 65279; /* \uFEFF */
// const CHAR_LEFT_SQUARE_BRACKET: char = 91; /* [ */
// const CHAR_RIGHT_SQUARE_BRACKET: char = 93; /* ] */
// const CHAR_LEFT_ANGLE_BRACKET: char = 60; /* < */
// const CHAR_RIGHT_ANGLE_BRACKET: char = 62; /* > */
// const CHAR_LEFT_CURLY_BRACKET: char = 123; /* { */
// const CHAR_RIGHT_CURLY_BRACKET: char = 125; /* } */
// const CHAR_HYPHEN_MINUS: char = 45; /* - */
// const CHAR_PLUS: char = 43; /* + */
// const CHAR_DOUBLE_QUOTE: char = 34; /* " */
// const CHAR_SINGLE_QUOTE: char = 39; /* ' */
// const CHAR_PERCENT: char = 37; /* % */
// const CHAR_SEMICOLON: char = 59; /* ; */
// const CHAR_CIRCUMFLEX_ACCENT: char = 94; /* ^ */
// const CHAR_GRAVE_ACCENT: char = 96; /* ` */
// const CHAR_AT: char = 64; /* @ */
// const CHAR_AMPERSAND: char = 38; /* & */
// const CHAR_EQUAL: char = 61; /* = */
// // Digits
// const CHAR_0: char = 48; /* 0 */
// const CHAR_9: char = 57; /* 9 */
// const EOL: isWindows ? '\r\n' : '\n'

#[inline]
fn is_posix_path_separator(code: &char) -> bool {
    code == &CHAR_FORWARD_SLASH
}
