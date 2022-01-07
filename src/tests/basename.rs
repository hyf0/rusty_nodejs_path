use crate as nodejs_path;

#[test]
fn universal() {
    assert_eq!(nodejs_path::basename!(".js", ".js"), "".to_string());
    assert_eq!(nodejs_path::basename!(""), "".to_string());
    assert_eq!(
        nodejs_path::basename!("/dir/basename.ext"),
        "basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::basename!("/basename.ext"),
        "basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::basename!("basename.ext"),
        "basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::basename!("basename.ext/"),
        "basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::basename!("basename.ext//"),
        "basename.ext".to_string()
    );
    assert_eq!(nodejs_path::basename!("aaa/bbb", "/bbb"), "bbb".to_string());
    assert_eq!(
        nodejs_path::basename!("aaa/bbb", "a/bbb"),
        "bbb".to_string()
    );
    assert_eq!(nodejs_path::basename!("aaa/bbb", "bbb"), "bbb".to_string());
    assert_eq!(
        nodejs_path::basename!("aaa/bbb//", "bbb"),
        "bbb".to_string()
    );
    assert_eq!(nodejs_path::basename!("aaa/bbb", "bb"), "b".to_string());
    assert_eq!(nodejs_path::basename!("aaa/bbb", "b"), "bb".to_string());
    assert_eq!(
        nodejs_path::basename!("/aaa/bbb", "/bbb"),
        "bbb".to_string()
    );
    assert_eq!(
        nodejs_path::basename!("/aaa/bbb", "a/bbb"),
        "bbb".to_string()
    );
    assert_eq!(nodejs_path::basename!("/aaa/bbb", "bbb"), "bbb".to_string());
    assert_eq!(
        nodejs_path::basename!("/aaa/bbb//", "bbb"),
        "bbb".to_string()
    );
    assert_eq!(nodejs_path::basename!("/aaa/bbb", "bb"), "b".to_string());
    assert_eq!(nodejs_path::basename!("/aaa/bbb", "b"), "bb".to_string());
    assert_eq!(nodejs_path::basename!("/aaa/bbb"), "bbb".to_string());
    assert_eq!(nodejs_path::basename!("/aaa/"), "aaa".to_string());
    assert_eq!(nodejs_path::basename!("/aaa/b"), "b".to_string());
    assert_eq!(nodejs_path::basename!("/a/b"), "b".to_string());
    assert_eq!(nodejs_path::basename!("//a"), "a".to_string());
    assert_eq!(nodejs_path::basename!("a", "a"), "".to_string());
}

#[test]
fn unix() {
    // On unix a backslash is just treated as any other character.
    assert_eq!(
        nodejs_path::posix::basename!("\\dir\\basename.ext"),
        "\\dir\\basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::posix::basename!("\\basename.ext"),
        "\\basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::posix::basename!("basename.ext"),
        "basename.ext".to_string()
    );
    assert_eq!(
        nodejs_path::posix::basename!("basename.ext\\"),
        "basename.ext\\".to_string()
    );
    assert_eq!(
        nodejs_path::posix::basename!("basename.ext\\\\"),
        "basename.ext\\\\".to_string()
    );
    assert_eq!(nodejs_path::posix::basename!("foo"), "foo".to_string());

    // // POSIX filenames may include control characters
    // // c.f. http://www.dwheeler.com/essays/fixing-unix-linux-filenames.html
    let mut control_char_filename = "Icon".to_owned();
    control_char_filename.push(13 as char);
    assert_eq!(
        nodejs_path::posix::basename!(&("/a/b/".to_owned() + &control_char_filename)),
        control_char_filename
    );
}
