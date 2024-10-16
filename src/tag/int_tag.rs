use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_INT;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

pub struct IntTag {
    value: u32
}

impl Tag for IntTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<IntTag>()
    }

    fn get_value(&self) -> u32 {
        self.value // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_INT
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_int(self.get_value())
    }
}

impl IntTag {

    pub fn new(value: u32) -> Self {
        IntTag{ value }
    }

    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> IntTag {
        let integer = serializer.read_int();

        IntTag{ value: integer }
    }
}
