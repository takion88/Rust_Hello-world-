extern crate cc;

fn main() {
    cc::Build::new()
    .flag("-C")
    .file("src/syscall.S")
    .compile("libsyscall.a");
}
