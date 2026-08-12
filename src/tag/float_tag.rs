use crate::nbt_serializer::{NBTReader, NBTWriter};

#[derive(Clone, Debug, serde::Serialize)]
pub struct FloatTag {
    value: f32
}

impl FloatTag {

    pub fn new(value: f32) -> Self {
        FloatTag{ value }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn read(serializer: &mut NBTReader) -> FloatTag {
        let float = serializer.read_float();
        FloatTag{ value: float }
    }

    pub fn write(&self, serializer: &mut NBTWriter) {
        serializer.write_float(self.get_value())
    }
}
