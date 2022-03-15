
use crate as nodejs_path;

#[cfg(target_family = "unix")]
#[test]
fn posix() {
    assert_eq!(&nodejs_path::posix::normalize("/foo/../../../bar"), "/bar");
    assert_eq!(&nodejs_path::posix::normalize("a//b//../b"), "a/b");
    assert_eq!(&nodejs_path::posix::normalize("a//b//./c"), "a/b/c");
    assert_eq!(&nodejs_path::posix::normalize("a//b//."), "a/b");
    assert_eq!(
        &nodejs_path::posix::normalize("/a/b/c/../../../x/y/z"),
        "/x/y/z"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("///..//./foo/.//bar"),
        "/foo/bar"
    );
    assert_eq!(&nodejs_path::posix::normalize("bar/foo../../"), "bar/");
    assert_eq!(&nodejs_path::posix::normalize("bar/foo../.."), "bar");
    assert_eq!(
        &nodejs_path::posix::normalize("bar/foo../../baz"),
        "bar/baz"
    );
    assert_eq!(&nodejs_path::posix::normalize("bar/foo../"), "bar/foo../");
    assert_eq!(&nodejs_path::posix::normalize("bar/foo.."), "bar/foo..");
    assert_eq!(
        &nodejs_path::posix::normalize("../foo../../../bar"),
        "../../bar"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("../.../.././.../../../bar"),
        "../../bar"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("../../../foo/../../../bar"),
        "../../../../../bar"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("../../../foo/../../../bar/../../"),
        "../../../../../../"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("../foobar/barfoo/foo/../../../bar/../../"),
        "../../"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("../.../../foobar/../../../bar/../../baz"),
        "../../../../baz"
    );
    assert_eq!(
        &nodejs_path::posix::normalize("foo/bar\\baz"),
        "foo/bar\\baz"
    );

    assert_eq!(&nodejs_path::posix::normalize(""), ".");
}
