use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug)]
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

    pub fn read(serializer: &mut NBTSerializer) -> StringTag {
        let string_data = serializer.read_string();
        StringTag{ value: string_data }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_string(self.get_value());
    }
}
