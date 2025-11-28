use crate::nbt::NBT;
use crate::tag::tag::{Tag, TagValue};
use std::collections::HashMap;
use crate::nbt_serializer::NBTSerializer;

#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTag {
    tag_type: u8,
    value: Vec<Tag>,
}

impl ListTag {
    pub fn new(value: Vec<Tag>, tag_type: u8) -> Self {
        let list_tag = Self {
            tag_type,
            value: Vec::from_iter(value),
        };
        list_tag.validate_tag_type();
        list_tag
    }

    pub fn get_all_values(&self) -> HashMap<usize, TagValue> {
        let mut values = HashMap::new();

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

    pub fn push(&mut self, tag: Tag) {
        self.check_tag_type(&tag);
        self.value.push(tag);
    }

    pub fn pop(&mut self) -> Option<Tag> {
        self.value.pop()
    }

    pub fn unshift(&mut self, tag: Tag) {
        self.check_tag_type(&tag);
        self.value.push(tag);
    }

    pub fn shift(&mut self) -> Option<Tag> {
        self.value.pop()
    }

    pub fn insert(&mut self, index: usize, tag: Tag) {
        self.check_tag_type(&tag);
        self.value.insert(index, tag);
    }

    pub fn remove(&mut self, index: usize) {
        self.value.remove(index);
    }

    pub fn get(&self, index: usize) -> Tag {
        self.value[index].clone()
    }

    pub fn first(&self) -> Option<Tag> {
        Option::from(self.value.first().unwrap().clone())
    }

    pub fn last(&self) -> Option<Tag> {
        Option::from(self.value.last().unwrap().clone())
    }

    pub fn set(&mut self, index: usize, tag: Tag) {
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

    fn check_tag_type(&self, tag: &Tag) {
        let tag_type = tag.get_id();

        if tag_type != self.tag_type {
            panic!("Invalid tag of type {:?} assigned to ListTag", tag_type);
        }
    }

    fn validate_tag_type(&self) {
        if self.tag_type == NBT::TAG_END && !self.value.is_empty() {
            panic!("Cannot have a non-empty ListTag with tag type TAG_End");
        }
    }

    pub fn read(serializer: &mut NBTSerializer) -> Self { // EDIT AGAIN?
        let mut tag_type = serializer.read_byte();
        let size = serializer.read_int();

        let mut value: Vec<Tag> = Vec::new();

        if size > 0 {
            if tag_type == NBT::TAG_END {
                panic!("Unexpected non-empty list of TAG_End");
            }
            for _ in 0..size {
                value.push(NBT::create_tag(tag_type, serializer/*, tracker*/).unwrap());
            }
        } else {
            tag_type = NBT::TAG_END; //Some older NBT implementations used TAG_Byte for empty lists.
        }

        Self::new(value, tag_type)
    }

    pub fn write(&self, serializer: &mut NBTSerializer) {
        serializer.write_byte(NBT::TAG_LIST); // I hope that's right
        serializer.write_int(self.count() as i32);
        for tag in &self.value {
            tag.write(serializer);
        }
    }

    pub fn get_value(&self) -> Vec<Tag> {
        self.value.iter().map(|tag| tag.clone()).collect()
    }
}
