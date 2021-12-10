use nodejs_path::{
    posix::{self as posix},
    Parsed,
};

fn check_parse_format_for_posix(paths: &[(&str, &str)]) {
    paths.iter().for_each(|(element, root)| {
        let output = posix::parse(element);
        println!("{:#?} for {:?}", output, element);
        assert_eq!(&posix::format(output.clone()), element);
        assert_eq!(&output.root, root);
        assert!(output.dir.starts_with(&output.root));
        assert_eq!(
            output.dir,
            if output.dir.len() != 0 {
                posix::dirname(element)
            } else {
                "".to_owned()
            }
        );
        assert_eq!(output.base, posix::basename!(element));
        assert_eq!(output.ext, posix::extname(element));
    });
}

fn check_format_for_posix(test_cases: &[(Parsed, String)]) {
    use posix as path;
    test_cases.iter().for_each(|(input, right)| {
        println!("input {:#?}", input);
        assert_eq!(&path::format(input.clone()), right);
    });
}
// { root: '/', dir: '/', base: '', ext: '', name: '' }

#[test]
fn posix_tests() {
    let win_paths = vec![
        // [path, root]
        vec!["C:\\path\\dir\\index.html", "C:\\"],
        vec!["C:\\another_path\\DIR\\1\\2\\33\\\\index", "C:\\"],
        vec!["another_path\\DIR with spaces\\1\\2\\33\\index", ""],
        vec!["\\", "\\"],
        vec!["\\foo\\C:", "\\"],
        vec!["file", ""],
        vec!["file:stream", ""],
        vec![".\\file", ""],
        vec!["C:", "C:"],
        vec!["C:.", "C:"],
        vec!["C:..", "C:"],
        vec!["C:abc", "C:"],
        vec!["C:\\", "C:\\"],
        vec!["C:\\abc", "C:\\"],
        vec!["", ""],
        // unc
        vec!["\\\\server\\share\\file_path", "\\\\server\\share\\"],
        vec![
            "\\\\server two\\shared folder\\file path.zip",
            "\\\\server two\\shared folder\\",
        ],
        vec!["\\\\teela\\admin$\\system32", "\\\\teela\\admin$\\"],
        vec!["\\\\?\\UNC\\server\\share", "\\\\?\\UNC\\"],
    ];

    // let winSpecialCaseParseTests = vec![
    //   vec!["t", { base: "t", name: "t", root: "", dir: "", ext: "" }],
    //   vec!["/foo/bar", { root: "/", dir: "/foo", base: "bar", ext: "", name: "bar" }],
    // ];

    // let winSpecialCaseFormatTests = vec![
    //   vec![{ dir: "some\\dir" }, "some\\dir\\"],
    //   vec![{ base: "index.html" }, "index.html"],
    //   vec![{ root: "C:\\" }, "C:\\"],
    //   vec![{ name: "index", ext: ".html" }, "index.html"],
    //   vec![{ dir: "some\\dir", name: "index", ext: ".html" }, "some\\dir\\index.html"],
    //   vec![{ root: "C:\\", name: "index", ext: ".html" }, "C:\\index.html"],
    //   vec![{}, ""],
    // ];

    let unix_paths = [
        // [path, root]
        ("/home/user/dir/file.txt", "/"),
        ("/home/user/a dir/another File.zip", "/"),
        ("/home/user/a dir//another&File.", "/"),
        ("/home/user/a$$$dir//another File.zip", "/"),
        ("user/dir/another File.zip", ""),
        ("file", ""),
        (".\\file", ""),
        ("./file", ""),
        ("C:\\foo", ""),
        ("/", "/"),
        ("", ""),
        (".", ""),
        ("..", ""),
        ("/foo", "/"),
        ("/foo.", "/"),
        ("/foo.bar", "/"),
        ("/.", "/"),
        ("/.foo", "/"),
        ("/.foo.bar", "/"),
        ("/foo/bar.baz", "/"),
    ];

    check_parse_format_for_posix(&unix_paths);

    let unix_special_case_format_tests = [
        (
            Parsed {
                dir: "some/dir".to_owned(),
                ..Parsed::default()
            },
            "some/dir/".to_owned(),
        ),
        (
            Parsed {
                base: "index.html".to_owned(),
                ..Parsed::default()
            },
            "index.html".to_owned(),
        ),
        (
            Parsed {
                root: "/".to_owned(),
                ..Parsed::default()
            },
            "/".to_owned(),
        ),
        (
            Parsed {
                name: "index".to_owned(),
                ext: ".html".to_owned(),
                ..Parsed::default()
            },
            "index.html".to_owned(),
        ),
        (
            Parsed {
                dir: "some/dir".to_owned(),
                name: "index".to_owned(),
                ext: ".html".to_owned(),
                ..Parsed::default()
            },
            "some/dir/index.html".to_owned(),
        ),
        (
            Parsed {
                root: "/".to_owned(),
                name: "index".to_owned(),
                ext: ".html".to_owned(),
                ..Parsed::default()
            },
            "/index.html".to_owned(),
        ),
        (
            Parsed {
                ..Parsed::default()
            },
            "".to_owned(),
        ),
    ];

    check_format_for_posix(&unix_special_case_format_tests);

    // let errors = [
    //   { method: "parse", input: [null] },
    //   { method: "parse", input: [{}] },
    //   { method: "parse", input: [true] },
    //   { method: "parse", input: [1] },
    //   { method: "parse", input: [] },
    //   { method: "format", input: [null] },
    //   { method: "format", input: [""] },
    //   { method: "format", input: [true] },
    //   { method: "format", input: [1] },
    // ];

    // checkParseFormat(path.win32, winPaths);
    // checkParseFormat(path.posix, unixPaths);
    // checkSpecialCaseParseFormat(path.win32, winSpecialCaseParseTests);
    // checkErrors(path.win32);
    // checkErrors(path.posix);
    // checkFormat(path.win32, winSpecialCaseFormatTests);
    // checkFormat(path.posix, unixSpecialCaseFormatTests);

    // Test removal of trailing path separators
    // let trailingTestsWin32 = [[".\\", { root: "", dir: "", base: ".", ext: "", name: "." }],
    // ["\\\\", { root: "\\", dir: "\\", base: "", ext: "", name: "" }],
    // ["\\\\", { root: "\\", dir: "\\", base: "", ext: "", name: "" }],
    // ["c:\\foo\\\\\\",
    //  { root: "c:\\", dir: "c:\\", base: "foo", ext: "", name: "foo" }],
    // ["D:\\foo\\\\\\bar.baz",
    //  { root: "D:\\",
    //    dir: "D:\\foo\\\\",
    //    base: "bar.baz",
    //    ext: ".baz",
    //    name: "bar" },
    // ]];
    let trailing_tests_posix = [
        (
            "./".to_owned(),
            Parsed {
                root: "".to_owned(),
                dir: "".to_owned(),
                base: ".".to_owned(),
                ext: "".to_owned(),
                name: ".".to_owned(),
            },
        ),
        (
            "//".to_owned(),
            Parsed {
                root: "/".to_owned(),
                dir: "/".to_owned(),
                base: "".to_owned(),
                ext: "".to_owned(),
                name: "".to_owned(),
            },
        ),
        (
            "///".to_owned(),
            Parsed {
                root: "/".to_owned(),
                dir: "/".to_owned(),
                base: "".to_owned(),
                ext: "".to_owned(),
                name: "".to_owned(),
            },
        ),
        (
            "/foo///".to_owned(),
            Parsed {
                root: "/".to_owned(),
                dir: "/".to_owned(),
                base: "foo".to_owned(),
                ext: "".to_owned(),
                name: "foo".to_owned(),
            },
        ),
        (
            "/foo///bar.baz".to_owned(),
            Parsed {
                root: "/".to_owned(),
                dir: "/foo//".to_owned(),
                base: "bar.baz".to_owned(),
                ext: ".baz".to_owned(),
                name: "bar".to_owned(),
            },
        ),
    ];

    trailing_tests_posix.iter().for_each(|(input, right)| {
        let left = nodejs_path::posix::parse(&input);
        assert_eq!(&left, right);
    })

    // let failures = [];
    // trailingTests.forEach((test) => {
    //   let parse = test[0];
    //   let os = parse === path.win32.parse ? "win32" : "posix";
    //   test[1].forEach((test) => {
    //     let actual = parse(test[0]);
    //     let expected = test[1];
    //     let message = `path.${os}.parse(${JSON.stringify(test[0])})\n  expect=${
    //       JSON.stringify(expected)}\n  actual=${JSON.stringify(actual)}`;
    //     let actualKeys = Object.keys(actual);
    //     let expectedKeys = Object.keys(expected);
    //     let failed = (actualKeys.length !== expectedKeys.length);
    //     if (!failed) {
    //       for (let i = 0; i < actualKeys.length; ++i) {
    //         let key = actualKeys[i];
    //         if (!expectedKeys.includes(key) || actual[key] !== expected[key]) {
    //           failed = true;
    //           break;
    //         }
    //       }
    //     }
    //     if (failed)
    //       failures.push(`\n${message}`);
    //   });
    // });
    // assert.strictEqual(failures.length, 0, failures.join(""));

    // function checkErrors(path) {
    //   errors.forEach(({ method, input }) => {
    //     assert.throws(() => {
    //       path[method].apply(path, input);
    //     }, {
    //       code: "ERR_INVALID_ARG_TYPE",
    //       name: "TypeError"
    //     });
    //   });
    // }

    // function checkParseFormat(path, paths) {
    //   paths.forEach(([element, root]) => {
    //     let output = path.parse(element);
    //     assert.strictEqual(typeof output.root, "string");
    //     assert.strictEqual(typeof output.dir, "string");
    //     assert.strictEqual(typeof output.base, "string");
    //     assert.strictEqual(typeof output.ext, "string");
    //     assert.strictEqual(typeof output.name, "string");
    //     assert.strictEqual(path.format(output), element);
    //     assert.strictEqual(output.root, root);
    //     assert(output.dir.startsWith(output.root));
    //     assert.strictEqual(output.dir, output.dir ? path.dirname(element) : "");
    //     assert.strictEqual(output.base, path.basename(element));
    //     assert.strictEqual(output.ext, path.extname(element));
    //   });
    // }

    // function checkSpecialCaseParseFormat(path, testCases) {
    //   testCases.forEach(([element, expect]) => {
    //     assert.deepStrictEqual(path.parse(element), expect);
    //   });
    // }

    // function checkFormat(path, testCases) {
    //   testCases.forEach(([element, expect]) => {
    //     assert.strictEqual(path.format(element), expect);
    //   });

    //   [null, undefined, 1, true, false, "string"].forEach((pathObject) => {
    //     assert.throws(() => {
    //       path.format(pathObject);
    //     }, {
    //       code: "ERR_INVALID_ARG_TYPE",
    //       name: "TypeError",
    //       message: "The "pathObject" argument must be of type object." +
    //                common.invalidArgTypeHelper(pathObject)
    //     });
    //   });
    // }
}
