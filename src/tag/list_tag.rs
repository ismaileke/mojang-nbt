use crate::base_nbt_serializer::BaseNBTSerializer;
use crate::nbt;
use crate::nbt::{NBT, TAG_LIST};
use crate::tag::tag::Tag;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct ListTag {
    tag_type: u8,
    value: Vec<Box<dyn Tag>>,
}

impl Tag for ListTag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListTag>()
    }

    fn get_value(&self) -> Box<dyn Any> {
        let cloned_vec: Vec<Box<dyn Tag>> = self.value.iter().map(|tag| tag.clone_box()).collect();
        Box::new(cloned_vec)
    }

    fn get_type(&self) -> u8 {
        TAG_LIST
    }

    fn write(&self, serializer: &mut dyn BaseNBTSerializer) {
        serializer.write_byte(self.get_type());
        serializer.write_int(self.count() as u32);
        for tag in &self.value {
            tag.write(serializer);
        }
    }

    fn clone_box(&self) -> Box<dyn Tag> {
        Box::new(self.clone())
    }
}

impl ListTag {
    pub fn new(value: Vec<Box<dyn Tag>>, tag_type: u8) -> Self {
        let list_tag = Self {
            tag_type,
            value: Vec::from_iter(value),
        };
        list_tag.validate_tag_type();
        list_tag
    }

    pub fn get_all_values(&self) -> HashMap<usize, Box<dyn Any>> {
        let mut values: HashMap<usize, Box<dyn Any>> = HashMap::new();

        let mut i: usize = 0;
        for tag in &self.value {
            values.insert(i, tag.get_value());
            i += 1;
        }

        values
    }

    pub fn count(&self) -> usize {
        self.value.len()
    }

    pub fn push(&mut self, tag: Box<dyn Tag>) {
        self.check_tag_type(&tag);
        self.value.push(tag);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Tag>> {
        self.value.pop()
    }

    pub fn unshift(&mut self, tag: Box<dyn Tag>) {
        self.check_tag_type(&tag);
        self.value.push(tag);
    }

    pub fn shift(&mut self) -> Option<Box<dyn Tag>> {
        self.value.pop()
    }

    pub fn insert(&mut self, index: usize, tag: Box<dyn Tag>) {
        self.check_tag_type(&tag);
        self.value.insert(index, tag);
    }

    pub fn remove(&mut self, index: usize) {
        self.value.remove(index);
    }

    pub fn get(&self, index: usize) -> Box<dyn Tag> {
        self.value[index].clone_box() // Cloned
    }

    pub fn first(&self) -> Option<Box<dyn Tag>> {
        Option::from(self.value.first().unwrap().clone_box())
    }

    pub fn last(&self) -> Option<Box<dyn Tag>> {
        Option::from(self.value.last().unwrap().clone_box())
    }

    pub fn set(&mut self, index: usize, tag: Box<dyn Tag>) {
        self.check_tag_type(&tag);
        self.value[index] = tag;
    }

    pub fn isset(&self, index: usize) -> bool {
        index < self.value.len()
    }

    pub fn empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn get_tag_type(&self) -> u8 {
        self.tag_type
    }

    pub fn set_tag_type(&mut self, tag_type: u8) {
        if !self.value.is_empty() {
            panic!("Cannot change tag type of non-empty ListTag");
        }
        self.tag_type = tag_type;
    }

    fn check_tag_type(&self, tag: &Box<dyn Tag>) {
        let tag_type = tag.get_type();

        if tag_type != self.tag_type {
            panic!("Invalid tag of type {:?} assigned to ListTag", tag_type);
        }
    }

    fn validate_tag_type(&self) {
        if self.tag_type == nbt::TAG_END && !self.value.is_empty() {
            panic!("Cannot have a non-empty ListTag with tag type TAG_End");
        }
    }

    pub fn read(serializer: &mut dyn BaseNBTSerializer) -> Self { // EDIT AGAIN?
        let mut tag_type = serializer.read_byte();
        let size = serializer.read_int();

        let mut value: Vec<Box<dyn Tag>> = Vec::new();

        if size > 0 {
            if tag_type == nbt::TAG_END {
                panic!("Unexpected non-empty list of TAG_End");
            }
            for _ in 0..size {
                value.push(NBT::create_tag(tag_type, serializer/*, tracker*/).unwrap());
            }
        } else {
            tag_type = nbt::TAG_END; //Some older NBT implementations used TAG_Byte for empty lists.
        }

        Self::new(value, tag_type)
    }
}

/*impl Clone for ListTag {
    fn clone(&self) -> Self {
        Self {
            tag_type: self.tag_type,
            value: self.value.iter().map(|tag| tag.clone_box()).collect(),
        }
    }
}*/
