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

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn get_type(&self) -> u8 {
        TAG_BYTE
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<i8>() {
            serializer.write_byte(*value as u8);
        } else {
            panic!("Failed to downcast to ByteTag");
        }
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