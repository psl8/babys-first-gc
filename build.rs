extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_gc/gc.c")
        .opt_level(3)
        .flag("--std=c11")
        .compile("c_gc");
}
