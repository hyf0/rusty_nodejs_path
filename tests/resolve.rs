use nodejs_path::posix;

fn posixy_cwd() -> String {
    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    if cfg!(target_os = "windows") {
        return cwd
            .chars()
            .map(|c| if c == '\\' { '/' } else { c })
            .take_while(|c| c != &'/')
            .collect();
    }

    return cwd;
}

#[test]
fn posix_test() {
    // nodejs_path::resolve_with_array(&[]);
    let tests = [
        (vec!["/var/lib", "../", "file/"], "/var/file"),
        (vec!["/var/lib", "/../", "file/"], "/file"),
        (vec!["a/b/c/", "../../.."], &posixy_cwd()),
        (vec!["."], &posixy_cwd()),
        (vec!["/some/dir", ".", "/absolute/"], "/absolute"),
        (
            vec!["/foo/tmp.3/", "../tmp.3/cycles/root.js"],
            "/foo/tmp.3/cycles/root.js",
        ),
    ];

    tests.iter().for_each(|(input, right)| {
        assert_eq!(posix::resolve_impl(&input), *right);
    });

    assert_eq!(posix::resolve!("/var/lib", "../", "file/"), "/var/file");
    assert_eq!(posix::resolve!("/var/lib", "/../", "file/"), "/file");
    assert_eq!(&posix::resolve!("a/b/c/", "../../.."), &posixy_cwd());
    assert_eq!(&posix::resolve!("."), &posixy_cwd());
    assert_eq!(posix::resolve!("/some/dir", ".", "/absolute/"), "/absolute");
    assert_eq!(
        posix::resolve!("/foo/tmp.3/", "../tmp.3/cycles/root.js"),
        "/foo/tmp.3/cycles/root.js"
    );
}
