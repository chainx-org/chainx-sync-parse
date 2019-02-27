use byteorder::LittleEndian;

use super::compact_integer::CompactInteger;
use super::io::{self, Read, Write};
use super::reader::{Deserializable, Reader};
use super::stream::{Serializable, Stream};
use super::{
    BlockHeader, Bytes, Compact, OutPoint, Transaction, TransactionInput, TransactionOutput,
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
