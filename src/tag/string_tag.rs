use crate::nbt_serializer::{NBTReader, NBTWriter};

#[derive(Clone, Debug, serde::Serialize)]
pub struct StringTag {
    value: String
}

impl StringTag {

    pub fn new(value: String) -> Self {
        StringTag{ value }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn read(serializer: &mut NBTReader) -> StringTag {
        let string_data = serializer.read_string();
        StringTag{ value: string_data.to_string() }
    }

    pub fn write(&self, serializer: &mut NBTWriter) {
        serializer.write_string(self.get_value().as_str());
    }
}
