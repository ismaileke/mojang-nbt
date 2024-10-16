use crate::base_nbt_serializer::BaseNBTSerializer;
use std::any::{Any, TypeId};

pub trait Tag: Any {

    fn as_any(&self) -> &dyn Any;

    fn get_type_id(&self) -> TypeId;

    fn get_value(&self) -> Box<dyn Any>;

    fn get_type(&self) -> u8;

    fn write(&self, serializer: &mut dyn BaseNBTSerializer);

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}