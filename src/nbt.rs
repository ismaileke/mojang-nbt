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
use crate::tag::tag::Tag;

pub struct NBT;

impl NBT {
    pub const TAG_END: u8 = 0;
    pub const TAG_BYTE: u8 = 1;
    pub const TAG_SHORT: u8 = 2;
    pub const TAG_INT: u8 = 3;
    pub const TAG_LONG: u8 = 4;
    pub const TAG_FLOAT: u8 = 5;
    pub const TAG_DOUBLE: u8 = 6;
    pub const TAG_BYTE_ARRAY: u8 = 7;
    pub const TAG_STRING: u8 = 8;
    pub const TAG_LIST: u8 = 9;
    pub const TAG_COMPOUND: u8 = 10;
    pub const TAG_INT_ARRAY: u8 = 11;

    pub fn create_tag(tag_type: u8, serializer: &mut NBTSerializer) -> Option<Tag> {
        match tag_type {
            Self::TAG_BYTE => Option::from(Tag::Byte(ByteTag::read(serializer))),
            Self::TAG_SHORT => Option::from(Tag::Short(ShortTag::read(serializer))),
            Self::TAG_INT => Option::from(Tag::Int(IntTag::read(serializer))),
            Self::TAG_LONG => Option::from(Tag::Long(LongTag::read(serializer))),
            Self::TAG_FLOAT => Option::from(Tag::Float(FloatTag::read(serializer))),
            Self::TAG_DOUBLE => Option::from(Tag::Double(DoubleTag::read(serializer))),
            Self::TAG_BYTE_ARRAY => Option::from(Tag::ByteArray(ByteArrayTag::read(serializer))),
            Self::TAG_STRING => Option::from(Tag::String(StringTag::read(serializer))),
            Self::TAG_LIST => Option::from(Tag::List(ListTag::read(serializer))),
            Self::TAG_COMPOUND => Option::from(Tag::Compound(CompoundTag::read(serializer))),
            Self::TAG_INT_ARRAY => Option::from(Tag::IntArray(IntArrayTag::read(serializer))),
            _ => { panic!("Unknown NBT tag type {}", tag_type) }
        }
    }
}
