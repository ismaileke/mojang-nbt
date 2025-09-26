use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_LONG;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
pub struct LongTag {
    value: i64
}

impl Tag for LongTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<LongTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn get_type(&self) -> u8 {
        TAG_LONG
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<i64>() {
            serializer.write_long(*value)
        } else {
            panic!("Failed to downcast to LongTag");
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}

impl LongTag {

    pub fn new(value: i64) -> Self {
        LongTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> LongTag {
        let long = serializer.read_long();
        LongTag { value: long }
    }
}
