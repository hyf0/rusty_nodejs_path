use criterion::{criterion_group, criterion_main, Criterion};
use nodejs_path::{posix};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("join_impl", |b| {
        let join_tests = [
            (vec![".", "x/b", "..", "/b/c.js"], "x/b/c.js"),
            (vec![], "."),
            (vec!["/.", "x/b", "..", "/b/c.js"], "/x/b/c.js"),
            (vec!["/foo", "../../../bar"], "/bar"),
            (vec!["foo", "../../../bar"], "../../bar"),
            (vec!["foo/", "../../../bar"], "../../bar"),
            (vec!["foo/x", "../../../bar"], "../bar"),
            (vec!["foo/x", "./bar"], "foo/x/bar"),
            (vec!["foo/x/", "./bar"], "foo/x/bar"),
            (vec!["foo/x/", ".", "bar"], "foo/x/bar"),
            (vec!["./"], "./"),
            (vec![".", "./"], "./"),
            (vec![".", ".", "."], "."),
            (vec![".", "./", "."], "."),
            (vec![".", "/./", "."], "."),
            (vec![".", "/////./", "."], "."),
            (vec!["."], "."),
            (vec!["", "."], "."),
            (vec!["", "foo"], "foo"),
            (vec!["foo", "/bar"], "foo/bar"),
            (vec!["", "/foo"], "/foo"),
            (vec!["", "", "/foo"], "/foo"),
            (vec!["", "", "foo"], "foo"),
            (vec!["foo", ""], "foo"),
            (vec!["foo/", ""], "foo/"),
            (vec!["foo", "", "/bar"], "foo/bar"),
            (vec!["./", "..", "/foo"], "../foo"),
            (vec!["./", "..", "..", "/foo"], "../../foo"),
            (vec![".", "..", "..", "/foo"], "../../foo"),
            (vec!["", "..", "..", "/foo"], "../../foo"),
            (vec!["/"], "/"),
            (vec!["/", "."], "/"),
            (vec!["/", ".."], "/"),
            (vec!["/", "..", ".."], "/"),
            (vec![""], "."),
            (vec!["", ""], "."),
            (vec![" /foo"], " /foo"),
            (vec![" ", "foo"], " /foo"),
            (vec![" ", "."], " "),
            (vec![" ", "/"], " /"),
            (vec![" ", ""], " "),
            (vec!["/", "foo"], "/foo"),
            (vec!["/", "/foo"], "/foo"),
            (vec!["/", "//foo"], "/foo"),
            (vec!["/", "", "/foo"], "/foo"),
            (vec!["", "/", "foo"], "/foo"),
            (vec!["", "/", "/foo"], "/foo"),
        ];
        b.iter(|| {
            for join_seq in &join_tests {
                let _res = posix::join_impl(&join_seq.0);
            }
        })
    });
    c.bench_function("cwd", |b| {
        b.iter(|| {
          posix::resolve!();
      })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
