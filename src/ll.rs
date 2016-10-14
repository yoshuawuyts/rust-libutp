#[link(name="utp", kind="static")]

// These are the low level (ll) bindings to libutp. All of these functions
// should be called from within unsafe{} blocks
extern {
  pub fn utp_create_socket ();
}
