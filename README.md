# rusty_nodejs_path
Port path module (and tests) of nodejs to rust.

# Progress

- posix
  - [x] path.basename(path[, ext])
  - [x] path.delimiter
  - [x] path.dirname(path)
  - [x] path.extname(path)
  - [x] path.format(pathObject)
  - [x] path.isAbsolute(path)
  - [ ] path.join([...paths])
  - [ ] path.normalize(path)
  - [x] path.parse(path)
  - [ ] path.relative(from, to)
  - [x] path.resolve([...paths])
  - [x] path.sep
  - [ ] path.toNamespacedPath(path)
- win32
  - [ ] path.basename(path[, ext])
  - [ ] path.delimiter
  - [ ] path.dirname(path)
  - [ ] path.extname(path)
  - [ ] path.format(pathObject)
  - [ ] path.isAbsolute(path)
  - [ ] path.join([...paths])
  - [ ] path.normalize(path)
  - [ ] path.parse(path)
  - [ ] path.relative(from, to)
  - [ ] path.resolve([...paths])
  - [ ] path.sep
  - [ ] path.toNamespacedPath(path)

# Related sources

- [Path in Rust](https://doc.rust-lang.org/std/path/index.html)
- [Path in Node](https://nodejs.org/docs/latest-v16.x/api/path.html#path)
  - [Source](https://github.com/nodejs/node/blob/master/lib/path.js)
  - Test
    - [`parse()`](https://github.com/nodejs/node/blob/master/test/parallel/test-path-parse-format.js)