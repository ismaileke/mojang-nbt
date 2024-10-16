use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_BYTE_ARRAY;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

pub struct ByteArrayTag {
    value: Vec<u8>
}

impl Tag for ByteArrayTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ByteArrayTag>()
    }

    fn get_value(&self) -> Vec<u8> {
        self.value.clone() // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_BYTE_ARRAY
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_byte_array(self.get_value());
    }
}

impl ByteArrayTag {

    pub fn new(value: Vec<u8>) -> Self {
       ByteArrayTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> ByteArrayTag {
        let byte_array = serializer.read_byte_array();

        ByteArrayTag{ value: byte_array }
    }
}
