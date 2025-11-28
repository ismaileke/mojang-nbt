use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug)]
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

    pub fn read(serializer: &mut NBTSerializer) -> IntTag {
        let integer = serializer.read_int();
        IntTag { value: integer }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_int(self.get_value());
    }
}
