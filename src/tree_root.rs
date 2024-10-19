use crate::tag::compound_tag::CompoundTag;
use crate::tag::tag::Tag;

pub struct TreeRoot {
    name: String,
    root: Box<dyn Tag>
}

impl TreeRoot {
    pub fn new(root: Box<dyn Tag>, name: String) -> Box<TreeRoot> {
        Box::new(TreeRoot{ name, root })
    }

    pub fn must_get_compound_tag(&self) -> Option<CompoundTag> {
        self.root.as_any().downcast_ref::<CompoundTag>().cloned()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_tag(&self) -> Box<dyn Tag> {
        self.root.clone_box()
    }
}