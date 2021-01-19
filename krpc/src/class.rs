use protobuf_but_worse::encoding::{Decode, Encode, EncodingError, Varint};

pub struct Class(Varint<u64>);

impl Decode for Class {
    fn decode<R: std::io::Read>(r: R) -> Result<Self, EncodingError> {
        <Varint<u64>>::decode(r).map(Class)
    }
}

impl Encode for Class {
    fn size(&self) -> u32 {
        self.0.size()
    }

    fn encode<W: std::io::Write>(&self, w: W) -> Result<(), EncodingError> {
        self.0.encode(w)
    }
}
