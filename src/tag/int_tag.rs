use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_INT;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
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

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value) // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_INT
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<u32>() {
            serializer.write_int(*value)
        } else {
            panic!("Failed to downcast to IntTag");
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
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
