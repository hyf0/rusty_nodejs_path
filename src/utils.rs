use std::path::{Component, Path, PathBuf};

// Copied from https://www.reddit.com/r/rust/comments/hkkquy/anyone_knows_how_to_fscanonicalize_but_without/
#[inline]
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut need_next = false;
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek() {
        need_next = true;
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };
    if need_next {
        components.next();
    }
    components.for_each(|component| match component {
        Component::Prefix(..) => unreachable!(),
        Component::RootDir => {
            ret.push(component.as_os_str());
        }
        Component::CurDir => {}
        Component::ParentDir => {
            ret.pop();
        }
        Component::Normal(c) => {
            ret.push(c);
        }
    });
    ret
}
