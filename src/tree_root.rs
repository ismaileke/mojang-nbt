use crate::tag::compound_tag::CompoundTag;
use crate::tag::tag::Tag;

pub struct TreeRoot {
    name: String,
    root: Tag
}

impl TreeRoot {
    pub fn new(root: Tag, name: String) -> TreeRoot {
        TreeRoot{ name, root }
    }

    pub fn must_get_compound_tag(&self) -> Option<CompoundTag> {
        match &self.root {
            Tag::Compound(tag) => Some(tag.clone()),
            _ => None
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_tag(&self) -> Tag {
        self.root.clone()
    }
}
