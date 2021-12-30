use std::{
    io::{Read, Write},
    string::FromUtf8Error,
};

pub type EncodingResult<T> = Result<T, EncodingError>;

#[derive(Debug, thiserror::Error)]
pub enum EncodingError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("FromUtf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("Varint too long")]
    VarintTooLong,
    #[error("Wrong wire type for {0}: {1}")]
    WrongWireType(&'static str, u8),
    #[error("Invalid enum value for {0}: {1}")]
    InvalidEnumValue(&'static str, u32),
    #[error("Invalid bool value: {0}")]
    InvalidBoolValue(u8),
    #[error("Missing field: {0}")]
    MissingField(u8),
    #[error("Error {0}: {1}")]
    Context(String, Box<dyn std::error::Error>),
}

impl EncodingError {
    pub fn context(self, context: impl Into<String>) -> Self {
        Self::Context(context.into(), Box::new(self))
    }
}

pub trait Encode {
    fn size(&self) -> u32;
    fn encode<W: Write>(&self, w: W) -> Result<(), EncodingError>;
    fn encode_with_len<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        let mut buf = vec![];
        self.encode(&mut buf)?;
        Varint(buf.len() as u32).encode(&mut w)?;
        w.write_all(&buf)?;
        Ok(())
    }
}

pub trait Decode: Sized {
    fn decode<R: Read>(r: R) -> Result<Self, EncodingError>;
    // Workaround to recurrent type instantiation blow-up (&mut &mut...[u8]: Read)
    fn decode_dyn(r: &mut dyn Read) -> Result<Self, EncodingError> {
        Self::decode(r)
    }
    fn decode_as_field(r: &mut dyn Read) -> Result<Self, EncodingError> {
        Self::decode(r)
    }
    fn decode_with_len<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let len = Varint::<u32>::decode(&mut r)
            .map_err(|x| x.context("varint len read"))?
            .0;
        let mut buf = vec![0; len as usize];
        r.read_exact(&mut buf)?;
        Self::decode(buf.as_slice())
    }
}

pub fn encode_field<W: Write, T: Encode>(
    mut w: W,
    field_number: u8,
    wire_type: u8,
    field: &T,
) -> Result<(), EncodingError> {
    let key = field_number << 3 | wire_type;
    w.write(&[key])?;
    field.encode(w)
}

pub fn decode_packed<R: Read, T: Decode>(
    r: R,
) -> Result<Vec<T>, EncodingError> {
    let mut buf = std::io::Cursor::new(<Vec<u8>>::decode_with_len(r)?);
    let mut res = vec![];
    while !buf.get_ref().is_empty() {
        res.push(T::decode(&mut buf)?);
    }
    Ok(res)
}

pub trait EncodeDyn {
    fn encode_dyn(&self, w: &mut dyn Write) -> Result<(), EncodingError>;
    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodingError>;
}

impl<E: Encode> EncodeDyn for E {
    fn encode_dyn(&self, w: &mut dyn Write) -> Result<(), EncodingError> {
        Encode::encode(self, w)
    }

    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodingError> {
        let mut vec = vec![];
        self.encode(&mut vec)?;
        Ok(vec)
    }
}

pub struct Varint<T>(pub T);
pub struct SVarint<T>(pub T);
pub struct Fixed<T>(pub T);

impl<T> From<T> for Varint<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> From<T> for SVarint<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> From<T> for Fixed<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

macro_rules! impl_encode_varint {
    ($t:ty) => {
        impl Encode for Varint<$t> {
            fn size(&self) -> u32 {
                // round up to multiply off 7 because varints
                ((<$t>::BITS - self.0.leading_zeros()) + 7) / 7
            }
        
            fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
                let mut i = self.0;
                while i > 127 {
                    let value = (i as u8 & 0x7f) | 0x80;
                    w.write_all(&[value])?;
                    i >>= 7;
                }
                w.write_all(&[i as u8 & 0x7f])?;
                Ok(())
            }
        }
    };
}

impl_encode_varint!(u32);
impl_encode_varint!(u64);

macro_rules! impl_decode_varint {
    ($t:ty : $varsize:literal) => {
        impl Decode for Varint<$t> {
            fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
                let mut res = 0;
                for i in 0..$varsize {
                    let mut byte = 0;
                    r.read_exact(std::slice::from_mut(&mut byte))?;
                    res |= (byte as $t & 0x7f) << i * 7;
                    if byte & 0x80 == 0 {
                        return Ok(Varint(res));
                    }
                }
                Err(EncodingError::VarintTooLong)
            }
        }
    };
}

impl_decode_varint!(u32: 5);
impl_decode_varint!(u64: 10);

impl Encode for Varint<i32> {
    fn size(&self) -> u32 {
        Varint(self.0 as u32).size()
    }

    fn encode<W: Write>(&self, w: W) -> Result<(), EncodingError> {
        Varint(self.0 as u32).encode(w)
    }
}

impl Decode for Varint<i32> {
    fn decode<R: Read>(r: R) -> Result<Self, EncodingError> {
        <Varint<u32>>::decode(r).map(|Varint(x)| Varint(x as i32))
    }
}

impl Encode for Varint<i64> {
    fn size(&self) -> u32 {
        Varint(self.0 as u64).size()
    }

    fn encode<W: Write>(&self, w: W) -> Result<(), EncodingError> {
        Varint(self.0 as u64).encode(w)
    }
}

impl Decode for Varint<i64> {
    fn decode<R: Read>(r: R) -> Result<Self, EncodingError> {
        <Varint<u64>>::decode(r).map(|Varint(x)| Varint(x as i64))
    }
}

impl Encode for Fixed<u32> {
    fn size(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        w.write_all(&self.0.to_le_bytes())?;
        Ok(())
    }
}

impl Decode for Fixed<u32> {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let mut buf = [0; std::mem::size_of::<Self>()];
        r.read_exact(&mut buf)?;
        Ok(Fixed(u32::from_le_bytes(buf)))
    }
}

impl Encode for Fixed<u64> {
    fn size(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        w.write_all(&self.0.to_le_bytes())?;
        Ok(())
    }
}

impl Decode for Fixed<u64> {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let mut buf = [0; std::mem::size_of::<Self>()];
        r.read_exact(&mut buf)?;
        Ok(Fixed(u64::from_le_bytes(buf)))
    }
}

impl Encode for f32 {
    fn size(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        w.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Decode for f32 {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let mut buf = [0; std::mem::size_of::<Self>()];
        r.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Encode for f64 {
    fn size(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        w.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Decode for f64 {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let mut buf = [0; std::mem::size_of::<Self>()];
        r.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Encode for bool {
    fn size(&self) -> u32 {
        1
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        w.write_all(&[*self as u8])?;
        Ok(())
    }
}

impl Decode for bool {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let mut byte = 0;
        r.read_exact(std::slice::from_mut(&mut byte))?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            e => Err(EncodingError::InvalidBoolValue(e)),
        }
    }
}

impl Encode for u8 {
    fn size(&self) -> u32 {
        1
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        Ok(w.write_all(&[*self])?)
    }
}

impl Decode for u8 {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let mut buf = 0;
        r.read_exact(std::slice::from_mut(&mut buf))?;
        Ok(buf)
    }
}

impl<T: Encode> Encode for &[T] {
    fn size(&self) -> u32 {
        let len = self.len() as u32;
        Varint(len).size() + len
    }

    fn encode<W: Write>(&self, mut w: W) -> Result<(), EncodingError> {
        Varint(self.len() as u32).encode(&mut w)?;
        for t in *self {
            t.encode(&mut w)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn size(&self) -> u32 {
        self.as_slice().size()
    }

    fn encode<W: Write>(&self, w: W) -> Result<(), EncodingError> {
        self.as_slice().encode(w)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<R: Read>(mut r: R) -> Result<Self, EncodingError> {
        let len = Varint::<u32>::decode(&mut r)?.0;
        (0..len).map(|_| T::decode(&mut r)).collect()
    }
}

impl Encode for &str {
    fn size(&self) -> u32 {
        self.as_bytes().size()
    }

    fn encode<W: Write>(&self, w: W) -> Result<(), EncodingError> {
        self.as_bytes().encode(w)
    }
}

impl Encode for String {
    fn size(&self) -> u32 {
        self.as_bytes().size()
    }

    fn encode<W: Write>(&self, w: W) -> Result<(), EncodingError> {
        self.as_bytes().encode(w)
    }
}

impl Decode for String {
    fn decode<R: Read>(r: R) -> Result<Self, EncodingError> {
        Ok(String::from_utf8(Vec::<u8>::decode(r)?)?)
    }
}

impl Encode for () {
    fn size(&self) -> u32 {
        0
    }

    fn encode<W: Write>(&self, _w: W) -> Result<(), EncodingError> {
        Ok(())
    }
}

impl Decode for () {
    fn decode<R: Read>(_r: R) -> Result<Self, EncodingError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varints() {
        // Varint<u32>
        let cases: &[(u32, &[u8])] = &[
            (1, &[1]),
            (300, &[0b10101100, 0b00000010]),
            (u32::MAX, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
        ];
        for (input, expected) in cases {
            let varint = Varint(*input);
            let mut vec = vec![];
            varint.encode(&mut vec).unwrap();
            assert_eq!(&vec, expected);
            let decoded = <Varint<u32>>::decode(vec.as_slice()).unwrap();
            assert_eq!(decoded.0, *input);
        }

        // Varint<u64>
        let cases: &[(u64, &[u8])] = &[
            (1, &[1]),
            (300, &[0b10101100, 0b00000010]),
            (
                u64::MAX,
                &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
        ];
        for (input, expected) in cases {
            let varint = Varint(*input);
            let mut vec = vec![];
            varint.encode(&mut vec).unwrap();
            assert_eq!(&vec, expected);
            let decoded = <Varint<u64>>::decode(vec.as_slice()).unwrap();
            assert_eq!(decoded.0, *input);
        }
    }
}
