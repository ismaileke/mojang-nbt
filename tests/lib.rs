#[cfg(test)]

mod tests {
    extern crate mojang_nbt;

    use mojang_nbt::big_endian_nbt_serializer::BigEndianNBTSerializer;
    use mojang_nbt::nbt::{NBT, TAG_COMPOUND};

    #[test]
    fn test() {
        let mut serializer = BigEndianNBTSerializer::new();
        let tag = NBT::create_tag(TAG_COMPOUND, &mut serializer);
        println!("result: {:?}", tag.unwrap());

    }
}