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

pub(crate) fn normalize_string(
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

    res.into_iter().collect()
}

fn last_index_of(vec: &Vec<char>, tar: &char) -> Option<usize> {
    vec.iter()
        .enumerate()
        .rev()
        .find_map(|(idx, c)| if c == tar { Some(idx) } else { None })
}

#[inline]
pub(crate) fn is_empty(s: &str) -> bool {
    s.len() == 0
}

pub(crate) fn format_inner(sep: &str, path_object: Parsed) -> String {
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
pub(crate) fn is_posix_path_separator(code: &char) -> bool {
    code == &CHAR_FORWARD_SLASH
}

#[inline]
pub(crate) fn is_path_separator(code: &char) -> bool {
    code == &CHAR_FORWARD_SLASH || code == &CHAR_BACKWARD_SLASH
}

// Alphabet chars.
pub(crate) const CHAR_UPPERCASE_A: char = 'A'; /* A */
pub(crate) const CHAR_LOWERCASE_A: char = 'a'; /* a */
pub(crate) const CHAR_UPPERCASE_Z: char = 'Z'; /* Z */
pub(crate) const CHAR_LOWERCASE_Z: char = 'z'; /* z */
// const CHAR_UPPERCASE_C: char = 67; /* C */
// const CHAR_LOWERCASE_B: char = 98; /* b */
// const CHAR_LOWERCASE_E: char = 101; /* e */
// const CHAR_LOWERCASE_N: char = 110; /* n */
// // Non-alphabetic chars.
pub(crate) const CHAR_DOT: char = '.'; /* . */
pub(crate) const CHAR_FORWARD_SLASH: char = '/'; /* / */
const CHAR_BACKWARD_SLASH: char = '\\'; /* \ */
// const CHAR_VERTICAL_LINE: char = 124; /* | */
pub(crate) const CHAR_COLON: char = ':'; /* : */
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
