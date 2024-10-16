use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_BYTE;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

pub struct ByteTag {
    value: i8
}

impl Tag for ByteTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ByteTag>()
    }

    fn get_value(&self) -> i8 {
        self.value
    }

    fn get_type(&self) -> u8 {
        TAG_BYTE
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_byte(self.get_value() as u8)
    }
}

impl ByteTag {

    pub fn new(value: i8) -> Self {
        ByteTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> ByteTag {
        let byte = serializer.read_signed_byte();
        ByteTag{ value: byte }
    }
}