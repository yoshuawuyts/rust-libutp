#[link(name="hello", kind="static")]
extern {
  fn hello_hello () -> i32;
}

fn main () {
  unsafe {
    hello_hello();
  };
}
