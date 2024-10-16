use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::TAG_STRING;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};

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

    fn get_value(&self) -> String {
        self.value.clone() // for now 'clone'
    }

    fn get_type(&self) -> u8 {
        TAG_STRING
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_string(self.get_value())
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
