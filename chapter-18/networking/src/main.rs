use std::io;
use std::net::TcpListener;
use std::thread::spawn;

fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listen on {}", addr);

    loop {
        // Wait for client connect
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from: {}", addr);

        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            io::copy(&mut stream, &mut write_stream).expect("error in client thread:");
            println!("connection closed")
        });
    }
}

// under macOS, you can use the nc command in the termninal to connect to the echo server for testing.
fn main() {
    echo_main("127.0.0.1:17001").expect("error: ");
}
