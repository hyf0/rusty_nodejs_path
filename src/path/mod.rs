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
    separator: &str,
    is_path_separator: &dyn Fn(char) -> bool,
) -> String {
    let res = String::new();
    let last_segment_length = 0;
    let last_slash = -1;
    let mut code = ' ';
    let dots = 0;

    {
        let path = path.chars().collect::<Vec<char>>();
        let mut i = 0;
        let path_len = path.len();

        while i <= path_len {
            if i < path_len {
                code = *path.get(i).unwrap();
            } else if is_path_separator(code) {
                i += 1;
                break;
            } else {
                code = (47 as char)
            }
        }
    }
    path.chars()
        .into_iter()
        .enumerate()
        .for_each(|(i, code)| if !is_path_separator(code) {});

    res
}

pub mod win32 {}

pub mod posix {
    use super::{format_inner, CHAR_DOT, CHAR_FORWARD_SLASH};
    use crate::Parsed;

    pub const sep: &'static str = "/";
    pub const delimiter: &'static str = ":";

    pub fn basename(path: &str) -> String {
        parse(path).base
    }

    pub fn dirname(path: &str) -> String {
        parse(path).dir
    }

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

    pub fn normalize() {
      todo!()
    }

    pub fn parse(raw_path: &str) -> Parsed {
        let path = raw_path.chars().collect::<Vec<char>>();
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

    pub fn resolve() {
      todo!()
    }

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
