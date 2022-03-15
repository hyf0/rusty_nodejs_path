use crate as nodejs_path;



#[test]
fn unix() {
    assert_eq!(nodejs_path::posix::is_absolute("/home/foo"), true);
    assert_eq!(nodejs_path::posix::is_absolute("/home/foo/.."), true);
    assert_eq!(nodejs_path::posix::is_absolute("bar/"), false);
    assert_eq!(nodejs_path::posix::is_absolute("./baz"), false);
}

#[test]
fn windows() {
    assert_eq!(nodejs_path::win32::is_absolute("/"), true);
    assert_eq!(nodejs_path::win32::is_absolute("//"), true);
    assert_eq!(nodejs_path::win32::is_absolute("//server"), true);
    assert_eq!(nodejs_path::win32::is_absolute("//server/file"), true);
    assert_eq!(nodejs_path::win32::is_absolute("\\\\server\\file"), true);
    assert_eq!(nodejs_path::win32::is_absolute("\\\\server"), true);
    assert_eq!(nodejs_path::win32::is_absolute("\\\\"), true);
    assert_eq!(nodejs_path::win32::is_absolute("c"), false);
    assert_eq!(nodejs_path::win32::is_absolute("c:"), false);
    assert_eq!(nodejs_path::win32::is_absolute("c:\\"), true);
    assert_eq!(nodejs_path::win32::is_absolute("c:/"), true);
    assert_eq!(nodejs_path::win32::is_absolute("c://"), true);
    assert_eq!(nodejs_path::win32::is_absolute("C:/Users/"), true);
    assert_eq!(nodejs_path::win32::is_absolute("C:\\Users\\"), true);
    assert_eq!(nodejs_path::win32::is_absolute("C:cwd/another"), false);
    assert_eq!(nodejs_path::win32::is_absolute("C:cwd\\another"), false);
    assert_eq!(
        nodejs_path::win32::is_absolute("directory/directory"),
        false
    );
    assert_eq!(
        nodejs_path::win32::is_absolute("directory\\directory"),
        false
    );
}
