use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt::{NBT, TAG_COMPOUND, TAG_END, TAG_LIST};
use crate::tag::byte_array_tag::ByteArrayTag;
use crate::tag::byte_tag::ByteTag;
use crate::tag::double_tag::DoubleTag;
use crate::tag::float_tag::FloatTag;
use crate::tag::int_array_tag::IntArrayTag;
use crate::tag::int_tag::IntTag;
use crate::tag::list_tag::ListTag;
use crate::tag::long_tag::LongTag;
use crate::tag::short_tag::ShortTag;
use crate::tag::string_tag::StringTag;
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CompoundTag {
    value: HashMap<String, Box<dyn Tag>>
}

impl Tag for CompoundTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<CompoundTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        let cloned_map: HashMap<String, Box<dyn Tag>> = self.value.iter().map(|(name, tag)| (name.clone(), tag.clone_box())).collect();
        Box::new(cloned_map)
    }

    fn get_type(&self) -> u8 {
        TAG_COMPOUND
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        for (name, tag) in &self.value {
            serializer.write_byte(tag.get_type());
            serializer.write_string(name.clone());
            tag.write(serializer);
        }
        serializer.write_byte(TAG_END);
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}

impl CompoundTag {

    pub fn new(value: HashMap<String, Box<dyn Tag>>) -> Self {
        CompoundTag{ value }
    }

    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> CompoundTag {
        let mut compound_tag = Self::new(HashMap::new());

        loop {
            let tag_type = serializer.read_byte();
            if tag_type == TAG_END {
                break;
            }

            let name = serializer.read_string();
            let tag = NBT::create_tag(tag_type, serializer/*, tracker*/).expect("NBT Tag creation error, Compound Tag read fn");

            // If a tag with the same name already exists, skip it.
            // This is technically a corruption case, but it's very common on older PM worlds
            // (pretty much every furnace in PM worlds prior to 2017 is affected),
            // and since we can't extricate this worked data from the rest in Anvil/McRegion worlds,
            // we can't throw an error - it would result in complete loss of the chunk.
            // TODO: Add a flag to enable throwing on this (strict mode).
            if compound_tag.get_tag(name.clone()).is_some() {
                continue;
            }

            compound_tag.set_tag(name, tag);
        }

        compound_tag
    }
    pub fn count(&self) -> usize {
        self.value.len()
    }

    pub fn get_tag(&self, name: String) -> Option<&Box<dyn Tag>> {
        self.value.get(&name)
    }

    pub fn get_list_tag(&self, name: String) -> Option<ListTag> {
        let tag = self.value.get(&name);

        if let Some(list_tag) = tag {
            return if list_tag.get_type() == TAG_LIST {
                Option::from(list_tag.as_any().downcast_ref::<ListTag>().unwrap()).cloned()
            } else {
                None
            }
        }
        None
    }

    pub fn get_compound_tag(&self, name: String) -> Option<CompoundTag> {
        let tag = self.value.get(&name);

        if let Some(compound_tag) = tag {
            return if compound_tag.get_type() == TAG_COMPOUND {
                Option::from(compound_tag.as_any().downcast_ref::<CompoundTag>().unwrap()).cloned()
            } else {
                None
            }
        }
        None
    }

    pub fn set_tag(&mut self, name: String, tag: Box<dyn Tag>) -> &mut Self {
        self.value.insert(name, tag);
        self
    }

    pub fn remove_tag(&mut self, names: Vec<String>) {
        for name in &names {
            self.value.remove(name);
        }
    }

    pub fn get_tag_value<T: 'static + Tag>(&self, name: &str) -> Option<Box<dyn Any>> {
        let tag = self.value.get(name)?;

        if tag.get_type_id() == TypeId::of::<T>() {
            Some(tag.get_value())
        } else {
            None
        }
    }

    pub fn get_byte(&self, name: &str) -> Option<i8> {          //  u8    ?????????????????????????
        let value = self.get_tag_value::<ByteTag>(name)?;

        if let Some(value) = value.downcast_ref::<i8>() {
            return Some(*value);
        }

        None
    }

    pub fn get_short(&self, name: &str) -> Option<i16> {
        let value = self.get_tag_value::<ShortTag>(name)?;

        if let Some(value) = value.downcast_ref::<i16>() {
            return Some(*value);
        }

        None
    }

    pub fn get_int(&self, name: &str) -> Option<u32> {
        let value = self.get_tag_value::<IntTag>(name)?;

        if let Some(value) = value.downcast_ref::<u32>() {
            return Some(*value);
        }

        None
    }

    pub fn get_long(&self, name: &str) -> Option<i64> {
        let value = self.get_tag_value::<LongTag>(name)?;

        if let Some(value) = value.downcast_ref::<i64>() {
            return Some(*value);
        }

        None
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        let value = self.get_tag_value::<FloatTag>(name)?;

        if let Some(value) = value.downcast_ref::<f32>() {
            return Some(*value);
        }

        None
    }

    pub fn get_double(&self, name: &str) -> Option<f64> {
        let value = self.get_tag_value::<DoubleTag>(name)?;

        if let Some(value) = value.downcast_ref::<f64>() {
            return Some(*value);
        }

        None
    }

    pub fn get_byte_array(&self, name: &str) -> Option<Vec<u8>> {
        let value = self.get_tag_value::<ByteArrayTag>(name)?;

        if let Some(value) = value.downcast_ref::<Vec<u8>>() {
            return Some(value.clone());
        }

        None
    }

    pub fn get_string(&self, name: &str) -> Option<String> {
        let value = self.get_tag_value::<StringTag>(name)?;

        if let Some(value) = value.downcast_ref::<String>() {
            return Some(value.clone());
        }

        None
    }

    pub fn get_int_array(&self, name: &str) -> Option<Vec<u32>> { // EDIT AGAIN
        let value = self.get_tag_value::<IntArrayTag>(name)?;

        if let Some(value) = value.downcast_ref::<Vec<u32>>() {
            return Some(value.clone());
        }

        None
    }

    pub fn set_byte(&mut self, name: String, value: i8) -> &mut Self {
        self.set_tag(name, Box::new(ByteTag::new(value)))
    }

    pub fn set_short(&mut self, name: String, value: i16) -> &mut Self {
        self.set_tag(name, Box::new(ShortTag::new(value)))
    }

    pub fn set_int(&mut self, name: String, value: u32) -> &mut Self {
        self.set_tag(name, Box::new(IntTag::new(value)))
    }

    pub fn set_long(&mut self, name: String, value: i64) -> &mut Self {
        self.set_tag(name, Box::new(LongTag::new(value)))
    }

    pub fn set_float(&mut self, name: String, value: f32) -> &mut Self {
        self.set_tag(name, Box::new(FloatTag::new(value)))
    }

    pub fn set_double(&mut self, name: String, value: f64) -> &mut Self {
        self.set_tag(name, Box::new(DoubleTag::new(value)))
    }

    pub fn set_byte_array(&mut self, name: String, value: Vec<u8>) -> &mut Self {
        self.set_tag(name, Box::new(ByteArrayTag::new(value)))
    }

    pub fn set_string(&mut self, name: String, value: String) -> &mut Self {
        self.set_tag(name, Box::new(StringTag::new(value)))
    }

    pub fn set_int_array(&mut self, name: String, value: Vec<u32>) -> &mut Self {
        self.set_tag(name, Box::new(IntArrayTag::new(value)))
    }

	pub fn merge(&self, other: CompoundTag) -> CompoundTag {
        let mut new = self.clone();

        for (name, named_tag) in &other.value {
            new.set_tag(name.clone(), named_tag.clone_box());
        }

		new
	}
}

/*impl Clone for CompoundTag {
    fn clone(&self) -> Self {
        Self {
            value: self.value.iter().map(|(k, v)| (k.clone(), v.clone_box())).collect(),
        }
    }
}*/