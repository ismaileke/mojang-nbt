use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_SHORT;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
pub struct ShortTag {
    value: i16
}

impl Tag for ShortTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ShortTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn get_type(&self) -> u8 {
        TAG_SHORT
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        if let Some(value) = self.get_value().downcast_ref::<i16>() {
            serializer.write_short(*value);
        } else {
            panic!("Failed to downcast to ShortTag");
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}

impl ShortTag {

    pub fn new(value: i16) -> Self {
        ShortTag{ value }
    }
    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> ShortTag {
        let signed_short = serializer.read_signed_short();

        ShortTag { value: signed_short }
    }
}
