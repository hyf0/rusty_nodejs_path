use crate as nodejs_path;

#[test]
fn test_posix() {
    assert_eq!(nodejs_path::posix::dirname("/a/b/"), "/a".to_string());
    assert_eq!(nodejs_path::posix::dirname("/a/b"), "/a".to_string());
    assert_eq!(nodejs_path::posix::dirname("/a"), "/".to_string());
    assert_eq!(nodejs_path::posix::dirname(""), ".".to_string());
    assert_eq!(nodejs_path::posix::dirname("/"), "/".to_string());
    assert_eq!(nodejs_path::posix::dirname("////"), "/".to_string());
    assert_eq!(nodejs_path::posix::dirname("//a"), "//".to_string());
    assert_eq!(nodejs_path::posix::dirname("foo"), ".".to_string());
}
