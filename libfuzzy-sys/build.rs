// ssdeep-rs: A Rust wrapper for ssdeep.
//
// Copyright (c) 2016 Petr Zemek <s3rvac@petrzemek.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate cc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let cfg = cc::Build::new();
    let compiler = cfg.get_compiler();
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    info(&format!("src dir: {}", src.display()));
    info(&format!("dst dir: {}", dst.display()));

    println!("cargo:rustc-link-lib=static=fuzzy");
    println!("cargo:rustc-link-search={}/.libs", dst.display());

    let _ = fs::create_dir(&dst);

    let mut cflags = OsString::new();
    for arg in compiler.args() {
        cflags.push(arg);
        cflags.push(" ");
    }
    // Without -fPIC, build on Linux fails with the following link error:
    // "relocation R_X86_64_32 against `.rodata' can not be used when making a
    // shared object; recompile with -fPIC"
    cflags.push("-fPIC");

    let mut cxxflags = OsString::new();
    for arg in compiler.args() {
        cxxflags.push(arg);
        cxxflags.push(" ");
    }

    let mut configure_command = Command::new(&src.join("libfuzzy/configure"));
    configure_command
        .arg("--enable-shared=no")
        .arg("--enable-static=yes");
    if cfg!(target_pointer_width = "32") {
        configure_command
            .arg("--build=x86_64-pc-linux-gnu")
            .arg("--target=i586-pc-linux-gnu");
    }
    configure_command
        .env("CFLAGS", cflags)
        .env("CXXFLAGS", cxxflags)
        .current_dir(&dst);

    run(&mut configure_command);


    run(Command::new("make")
        .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
        // We do not want `make` to rebuild any autotools-related files, which
        // might happen if the file timestamps get messed up (this can happen
        // when using Git). Since the upstream configure script from ssdeep is
        // old and does not support AM_MAINTAINER_MODE (or parameters
        // --enable-maintainer-mode/--disable-maintainer-mode), we need to use
        // a workaround based on https://stackoverflow.com/a/5745366/2580955.
        // This fixes `aclocal-1.15: command not found` errors during the build.
        .arg("AUTOCONF=:")
        .arg("AUTOHEADER=:")
        .arg("AUTOMAKE=: ")
        .arg("ACLOCAL=:")
        .current_dir(&dst));
}

fn run(cmd: &mut Command) {
    info(&format!("running command: {:?}", cmd));
    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully: {}", status));
    }
    info(&format!("command finished: {}", status));
}

fn info(msg: &str) {
    println!("INFO: {}", msg);
}

fn fail(reason: &str) -> ! {
    panic!("FAIL: {}\n\nbuild script failed", reason)
}
