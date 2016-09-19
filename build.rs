extern crate gcc;

fn main() {
  gcc::Config::new()
    .file("src/hello.c")
    .include("src")
    .compile("libhello.a");
}
