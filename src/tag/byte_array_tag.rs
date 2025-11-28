use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug)]
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

    pub fn read(serializer: &mut NBTSerializer) -> ByteArrayTag {
        let byte_array = serializer.read_byte_array();

        ByteArrayTag{ value: byte_array }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_byte_array(self.get_value());
    }
}
