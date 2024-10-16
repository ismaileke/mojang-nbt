use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_FLOAT;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

pub struct FloatTag {
    value: f32
}

impl Tag for FloatTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<FloatTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn get_type(&self) -> u8 {
        TAG_FLOAT
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<f32>() {
            serializer.write_float(*value)
        } else {
            panic!("Failed to downcast to FloatTag");
        }
    }
}

impl FloatTag {

    pub fn new(value: f32) -> Self {
        FloatTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> FloatTag {
        let float = serializer.read_float();
        FloatTag{ value: float }
    }
}
