use byteorder;
use parity_codec::{Decode, Encode, Input};
use parity_codec_derive::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

use crate::serde_ext::Bytes;

/// A type of variable-length integer commonly used in the Bitcoin P2P protocol and Bitcoin serialized data structures.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct CompactInteger(u64);

impl From<CompactInteger> for usize {
    fn from(i: CompactInteger) -> Self {
        i.0 as usize
    }
}

impl From<CompactInteger> for u64 {
    fn from(i: CompactInteger) -> Self {
        i.0
    }
}

impl From<u8> for CompactInteger {
    fn from(i: u8) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<u16> for CompactInteger {
    fn from(i: u16) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<u32> for CompactInteger {
    fn from(i: u32) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<usize> for CompactInteger {
    fn from(i: usize) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<u64> for CompactInteger {
    fn from(i: u64) -> Self {
        CompactInteger(i)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Compact(u32);

impl Compact {
    pub fn new(u: u32) -> Self {
        Compact(u)
    }
}

impl From<u32> for Compact {
    fn from(u: u32) -> Self {
        Compact(u)
    }
}

impl From<Compact> for u32 {
    fn from(c: Compact) -> Self {
        c.0
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BlockHeader {
    pub version: u32,
    pub previous_header_hash: substrate_primitives::H256,
    pub merkle_root_hash: substrate_primitives::H256,
    pub time: u32,
    pub bits: Compact,
    pub nonce: u32,
}

impl Encode for BlockHeader {
    fn encode(&self) -> Vec<u8> {
        let value = stream::serialize::<BlockHeader>(&self);
        value.encode()
    }
}

impl Decode for BlockHeader {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let value: Vec<u8> = Decode::decode(input).unwrap();
        if let Ok(header) = reader::deserialize(reader::Reader::new(&value)) {
            Some(header)
        } else {
            None
        }
    }
}

mod io {
    use super::byteorder::ByteOrder;

    pub enum ErrorKind {
        Interrupted,
        UnexpectedEof,
        WriteZero,
        MalformedData,
        UnexpectedEnd,
        UnreadData,
        Deserialize,
        InvalidVersion,
    }

    pub type Error = ErrorKind;

    pub trait Read {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

        //        fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        //            read_to_end(self, buf)
        //        }

        //        #[inline]
        //        unsafe fn initializer(&self) -> Initializer {
        //            Initializer::zeroing()
        //        }

        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), Error> {
            while !buf.is_empty() {
                match self.read(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let tmp = buf;
                        buf = &mut tmp[n..];
                    }
                    //Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
            if !buf.is_empty() {
                Err(ErrorKind::UnexpectedEof)
            } else {
                Ok(())
            }
        }

        fn by_ref(&mut self) -> &mut Self
        where
            Self: Sized,
        {
            self
        }

        /*fn bytes(self) -> Bytes<Self> where Self: Sized {
            Bytes { inner: self }
        }

        fn chain<R: Read>(self, next: R) -> Chain<Self, R> where Self: Sized {
            Chain { first: self, second: next, done_first: false }
        }*/

        //        fn take(self, limit: u64) -> Take<Self> where Self: Sized {
        //            Take { inner: self, limit: limit }
        //        }

        fn read_u8(&mut self) -> Result<u8, Error> {
            let mut buf = [0; 1];
            self.read_exact(&mut buf)?;
            Ok(buf[0])
        }

        fn read_u16<BO: ByteOrder>(&mut self) -> Result<u16, Error> {
            let mut buf = [0; 2];
            self.read_exact(&mut buf)?;
            Ok(BO::read_u16(&buf))
        }

        fn read_u32<BO: ByteOrder>(&mut self) -> Result<u32, Error> {
            let mut buf = [0; 4];
            self.read_exact(&mut buf)?;
            Ok(BO::read_u32(&buf))
        }

        fn read_u64<BO: ByteOrder>(&mut self) -> Result<u64, Error> {
            let mut buf = [0; 8];
            self.read_exact(&mut buf)?;
            Ok(BO::read_u64(&buf))
        }

        fn read_i16<BO: ByteOrder>(&mut self) -> Result<i16, Error> {
            let mut buf = [0; 2];
            self.read_exact(&mut buf)?;
            Ok(BO::read_i16(&buf))
        }

        fn read_i32<BO: ByteOrder>(&mut self) -> Result<i32, Error> {
            let mut buf = [0; 4];
            self.read_exact(&mut buf)?;
            Ok(BO::read_i32(&buf))
        }

        fn read_i64<BO: ByteOrder>(&mut self) -> Result<i64, Error> {
            let mut buf = [0; 8];
            self.read_exact(&mut buf)?;
            Ok(BO::read_i64(&buf))
        }
    }

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;

        fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
            while !buf.is_empty() {
                match self.write(buf) {
                    Ok(0) => return Err(ErrorKind::WriteZero),
                    Ok(n) => buf = &buf[n..],
                    //Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }

        fn flush(&mut self) -> Result<(), Error>;

        fn by_ref(&mut self) -> &mut Self
        where
            Self: Sized,
        {
            self
        }

        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            let mut buf = [0; 1];
            buf[0] = val;
            self.write_all(&buf)
        }

        fn write_u16<BO: ByteOrder>(&mut self, val: u16) -> Result<(), Error> {
            let mut buf = [0; 2];
            BO::write_u16(&mut buf, val);
            self.write_all(&buf)
        }

        fn write_u32<BO: ByteOrder>(&mut self, val: u32) -> Result<(), Error> {
            let mut buf = [0; 4];
            BO::write_u32(&mut buf, val);
            self.write_all(&buf)
        }

        fn write_u64<BO: ByteOrder>(&mut self, val: u64) -> Result<(), Error> {
            let mut buf = [0; 8];
            BO::write_u64(&mut buf, val);
            self.write_all(&buf)
        }

        fn write_i16<BO: ByteOrder>(&mut self, val: i16) -> Result<(), Error> {
            let mut buf = [0; 2];
            BO::write_i16(&mut buf, val);
            self.write_all(&buf)
        }

        fn write_i32<BO: ByteOrder>(&mut self, val: i32) -> Result<(), Error> {
            let mut buf = [0; 4];
            BO::write_i32(&mut buf, val);
            self.write_all(&buf)
        }

        fn write_i64<BO: ByteOrder>(&mut self, val: i64) -> Result<(), Error> {
            let mut buf = [0; 8];
            BO::write_i64(&mut buf, val);
            self.write_all(&buf)
        }
    }

    impl<'a, R: Read + ?Sized> Read for &'a mut R {
        #[inline]
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            (**self).read(buf)
        }

        //        #[inline]
        //        unsafe fn initializer(&self) -> Initializer {
        //            (**self).initializer()
        //        }

        //        #[inline]
        //        fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        //            (**self).read_to_end(buf)
        //        }

        #[inline]
        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
            (**self).read_exact(buf)
        }
    }

    impl<'a, W: Write + ?Sized> Write for &'a mut W {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            (**self).write(buf)
        }

        #[inline]
        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
            (**self).write_all(buf)
        }

        #[inline]
        fn flush(&mut self) -> Result<(), Error> {
            (**self).flush()
        }
    }

    impl<'a> Read for &'a [u8] {
        #[inline]
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            let amt = ::std::cmp::min(buf.len(), self.len());
            let (a, b) = self.split_at(amt);

            // First check if the amount of bytes we want to read is small:
            // `copy_from_slice` will generally expand to a call to `memcpy`, and
            // for a single byte the overhead is significant.
            if amt == 1 {
                buf[0] = a[0];
            } else {
                buf[..amt].copy_from_slice(a);
            }

            *self = b;
            Ok(amt)
        }

        //        #[inline]
        //        unsafe fn initializer(&self) -> Initializer {
        //            Initializer::nop()
        //        }

        #[inline]
        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
            if buf.len() > self.len() {
                return Err(ErrorKind::UnexpectedEof);
            }
            let (a, b) = self.split_at(buf.len());

            // First check if the amount of bytes we want to read is small:
            // `copy_from_slice` will generally expand to a call to `memcpy`, and
            // for a single byte the overhead is significant.
            if buf.len() == 1 {
                buf[0] = a[0];
            } else {
                buf.copy_from_slice(a);
            }

            *self = b;
            Ok(())
        }

        //        #[inline]
        //        fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        //            buf.extend_from_slice(*self);
        //            let len = self.len();
        //            *self = &self[len..];
        //            Ok(len)
        //        }
    }

    impl<'a> Write for &'a mut [u8] {
        #[inline]
        fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
            let amt = ::std::cmp::min(data.len(), self.len());
            let (a, b) = ::std::mem::replace(self, &mut []).split_at_mut(amt);
            a.copy_from_slice(&data[..amt]);
            *self = b;
            Ok(amt)
        }

        #[inline]
        fn write_all(&mut self, data: &[u8]) -> Result<(), Error> {
            if self.write(data)? == data.len() {
                Ok(())
            } else {
                Err(ErrorKind::WriteZero)
            }
        }

        #[inline]
        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    impl Write for Vec<u8> {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            self.extend_from_slice(buf);
            Ok(buf.len())
        }

        #[inline]
        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
            self.extend_from_slice(buf);
            Ok(())
        }

        #[inline]
        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }
}

mod stream {
    use super::{
        io::{self, Write},
        Bytes, CompactInteger,
    };
    use std::borrow::Borrow;

    /// Do not serialize transaction witness data.
    pub const SERIALIZE_TRANSACTION_WITNESS: u32 = 0x40000000;

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
            self.buffer.write(bytes);
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
}

mod reader {
    use super::io;
    use super::CompactInteger;

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

        //    pub fn read_with_proxy<T, F>(&mut self, proxy: F) -> Result<T, io::Error> where T: Deserializable, F: FnMut(&[u8]) {
        //        let mut reader = Reader::from_read(Proxy::new(self, proxy));
        //        T::deserialize(&mut reader)
        //    }

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

        //    pub fn read_list_max<T>(&mut self, max: usize) -> Result<Vec<T>, io::Error> where T: Deserializable {
        //        let len: usize = self.read::<CompactInteger>()?.into();
        //        if len > max {
        //            return Err(io::ErrorKind::MalformedData);
        //        }
        //
        //        let mut result = Vec::with_capacity(len);
        //
        //        for _ in 0..len {
        //            result.push(self.read()?);
        //        }
        //
        //        Ok(result)
        //    }

        #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
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
}

mod impls {
    use super::byteorder::LittleEndian;
    use super::io::{self, Read, Write};
    use super::reader::{Deserializable, Reader};
    use super::stream::{Serializable, Stream};
    use super::{
        BlockHeader, Bytes, Compact, CompactInteger, OutPoint, Transaction, TransactionInput,
        TransactionOutput,
    };

    /// Must be zero.
    const WITNESS_MARKER: u8 = 0;
    /// Must be nonzero.
    const WITNESS_FLAG: u8 = 1;

    impl Serializable for i32 {
        #[inline]
        fn serialize(&self, s: &mut Stream) {
            s.write_i32::<LittleEndian>(*self);
        }

        #[inline]
        fn serialized_size(&self) -> usize {
            4
        }
    }

    impl Deserializable for i32 {
        #[inline]
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            Ok(reader.read_i32::<LittleEndian>()?)
        }
    }

    impl Serializable for u8 {
        #[inline]
        fn serialize(&self, s: &mut Stream) {
            s.write_u8(*self);
        }

        #[inline]
        fn serialized_size(&self) -> usize {
            1
        }
    }

    impl Deserializable for u8 {
        #[inline]
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            Ok(reader.read_u8()?)
        }
    }

    impl Serializable for u16 {
        #[inline]
        fn serialize(&self, s: &mut Stream) {
            s.write_u16::<LittleEndian>(*self);
        }

        #[inline]
        fn serialized_size(&self) -> usize {
            2
        }
    }

    impl Deserializable for u16 {
        #[inline]
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            Ok(reader.read_u16::<LittleEndian>()?)
        }
    }

    impl Serializable for u32 {
        #[inline]
        fn serialize(&self, s: &mut Stream) {
            s.write_u32::<LittleEndian>(*self);
        }

        #[inline]
        fn serialized_size(&self) -> usize {
            4
        }
    }

    impl Deserializable for u32 {
        #[inline]
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            Ok(reader.read_u32::<LittleEndian>()?)
        }
    }

    impl Serializable for u64 {
        #[inline]
        fn serialize(&self, s: &mut Stream) {
            s.write_u64::<LittleEndian>(*self);
        }

        #[inline]
        fn serialized_size(&self) -> usize {
            8
        }
    }

    impl Deserializable for u64 {
        #[inline]
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            Ok(reader.read_u64::<LittleEndian>()?)
        }
    }

    impl Serializable for Bytes {
        fn serialize(&self, stream: &mut Stream) {
            stream
                .append(&CompactInteger::from(self.len()))
                .append_slice(self.as_ref());
        }

        #[inline]
        fn serialized_size(&self) -> usize {
            CompactInteger::from(self.len()).serialized_size() + self.len()
        }
    }

    impl Deserializable for Bytes {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            let len = reader.read::<CompactInteger>()?;
            let mut bytes = Bytes::new_with_len(len.into());
            reader.read_slice(bytes.as_mut())?;
            Ok(bytes)
        }
    }

    impl Serializable for CompactInteger {
        fn serialize(&self, stream: &mut Stream) {
            match self.0 {
                0...0xfc => {
                    stream.append(&(self.0 as u8));
                }
                0xfd...0xffff => {
                    stream.append(&0xfdu8).append(&(self.0 as u16));
                }
                0x10000...0xffff_ffff => {
                    stream.append(&0xfeu8).append(&(self.0 as u32));
                }
                _ => {
                    stream.append(&0xffu8).append(&self.0);
                }
            }
        }

        fn serialized_size(&self) -> usize {
            match self.0 {
                0...0xfc => 1,
                0xfd...0xffff => 3,
                0x10000...0xffff_ffff => 5,
                _ => 9,
            }
        }
    }

    impl Deserializable for CompactInteger {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            let result = match reader.read::<u8>()? {
                i @ 0...0xfc => i.into(),
                0xfd => reader.read::<u16>()?.into(),
                0xfe => reader.read::<u32>()?.into(),
                _ => reader.read::<u64>()?.into(),
            };

            Ok(result)
        }
    }

    impl Serializable for Compact {
        fn serialize(&self, stream: &mut Stream) {
            stream.append(&u32::from(*self));
        }
    }

    impl Deserializable for Compact {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            reader.read::<u32>().map(Compact::new)
        }
    }

    macro_rules! impl_ser_for_hash {
        ($name: ident, $size: expr) => {
            impl Serializable for $name {
                fn serialize(&self, stream: &mut Stream) {
                    stream.append_slice(&self.as_ref());
                }

                #[inline]
                fn serialized_size(&self) -> usize {
                    $size
                }
            }

            impl Deserializable for $name {
                fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
                where
                    T: io::Read,
                {
                    let mut result = Self::default();
                    reader.read_slice(&mut result.as_mut())?;
                    Ok(result)
                }
            }
        };
    }

    use substrate_primitives::H256;
    impl_ser_for_hash!(H256, 32);

    impl Serializable for BlockHeader {
        fn serialize(&self, stream: &mut Stream) {
            stream
                .append(&self.version)
                .append(&self.previous_header_hash)
                .append(&self.merkle_root_hash)
                .append(&self.time)
                .append(&self.bits)
                .append(&self.nonce);
        }
    }

    impl Deserializable for BlockHeader {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            T: io::Read,
        {
            Ok(BlockHeader {
                version: reader.read()?,
                previous_header_hash: reader.read()?,
                merkle_root_hash: reader.read()?,
                time: reader.read()?,
                bits: reader.read()?,
                nonce: reader.read()?,
            })
        }
    }

    impl Serializable for OutPoint {
        fn serialize(&self, stream: &mut Stream) {
            stream.append(&self.hash).append(&self.index);
        }
    }

    impl Deserializable for OutPoint {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            Self: Sized,
            T: io::Read,
        {
            Ok(OutPoint {
                hash: reader.read()?,
                index: reader.read()?,
            })
        }
    }

    impl Serializable for TransactionInput {
        fn serialize(&self, stream: &mut Stream) {
            stream
                .append(&self.previous_output)
                .append(&self.script_sig)
                .append(&self.sequence);
        }
    }

    impl Deserializable for TransactionInput {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            Self: Sized,
            T: io::Read,
        {
            Ok(TransactionInput {
                previous_output: reader.read()?,
                script_sig: reader.read()?,
                sequence: reader.read()?,
                script_witness: vec![],
            })
        }
    }

    impl Serializable for TransactionOutput {
        fn serialize(&self, stream: &mut Stream) {
            stream.append(&self.value).append(&self.script_pubkey);
        }
    }

    impl Deserializable for TransactionOutput {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            Self: Sized,
            T: io::Read,
        {
            Ok(TransactionOutput {
                value: reader.read()?,
                script_pubkey: reader.read()?,
            })
        }
    }

    impl Serializable for Transaction {
        fn serialize(&self, stream: &mut Stream) {
            let include_transaction_witness =
                stream.include_transaction_witness() && self.has_witness();
            match include_transaction_witness {
                false => stream
                    .append(&self.version)
                    .append_list(&self.inputs)
                    .append_list(&self.outputs)
                    .append(&self.lock_time),
                true => {
                    stream
                        .append(&self.version)
                        .append(&WITNESS_MARKER)
                        .append(&WITNESS_FLAG)
                        .append_list(&self.inputs)
                        .append_list(&self.outputs);
                    for input in &self.inputs {
                        stream.append_list(&input.script_witness);
                    }
                    stream.append(&self.lock_time)
                }
            };
        }
    }

    impl Deserializable for Transaction {
        fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error>
        where
            Self: Sized,
            T: io::Read,
        {
            let version = reader.read()?;
            let mut inputs: Vec<TransactionInput> = reader.read_list()?;
            let read_witness = if inputs.is_empty() {
                let witness_flag: u8 = reader.read()?;
                if witness_flag != WITNESS_FLAG {
                    return Err(io::ErrorKind::MalformedData);
                }

                inputs = reader.read_list()?;
                true
            } else {
                false
            };
            let outputs = reader.read_list()?;
            if read_witness {
                for input in inputs.iter_mut() {
                    input.script_witness = reader.read_list()?;
                }
            }

            Ok(Transaction {
                version,
                inputs,
                outputs,
                lock_time: reader.read()?,
            })
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum Type {
    /// Pay to PubKey Hash
    /// Common P2PKH which begin with the number 1, eg: 1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2.
    /// https://bitcoin.org/en/glossary/p2pkh-address
    P2PKH,
    /// Pay to Script Hash
    /// Newer P2SH type starting with the number 3, eg: 3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy.
    /// https://bitcoin.org/en/glossary/p2sh-address
    P2SH,
}

impl Default for Type {
    fn default() -> Self {
        Type::P2PKH
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum Network {
    Mainnet,
    Testnet,
}

impl Default for Network {
    fn default() -> Self {
        Network::Mainnet
    }
}

/// `AddressHash` with network identifier and format type
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Address {
    /// The type of the address.
    pub kind: Type,
    /// The network of the address.
    pub network: Network,
    /// Public key hash.
    pub hash: substrate_primitives::H160,
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn has_witness(&self) -> bool {
        self.inputs.iter().any(TransactionInput::has_witness)
    }
}

impl Encode for Transaction {
    fn encode(&self) -> Vec<u8> {
        let value = stream::serialize::<Transaction>(&self);
        value.encode()
    }
}

impl Decode for Transaction {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let value: Vec<u8> = Decode::decode(input).unwrap();
        if let Ok(tx) = reader::deserialize(reader::Reader::new(&value)) {
            Some(tx)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Bytes,
    pub sequence: u32,
    pub script_witness: Vec<Bytes>,
}

impl TransactionInput {
    pub fn has_witness(&self) -> bool {
        !self.script_witness.is_empty()
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct OutPoint {
    pub hash: substrate_primitives::H256,
    pub index: u32,
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Bytes,
}
