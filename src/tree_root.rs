use crate::tag::compound_tag::CompoundTag;
use crate::tag::tag::Tag;

pub struct TreeRoot<'a> {
    name: &'a str,
    root: Tag
}

impl<'a> TreeRoot<'a> {
    pub fn new(root: Tag, name: &'a str) -> TreeRoot<'a> {
        TreeRoot{ name, root }
    }

    pub fn must_get_compound_tag(&self) -> Option<CompoundTag> {
        match &self.root {
            Tag::Compound(tag) => Some(tag.clone()),
            _ => None
        }
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_tag(&self) -> &Tag {
        &self.root
    }
}
