use crate::base_nbt_serializer::BaseNBTSerializer;
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

pub struct NBT {}

impl NBT {
    pub fn create_tag(tag_type: u8, serializer: &mut dyn BaseNBTSerializer) -> Option<Box<dyn Tag>> {
        match tag_type {
            TAG_BYTE => Option::from(ByteTag::read(serializer).clone_box()),
            TAG_SHORT => Option::from(ShortTag::read(serializer).clone_box()),
            TAG_INT => Option::from(IntTag::read(serializer).clone_box()),
            TAG_LONG => Option::from(LongTag::read(serializer).clone_box()),
            TAG_FLOAT => Option::from(FloatTag::read(serializer).clone_box()),
            TAG_DOUBLE => Option::from(DoubleTag::read(serializer).clone_box()),
            TAG_BYTE_ARRAY => Option::from(ByteArrayTag::read(serializer).clone_box()),
            TAG_STRING => Option::from(StringTag::read(serializer).clone_box()),
            TAG_LIST => Option::from(ListTag::read(serializer).clone_box()),
            TAG_COMPOUND => Option::from(CompoundTag::read(serializer).clone_box()),
            TAG_INT_ARRAY => Option::from(IntArrayTag::read(serializer).clone_box()),

            _ => { panic!("Unknown NBT tag type {}", tag_type) }
        }
    }
}
