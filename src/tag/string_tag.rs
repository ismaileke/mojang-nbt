use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_STRING;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

#[derive(Clone)]
pub struct StringTag {
    value: String
}

impl Tag for StringTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<StringTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone()) // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_STRING
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<String>() {
            serializer.write_string(value.to_string());
        } else {
            panic!("Failed to downcast to StringTag");
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}

impl StringTag {

    pub fn new(value: String) -> Self {
        StringTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> StringTag {
        let string_data = serializer.read_string();

        StringTag{ value: string_data }
    }
}
