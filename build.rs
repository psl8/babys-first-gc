extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_gc/gc.c")
        .opt_level(3)
        .compile("c_gc.o");
}
