use std::net::UdpSocket;

fn main() {
    println!("Starting angry-server");

    let mut socket = match UdpSocket::bind("0.0.0.0:34543") {
        Ok(s) => s,
        Err(e) => panic!("Unable to bind socket: {}", e)
    };

    println!("Bound to port 34543.");

    // read from the socket
    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let buf = &mut buf[..amt];
                buf.reverse();
                socket.send_to(buf, &src);
            },

            Err(e) => { println!("Unable to recieve data: {}", e); }
        }

    }
}
