use std::net::UdpSocket;

fn main () {
  // let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

  // // read from the socket
  // let mut buf = [0; 10];
  // let (amt, src) = socket.recv_from(&mut buf).unwrap();

  // // send a reply to the socket we received data from
  // let buf = &mut buf[..amt];
  // buf.reverse();
  // socket.send_to(buf, &src).unwrap();
}
