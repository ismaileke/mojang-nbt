use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_INT_ARRAY;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
pub struct IntArrayTag {
    value: Vec<i32>
}

impl Tag for IntArrayTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<IntArrayTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone()) // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_INT_ARRAY
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<Vec<i32>>() {
            serializer.write_int_array(value.to_vec())
        } else {
            panic!("Failed to downcast to IntArrayTag");
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}

impl IntArrayTag {

    pub fn new(value: Vec<i32>) -> Self {
        IntArrayTag { value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> IntArrayTag {
        let int_array = serializer.read_int_array();

        IntArrayTag { value: int_array }
    }
}
