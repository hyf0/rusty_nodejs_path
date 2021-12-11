#[test]
fn unix() {
    let cases = [
        ("/var/lib", "/var", ".."),
        ("/var/lib", "/bin", "../../bin"),
        ("/var/lib", "/var/lib", ""),
        ("/var/lib", "/var/apache", "../apache"),
        ("/var/", "/var/lib", "lib"),
        ("/", "/var/lib", "var/lib"),
        (
            "/foo/test",
            "/foo/test/bar/package.json",
            "bar/package.json",
        ),
        ("/Users/a/web/b/test/mails", "/Users/a/web/b", "../.."),
        ("/foo/bar/baz-quux", "/foo/bar/baz", "../baz"),
        ("/foo/bar/baz", "/foo/bar/baz-quux", "../baz-quux"),
        ("/baz-quux", "/baz", "../baz"),
        ("/baz", "/baz-quux", "../baz-quux"),
        ("/page1/page2/foo", "/", "../../.."),
    ];

    cases.into_iter().for_each(|(from, to, right)| {
        assert_eq!(
            nodejs_path::posix::relative(from, to),
            right,
            "for input from: {} to: {}",
            from,
            to
        );
    })
}
