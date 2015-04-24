use std::io::{self, Read, Write, ErrorKind, BufReader, BufWriter};
use std::fs::{self, File};

const DEFAULT_BUF_SIZE: usize = 64 * 1024;
const INTENSITY_DENOM: u64 = 1 * 1024 * 1024;

struct CorruptingCopy {
    pos: u64,
    insertions: bool,
    deletions: bool,
    bitflips: bool,
}

impl CorruptingCopy {
    pub fn new() -> CorruptingCopy {
        CorruptingCopy {
            pos: 0,
            insertions: false,
            deletions: false,
            bitflips: true,
        }
    }

    fn corrupt_buf(&self, buf: &mut [u8]) {
        let end = self.pos + buf.len() as u64;
        for pos in self.pos..end {
            let intensity = self.pos / INTENSITY_DENOM;
            if intensity > 0 && pos % (10_000_000 / intensity) == 0 {
                let offset = (pos - self.pos) as usize;
                buf[offset] = buf[offset] ^ 0x3;
            }
        }
    }

    pub fn copy<R, W>(&mut self, r: &mut R, w: &mut W) -> io::Result<u64>
        where
            R: Read,
            W: Write {

        let mut buf = [0; DEFAULT_BUF_SIZE];
        let mut written = 0;

        loop {
            let len = match r.read(&mut buf) {
                Ok(0) => return Ok(written),
                Ok(len) => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };

            self.corrupt_buf(&mut buf[..len]);
            try!(w.write_all(&buf[..len]));

            written += len as u64;
            self.pos += len as u64;
        }
    }
}

fn main() {
    let mut input = BufReader::new(io::stdin());
    let mut output = BufWriter::new(io::stdout());

    if let Err(err) = CorruptingCopy::new().copy(&mut input, &mut output) {
        panic!("error during copy: {}", err);
    }
}
