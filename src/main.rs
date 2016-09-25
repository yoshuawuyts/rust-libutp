#[link(name="utp", kind="static")]
extern {
  fn utp_create_socket ();
}

fn main () {
  unsafe {
    utp_create_socket();
  };
}
