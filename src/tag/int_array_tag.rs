use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug)]
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

    pub fn read(serializer: &mut NBTSerializer) -> IntArrayTag {
        let int_array = serializer.read_int_array();

        IntArrayTag { value: int_array }
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_int_array(self.get_value())
    }
}
