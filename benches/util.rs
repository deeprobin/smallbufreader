use std::io::Read;

pub(crate) struct EndlessZeroReader;

impl Read for EndlessZeroReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        buf.fill(0);
        Ok(buf.len())
    }
}
