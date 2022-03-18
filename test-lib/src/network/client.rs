use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("starting connect");

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().
            read_line(&mut input)
            .expect("Failed to read from stdin");

        stream.write(input.as_bytes())
            .expect("Failed to write to stream");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!("{}", String::from_utf8(buffer).expect("Could not write buffer as string"));
    }

    Ok(())
}

