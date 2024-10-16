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

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn get_type(&self) -> u8 {
        TAG_DOUBLE
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<f64>() {
            serializer.write_double(*value)
        } else {
            panic!("Failed to downcast to DoubleTag");
        }
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
