use super::compact_integer::CompactInteger;
use super::io;

pub trait Deserializable {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
    where
        Self: Sized,
        T: io::Read;
}

/// Bitcoin structures reader.
#[derive(Debug)]
pub struct Reader<T> {
    buffer: T,
    peeked: Option<u8>,
}

impl<'a> Reader<&'a [u8]> {
    /// Convenient way of creating for slice of bytes
    pub fn new(buffer: &'a [u8]) -> Self {
        Reader {
            buffer,
            peeked: None,
        }
    }
}

impl<R> Reader<R>
where
    R: io::Read,
{
    pub fn from_read(read: R) -> Self {
        Reader {
            buffer: read,
            peeked: None,
        }
    }

    pub fn read<T>(&mut self) -> Result<T, io::Error>
    where
        T: Deserializable,
    {
        T::deserialize(self)
    }

    pub fn read_slice(&mut self, bytes: &mut [u8]) -> Result<(), io::Error> {
        io::Read::read_exact(self, bytes).map_err(|_| io::ErrorKind::UnexpectedEnd)
    }

    pub fn read_list<T>(&mut self) -> Result<Vec<T>, io::Error>
    where
        T: Deserializable,
    {
        let len: usize = self.read::<CompactInteger>()?.into();
        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            result.push(self.read()?);
        }

        Ok(result)
    }

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
    pub fn is_finished(&mut self) -> bool {
        if self.peeked.is_some() {
            return false;
        }

        let peek: &mut [u8] = &mut [0u8];
        match self.read_slice(peek) {
            Ok(_) => {
                self.peeked = Some(peek[0]);
                false
            }
            Err(_) => true,
        }
    }
}

impl<T> io::Read for Reader<T>
where
    T: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        // most of the times, there will be nothing in peeked,
        // so to make it as efficient as possible, check it
        // only once
        match self.peeked.take() {
            None => io::Read::read(&mut self.buffer, buf),
            Some(peeked) if buf.is_empty() => {
                self.peeked = Some(peeked);
                Ok(0)
            }
            Some(peeked) => {
                buf[0] = peeked;
                io::Read::read(&mut self.buffer, &mut buf[1..]).map(|x| x + 1)
            }
        }
    }
}

pub fn deserialize<R, T>(buffer: R) -> Result<T, io::Error>
where
    R: io::Read,
    T: Deserializable,
{
    let mut reader = Reader::from_read(buffer);
    let result = reader.read()?;

    if reader.is_finished() {
        Ok(result)
    } else {
        Err(io::ErrorKind::UnreadData)
    }
}
