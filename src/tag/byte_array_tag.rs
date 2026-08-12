use crate::nbt_serializer::{NBTReader, NBTWriter};

#[derive(Clone, Debug, serde::Serialize)]
pub struct ByteArrayTag {
    value: Vec<u8>
}

impl ByteArrayTag {

    pub fn new(value: Vec<u8>) -> Self {
       ByteArrayTag{ value }
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn read(serializer: &mut NBTReader) -> ByteArrayTag {
        let byte_array = serializer.read_byte_array();

        ByteArrayTag{ value: byte_array.to_vec() }
    }

    pub fn write(&self, serializer: &mut NBTWriter) {
        serializer.write_byte_array(self.get_value().as_slice());
    }
}
