use std::borrow::Borrow;

use super::compact_integer::CompactInteger;
use super::io;
use super::Bytes;

/// Do not serialize transaction witness data.
pub const SERIALIZE_TRANSACTION_WITNESS: u32 = 0x4000_0000;

pub trait Serializable {
    /// Serialize the struct and appends it to the end of stream.
    fn serialize(&self, s: &mut Stream);

    /// Hint about the size of serialized struct.
    fn serialized_size(&self) -> usize
    where
        Self: Sized,
    {
        // fallback implementation
        serialize(self).len()
    }
}

/// Stream used for serialization of Bitcoin structures
#[derive(Default)]
pub struct Stream {
    buffer: Vec<u8>,
    flags: u32,
}

impl Stream {
    /// Are transactions written to this stream with witness data?
    pub fn include_transaction_witness(&self) -> bool {
        (self.flags & SERIALIZE_TRANSACTION_WITNESS) != 0
    }

    /// Serializes the struct and appends it to the end of stream.
    pub fn append<T>(&mut self, t: &T) -> &mut Self
    where
        T: Serializable,
    {
        t.serialize(self);
        self
    }

    /// Appends raw bytes to the end of the stream.
    pub fn append_slice(&mut self, bytes: &[u8]) -> &mut Self {
        // discard error for now, since we write to simple vector
        io::Write::write(&mut self.buffer, bytes);
        self
    }

    /// Appends a list of serializable structs to the end of the stream.
    pub fn append_list<T, K>(&mut self, t: &[K]) -> &mut Self
    where
        T: Serializable,
        K: Borrow<T>,
    {
        CompactInteger::from(t.len()).serialize(self);
        for i in t {
            i.borrow().serialize(self);
        }
        self
    }

    /// Full stream.
    pub fn out(self) -> Bytes {
        self.buffer.into()
    }
}

impl io::Write for Stream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.buffer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffer.flush()
    }
}

pub fn serialize<T>(t: &T) -> Bytes
where
    T: Serializable,
{
    let mut stream = Stream::default();
    stream.append(t);
    stream.out()
}
