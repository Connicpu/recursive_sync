extern crate gcc;
use std::default::Default;

fn main() {
    gcc::compile_library(
        "libsys_mutex.a",
        &Default::default(),
        &["src/sys_mutex.c"]
    );
}
