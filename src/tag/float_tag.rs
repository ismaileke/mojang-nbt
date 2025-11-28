use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug)]
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

    pub fn read(serializer: &mut NBTSerializer) -> FloatTag {
        let float = serializer.read_float();
        FloatTag{ value: float }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_float(self.get_value())
    }
}
