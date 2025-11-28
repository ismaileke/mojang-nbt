use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug, serde::Serialize)]
pub struct LongTag {
    value: i64
}

impl LongTag {

    pub fn new(value: i64) -> Self {
        LongTag{ value }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn read(serializer: &mut NBTSerializer) -> LongTag {
        let long = serializer.read_long();
        LongTag { value: long }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_long(self.get_value());
    }
}
