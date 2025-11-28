use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::nbt::NBT;
use crate::nbt_serializer::NBTSerializer;
use crate::tag::byte_array_tag::ByteArrayTag;
use crate::tag::byte_tag::ByteTag;
use crate::tag::compound_tag::CompoundTag;
use crate::tag::double_tag::DoubleTag;
use crate::tag::float_tag::FloatTag;
use crate::tag::int_array_tag::IntArrayTag;
use crate::tag::int_tag::IntTag;
use crate::tag::list_tag::ListTag;
use crate::tag::long_tag::LongTag;
use crate::tag::short_tag::ShortTag;
use crate::tag::string_tag::StringTag;

#[derive(Clone, Debug, Serialize)]
pub enum Tag {
    ByteArray(ByteArrayTag),
    Byte(ByteTag),
    Compound(CompoundTag),
    Double(DoubleTag),
    Float(FloatTag),
    IntArray(IntArrayTag),
    Int(IntTag),
    List(ListTag),
    Long(LongTag),
    Short(ShortTag),
    String(StringTag)
}

#[derive(Clone, Debug, Serialize)]
pub enum TagValue {
    ByteArray(Vec<u8>),
    Byte(i8),
    #[serde(serialize_with = "serialize_map")]
    Compound(LinkedHashMap<String, Tag>),
    Double(f64),
    Float(f32),
    IntArray(Vec<i32>),
    Int(i32),
    List(Vec<Tag>),
    Long(i64),
    Short(i16),
    String(String)
}

impl Tag {
    pub fn get_id(&self) -> u8 {
        match self {
            Tag::ByteArray(_) => NBT::TAG_BYTE_ARRAY,
            Tag::Byte(_) => NBT::TAG_BYTE,
            Tag::Compound(_) => NBT::TAG_COMPOUND,
            Tag::Double(_) => NBT::TAG_DOUBLE,
            Tag::Float(_) => NBT::TAG_FLOAT,
            Tag::IntArray(_) => NBT::TAG_INT_ARRAY,
            Tag::Int(_) => NBT::TAG_INT,
            Tag::List(_) => NBT::TAG_LIST,
            Tag::Long(_) => NBT::TAG_LONG,
            Tag::Short(_) => NBT::TAG_SHORT,
            Tag::String(_) => NBT::TAG_STRING
        }
    }
    pub fn write(&self, serializer: &mut NBTSerializer) {
        match self {
            Tag::ByteArray(tag) => tag.write(serializer),
            Tag::Byte(tag) => tag.write(serializer),
            Tag::Compound(tag) => tag.write(serializer),
            Tag::Double(tag) => tag.write(serializer),
            Tag::Float(tag) => tag.write(serializer),
            Tag::IntArray(tag) => tag.write(serializer),
            Tag::Int(tag) => tag.write(serializer),
            Tag::List(tag) => tag.write(serializer),
            Tag::Long(tag) => tag.write(serializer),
            Tag::Short(tag) => tag.write(serializer),
            Tag::String(tag) => tag.write(serializer)
        }
    }

    pub fn get_value(&self) -> TagValue {
        match self {
            Tag::ByteArray(tag) => TagValue::ByteArray(tag.get_value()),
            Tag::Byte(tag) => TagValue::Byte(tag.get_value()),
            Tag::Compound(tag) => TagValue::Compound(tag.get_value()),
            Tag::Double(tag) => TagValue::Double(tag.get_value()),
            Tag::Float(tag) => TagValue::Float(tag.get_value()),
            Tag::IntArray(tag) => TagValue::IntArray(tag.get_value()),
            Tag::Int(tag) => TagValue::Int(tag.get_value()),
            Tag::List(tag) => TagValue::List(tag.get_value()),
            Tag::Long(tag) => TagValue::Long(tag.get_value()),
            Tag::Short(tag) => TagValue::Short(tag.get_value()),
            Tag::String(tag) => TagValue::String(tag.get_value())
        }
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
