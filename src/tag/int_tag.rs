use crate::nbt_serializer::{NBTReader, NBTWriter};

#[derive(Clone, Debug, serde::Serialize)]
pub struct IntTag {
    value: i32
}

impl IntTag {
    pub fn new(value: i32) -> Self {
        IntTag { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn read(serializer: &mut NBTReader) -> IntTag {
        let integer = serializer.read_int();
        IntTag { value: integer }
    }

    pub fn write(&self, serializer: &mut NBTWriter) {
        serializer.write_int(self.get_value());
    }
}
