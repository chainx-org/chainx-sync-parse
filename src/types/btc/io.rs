use byteorder::ByteOrder;

#[allow(dead_code)]
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
