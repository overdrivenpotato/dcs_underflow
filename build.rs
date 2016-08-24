extern crate diesel_codegen_syntex;
extern crate syntex;

use std::env;
use std::path::Path;

fn main() {
    let mut registry = syntex::Registry::new();

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/main.rs");
    let dst = Path::new(&out_dir).join("main.rs");

    diesel_codegen_syntex::register(&mut registry);

    registry.expand("syntex_underflow", &src, &dst).unwrap();
}
