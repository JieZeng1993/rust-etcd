use std::fmt::Error;
use std::io::Cursor;
use tokio::net::TcpStream;

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
        loop {
            let mut buf = Cursor::new(&self.buffer[..]);
            if buf.position() > 2 {
                //0到3 index 是4个字节
                let len = buf.get_u32();
                if buf.remaining() >= len as usize {
                    //do parse
                    // let data = self.buffer[4..(len - 1)];
                    self.buffer.advance((len + 4) as usize);
                }
            }

            // if 0 == self.stream.read_buf(&mut self.buffer).await? {
            //     return if self.buffer.is_empty() {
            //         Ok(None)
            //     } else {
            //         Err("connection reset by peer".into())
            //     };
            // }
        }
    }
}