extern crate cheddar;

fn main() {
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("capi").expect("could not find module")
        .run_build("include/my_header.h");
}