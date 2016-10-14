mod ll;

// These are the public bindings to libutp. These bindings are considered safe,
// though this will not prevent libutp from crashing. It basically means Rust
// is ready to take over from this point onward and do it's thing safely.
pub fn utp_create_socket () {
  unsafe {
    ll::utp_create_socket()
  };
}
