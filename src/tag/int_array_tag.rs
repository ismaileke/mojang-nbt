use crate::nbt_serializer::{NBTReader, NBTWriter};

#[derive(Clone, Debug, serde::Serialize)]
pub struct IntArrayTag {
    value: Vec<i32>
}

impl IntArrayTag {

    pub fn new(value: Vec<i32>) -> Self {
        IntArrayTag { value }
    }

    pub fn get_value(&self) -> Vec<i32> {
        self.value.clone()
    }

    pub fn read(serializer: &mut NBTReader) -> IntArrayTag {
        let int_array = serializer.read_int_array();

        IntArrayTag { value: int_array }
    }

    pub fn write(&self, serializer: &mut NBTWriter) {
        serializer.write_int_array(self.get_value())
    }
}
