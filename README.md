# rusty_nodejs_path
Port path module (and tests) of nodejs to rust using the same algorithms.

- [crates.io](https://crates.io/crates/nodejs_path)
- [Documents](https://docs.rs/nodejs_path/latest/nodejs_path/)

# Progress

- posix
  - [x] [path.basename(path[, ext])](https://docs.rs/nodejs_path/latest/nodejs_path/posix/macro.basename.html)
  - [x] [path.delimiter](https://docs.rs/nodejs_path/latest/nodejs_path/posix/constant.delimiter.html)
  - [x] [path.dirname(path)](https://docs.rs/nodejs_path/latest/nodejs_path/posix/fn.dirname.html)
  - [x] [path.extname(path)](https://docs.rs/nodejs_path/latest/nodejs_path/posix/fn.extname.html)
  - [x] [path.format(pathObject)](https://docs.rs/nodejs_path/latest/nodejs_path/posix/fn.format.html)
  - [x] [path.isAbsolute(path)](https://docs.rs/nodejs_path/latest/nodejs_path/posix/fn.is_absolute.html)
  - [x] [path.join([...paths])](https://docs.rs/nodejs_path/latest/nodejs_path/macro.join.html)
  - [x] [path.normalize(path)](https://docs.rs/nodejs_path/latest/nodejs_path/posix/fn.normalize.html)
  - [x] [path.parse(path)](https://docs.rs/nodejs_path/latest/nodejs_path/posix/fn.parse.html)
  - [x] [path.relative(from, to)](https://docs.rs/nodejs_path/latest/nodejs_path/fn.relative.html)
  - [x] [path.resolve([...paths])](https://docs.rs/nodejs_path/latest/nodejs_path/posix/macro.resolve.html)
  - [x] [path.sep](https://docs.rs/nodejs_path/latest/nodejs_path/posix/constant.sep.html)
  - [ ] ~~path.toNamespacedPath(path)~~
- win32
  - [ ] path.basename(path[, ext])
  - [x] [path.delimiter](https://docs.rs/nodejs_path/latest/nodejs_path/win32/constant.delimiter.html)
  - [ ] path.dirname(path)
  - [ ] path.extname(path)
  - [ ] path.format(pathObject)
  - [x] path.isAbsolute(path)
  - [ ] path.join([...paths])
  - [ ] path.normalize(path)
  - [ ] path.parse(path)
  - [ ] path.relative(from, to)
  - [ ] path.resolve([...paths])
  - [x] [path.sep](https://docs.rs/nodejs_path/latest/nodejs_path/win32/constant.sep.html)
  - [ ] ~~path.toNamespacedPath(path)~~

# Related sources

- [Path in Rust](https://doc.rust-lang.org/std/path/index.html)
- [Path in Node](https://nodejs.org/docs/latest-v16.x/api/path.html#path)
  - [Source](https://github.com/nodejs/node/blob/master/lib/path.js)
  - Test
    - [`parse()`](https://github.com/nodejs/node/blob/master/test/parallel/test-path-parse-format.js)
