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

    fn get_value(&self) -> f32 {
        self.value
    }

    fn get_type(&self) -> u8 {
        TAG_FLOAT
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_float(self.get_value())
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
