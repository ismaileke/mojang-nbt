use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_BYTE_ARRAY;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

#[derive(Clone)]
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

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone()) // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_BYTE_ARRAY
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<Vec<u8>>() {
            serializer.write_byte_array(value.to_vec());
        } else {
            panic!("Failed to downcast to ByteArrayTag");
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
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
