use std::str;
use std::net::UdpSocket;

fn main() {
    println!("Starting angry-server...");

    let port = 34543;
    let socket = match UdpSocket::bind(("0.0.0.0", port)) {
        Ok(s) => s,
        Err(e) => panic!("Unable to bind socket: {}", e)
    };

    println!("Server started on port {}.", port);

    // read from the socket
    loop {
        let mut buf = [0; 1024];
        
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                println!("** ({}): {}", src, str::from_utf8(&buf).unwrap_or(""));

                let lowercase_text = String::from_utf8_lossy(&buf);
                let uppercase_text = lowercase_text.to_uppercase().into_bytes();
                
                socket.send_to(&uppercase_text, &src).expect("Could not return message.");
            },

            Err(e) => { println!("Unable to recieve data: {}", e); }
        }

    }
}
