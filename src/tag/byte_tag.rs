use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug, serde::Serialize)]
pub struct ByteTag {
    value: i8
}

impl ByteTag {

    pub fn new(value: i8) -> Self {
        ByteTag{ value }
    }

    pub fn get_value(&self) -> i8 {
        self.value
    }

    pub fn read(serializer: &mut NBTSerializer) -> ByteTag {
        let byte = serializer.read_signed_byte();
        ByteTag{ value: byte }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_byte(self.get_value() as u8);
    }
}