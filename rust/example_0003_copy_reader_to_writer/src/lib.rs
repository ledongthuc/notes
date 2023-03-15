use std::io::{Read, Write, self, ErrorKind};

const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where R: Read, W: Write
{
    let mut buf = [0; DEFAULT_BUFFER_SIZE];
    let mut written = 0;

    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}
