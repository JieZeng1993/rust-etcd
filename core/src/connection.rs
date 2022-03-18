use std::fmt::Error;
use std::io::Cursor;
use std::net::TcpStream;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};

#[derive(Debug)]
pub struct Connection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    pub(crate) stream: BufWriter<TcpStream>,

    // The buffer for reading frames.
    pub(crate) buffer: BytesMut,
}

impl Connection {
    pub async fn read_frame(&mut self) -> Result<Option<String>, Error> {
        let mut buf = Cursor::new(&self.buffer[..]);
        if 0 == self.stream.read_buf(&mut self.buffer).await?{
            if self.stream.
        }
        let len = buf.get_u32();
        Ok(None)
    }
}