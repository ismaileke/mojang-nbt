use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_DOUBLE;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

pub struct DoubleTag {
    value: f64
}

impl Tag for DoubleTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<DoubleTag>()
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    fn get_type(&self) -> u8 {
        TAG_DOUBLE
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_double(self.get_value())
    }
}

impl DoubleTag {

    pub fn new(value: f64) -> Self {
        DoubleTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> DoubleTag {
        let double = serializer.read_double();
        DoubleTag{ value: double }
    }
}
