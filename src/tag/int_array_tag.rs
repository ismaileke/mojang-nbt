use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_INT_ARRAY;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

pub struct IntArrayTag {
    value: Vec<u32>
}

impl Tag for IntArrayTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<IntArrayTag>()
    }

    fn get_value(&self) -> Vec<u32> {
        self.value.clone() // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_INT_ARRAY
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_int_array(self.get_value())
    }
}

impl IntArrayTag {

    pub fn new(value: Vec<u32>) -> Self {
        IntArrayTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> IntArrayTag {
        let int_array = serializer.read_int_array();

        IntArrayTag{ value: int_array }
    }
}
