#[test]
fn unix() {
    // On *nix, backslash is a valid name component like any other character.
    assert_eq!(&nodejs_path::posix::extname(".\\"), "");
    assert_eq!(&nodejs_path::posix::extname("..\\"), ".\\");
    assert_eq!(&nodejs_path::posix::extname("file.ext\\"), ".ext\\");
    assert_eq!(&nodejs_path::posix::extname("file.ext\\\\"), ".ext\\\\");
    assert_eq!(&nodejs_path::posix::extname("file\\"), "");
    assert_eq!(&nodejs_path::posix::extname("file\\\\"), "");
    assert_eq!(&nodejs_path::posix::extname("file.\\"), ".\\");
    assert_eq!(&nodejs_path::posix::extname("file.\\\\"), ".\\\\");
}

#[test]
fn basic() {
    let cases = [
        // (__filename, ".js"),
        ("", ""),
        ("/path/to/file", ""),
        ("/path/to/file.ext", ".ext"),
        ("/path.to/file.ext", ".ext"),
        ("/path.to/file", ""),
        ("/path.to/.file", ""),
        ("/path.to/.file.ext", ".ext"),
        ("/path/to/f.ext", ".ext"),
        ("/path/to/..ext", ".ext"),
        ("/path/to/..", ""),
        ("file", ""),
        ("file.ext", ".ext"),
        (".file", ""),
        (".file.ext", ".ext"),
        ("/file", ""),
        ("/file.ext", ".ext"),
        ("/.file", ""),
        ("/.file.ext", ".ext"),
        (".path/file.ext", ".ext"),
        ("file.ext.ext", ".ext"),
        ("file.", "."),
        (".", ""),
        ("./", ""),
        (".file.ext", ".ext"),
        (".file", ""),
        (".file.", "."),
        (".file..", "."),
        ("..", ""),
        ("../", ""),
        ("..file.ext", ".ext"),
        ("..file", ".file"),
        ("..file.", "."),
        ("..file..", "."),
        ("...", "."),
        ("...ext", ".ext"),
        ("....", "."),
        ("file.ext/", ".ext"),
        ("file.ext//", ".ext"),
        ("file/", ""),
        ("file//", ""),
        ("file./", "."),
        ("file.//", "."),
    ];

    cases.into_iter().for_each(|(input, right)| {
        if cfg!(target_family = "unix") {
            assert_eq!(&nodejs_path::posix::extname(input), right);
        }
    })
}
