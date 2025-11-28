use crate::nbt_serializer::NBTSerializer;
use crate::nbt::NBT;
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
use crate::tag::tag::{Tag, TagValue};
use linked_hash_map::LinkedHashMap;
use serde::ser::SerializeMap;
use serde::Serializer;

#[derive(Clone, Debug, serde::Serialize)]
pub struct CompoundTag {
    #[serde(serialize_with = "serialize_map")]
    value: LinkedHashMap<String, Tag>
}

impl CompoundTag {

    pub fn new(value: LinkedHashMap<String, Tag>) -> Self {
        CompoundTag{ value }
    }

    pub fn read(serializer: &mut NBTSerializer) -> CompoundTag {
        let mut compound_tag = Self::new(LinkedHashMap::new());

        loop {
            let tag_type = serializer.read_byte();
            if tag_type == NBT::TAG_END {
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

    pub fn write(&self, serializer: &mut NBTSerializer) {
        for (name, tag) in &self.value {
            serializer.write_byte(tag.get_id());
            serializer.write_string(name.clone());
            tag.write(serializer);
        }
        serializer.write_byte(NBT::TAG_END);
    }

    pub fn count(&self) -> usize {
        self.value.len()
    }

    pub fn get_tag(&self, name: String) -> Option<&Tag> {
        self.value.get(&name)
    }

    pub fn get_list_tag(&self, name: String) -> Option<ListTag> {
        let opt_tag = self.value.get(&name);

        if let Some(tag) = opt_tag {
            return if let Tag::List(list_tag) = tag {
                Option::from(list_tag.clone())
            } else { None }
        }
        None
    }

    pub fn get_compound_tag(&self, name: String) -> Option<CompoundTag> {
        let opt_tag = self.value.get(&name);

        if let Some(tag) = opt_tag {
            return if let Tag::Compound(compound_tag) = tag {
                Option::from(compound_tag.clone())
            } else { None }
        }
        None
    }

    pub fn set_tag(&mut self, name: String, tag: Tag) -> &mut Self {
        self.value.insert(name, tag);
        self
    }

    pub fn remove_tag(&mut self, names: Vec<String>) {
        for name in &names {
            self.value.remove(name);
        }
    }

    pub fn get_tag_value(&self, name: &str) -> TagValue {
        let tag = self.value.get(name).expect(&format!("Tag {} not found", name));
        tag.get_value()
    }

    pub fn get_byte(&self, name: &str) -> Option<i8> {
        let value = self.get_tag_value(name);
        if let TagValue::Byte(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_short(&self, name: &str) -> Option<i16> {
        let value = self.get_tag_value(name);
        if let TagValue::Short(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        let value = self.get_tag_value(name);
        if let TagValue::Int(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_long(&self, name: &str) -> Option<i64> {
        let value = self.get_tag_value(name);
        if let TagValue::Long(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        let value = self.get_tag_value(name);
        if let TagValue::Float(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_double(&self, name: &str) -> Option<f64> {
        let value = self.get_tag_value(name);
        if let TagValue::Double(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_byte_array(&self, name: &str) -> Option<Vec<u8>> {
        let value = self.get_tag_value(name);
        if let TagValue::ByteArray(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_string(&self, name: &str) -> Option<String> {
        let value = self.get_tag_value(name);
        if let TagValue::String(v) = value {
            return Some(v);
        }

        None
    }

    pub fn get_int_array(&self, name: &str) -> Option<Vec<i32>> { // EDIT AGAIN
        let value = self.get_tag_value(name);
        if let TagValue::IntArray(v) = value {
            return Some(v);
        }

        None
    }

    pub fn set_byte(&mut self, name: String, value: i8) -> &mut Self {
        self.set_tag(name, Tag::Byte(ByteTag::new(value)))
    }

    pub fn set_short(&mut self, name: String, value: i16) -> &mut Self {
        self.set_tag(name, Tag::Short(ShortTag::new(value)))
    }

    pub fn set_int(&mut self, name: String, value: i32) -> &mut Self {
        self.set_tag(name, Tag::Int(IntTag::new(value)))
    }

    pub fn set_long(&mut self, name: String, value: i64) -> &mut Self {
        self.set_tag(name, Tag::Long(LongTag::new(value)))
    }

    pub fn set_float(&mut self, name: String, value: f32) -> &mut Self {
        self.set_tag(name, Tag::Float(FloatTag::new(value)))
    }

    pub fn set_double(&mut self, name: String, value: f64) -> &mut Self {
        self.set_tag(name, Tag::Double(DoubleTag::new(value)))
    }

    pub fn set_byte_array(&mut self, name: String, value: Vec<u8>) -> &mut Self {
        self.set_tag(name, Tag::ByteArray(ByteArrayTag::new(value)))
    }

    pub fn set_string(&mut self, name: String, value: String) -> &mut Self {
        self.set_tag(name, Tag::String(StringTag::new(value)))
    }

    pub fn set_int_array(&mut self, name: String, value: Vec<i32>) -> &mut Self {
        self.set_tag(name, Tag::IntArray(IntArrayTag::new(value)))
    }

	pub fn merge(&self, other: CompoundTag) -> CompoundTag {
        let mut new = self.clone();
        for (name, named_tag) in &other.value {
            new.set_tag(name.clone(), named_tag.clone());
        }

		new
	}

    pub fn get_value(&self) -> LinkedHashMap<String, Tag> {
        self.value.iter().map(|(name, tag)| (name.clone(), tag.clone())).collect()
    }
}

fn serialize_map<S>(map: &LinkedHashMap<String, Tag>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut s = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        s.serialize_entry(k, v)?;
    }
    s.end()
}
