#[test]
fn unix() {
    assert_eq!(nodejs_path::posix::is_absolute("/home/foo"), true);
    assert_eq!(nodejs_path::posix::is_absolute("/home/foo/.."), true);
    assert_eq!(nodejs_path::posix::is_absolute("bar/"), false);
    assert_eq!(nodejs_path::posix::is_absolute("./baz"), false);
}
