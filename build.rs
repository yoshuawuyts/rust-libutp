extern crate gcc;

fn main () {
  gcc::Config::new()
    .cpp(true)
    .flag("-D POSIX")
    .flag("-D UTP_DEBUG_LOGGING")
    .file("libutp/utp_internal.cpp")
    .file("libutp/utp_utils.cpp")
    .file("libutp/utp_hash.cpp")
    .file("libutp/utp_callbacks.cpp")
    .file("libutp/utp_api.cpp")
    .file("libutp/utp_packedsockaddr.cpp")
    .include("libutp")
    .compile("libutp.a");
}
