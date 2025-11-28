use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug)]
pub struct DoubleTag {
    value: f64
}

impl DoubleTag {

    pub fn new(value: f64) -> Self {
        DoubleTag{ value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn read(serializer: &mut NBTSerializer) -> DoubleTag {
        let double = serializer.read_double();
        DoubleTag{ value: double }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_double(self.get_value());
    }
}
