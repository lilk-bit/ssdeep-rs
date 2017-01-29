Rust Wrapper for ssdeep
=======================

[![crates.io](https://img.shields.io/crates/v/ssdeep.svg)](https://crates.io/crates/ssdeep)

A Rust wrapper for [ssdeep by Jesse Kornblum](http://ssdeep.sourceforge.net/),
which is a C library for computing [context triggered piecewise
hashes](http://dfrws.org/2006/proceedings/12-Kornblum.pdf) (CTPH). Also called
fuzzy hashes, CTPH can match inputs that have homologies. Such inputs have
sequences of identical bytes in the same order, although bytes in between these
sequences may be different in both content and length. In contrast to standard
hashing algorithms, CTPH can be used to identify files that are highly similar
but not identical. See [my blog
post](https://blog.petrzemek.net/2016/11/01/computing-context-triggered-piecewise-hashes-in-rust/)
for more details.

Usage
-----

To compute the fuzzy hash of a given buffer, use the `hash()` function:

```rust
extern crate ssdeep;

let h = ssdeep::hash(b"Hello there!").unwrap();
assert_eq!(h, "3:aNRn:aNRn");
```

If you want to obtain the fuzzy hash of a file, you can use `hash_from_file()`:

```rust
let h = ssdeep::hash_from_file("path/to/file").unwrap();
```

To compare two fuzzy hashes, use `compare()`, which returns an integer between
0 (no match) and 100:

```rust
let h1 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
let h2 = b"3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
let score = ssdeep::compare(h1, h2).unwrap();
assert_eq!(score, 22);
```

Each of these functions return an
[`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), where `None`
is returned when the underlying C function fails.

Installation
------------

Add the following lines into your `Cargo.toml` file:

```
[dependencies]
ssdeep = "0.1.0"
```

Then, when you run `cargo build`, it will automatically get the wrapper's
source code from [crates.io](https://crates.io/), compile the underlying C
library, and build the wrapper. The C library is statically linked into the
wrapper.

The build process is known to work under Linux with GCC. If you have a
different operating system or compiler and the build fails, feel free to
[submit a pull request](https://github.com/s3rvac/ssdeep-rs/pulls) or [open an
issue](https://github.com/s3rvac/ssdeep-rs/issues).

Documentation
-------------

An automatically generated API documentation is available here:

* [latest](https://projects.petrzemek.net/ssdeep-rs/doc/latest/ssdeep/)
* [0.1.0](https://projects.petrzemek.net/ssdeep-rs/doc/0.1.0/ssdeep/)

License
-------

The code is licensed under the terms of GPLv3.

This wrapper includes the unchanged source distribution of [ssdeep version
2.13](http://ssdeep.sourceforge.net/changes.txt), which is compiled and
statically linked into the wrapper during build. It is licensed under GPLv2.
