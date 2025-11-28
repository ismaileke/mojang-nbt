#[cfg(test)]

mod tests {
    extern crate mojang_nbt;

    use mojang_nbt::tag::compound_tag::CompoundTag;
    use mojang_nbt::tree_root::TreeRoot;
    use linked_hash_map::LinkedHashMap;
    use mojang_nbt::nbt::NBT;
    use mojang_nbt::tag::byte_tag::ByteTag;
    use mojang_nbt::tag::list_tag::ListTag;
    use mojang_nbt::tag::tag::Tag;

    #[test]
    fn test() {
        let compound_tag = CompoundTag::new(LinkedHashMap::new())
            .set_byte("byte".to_string(), -128i8)
            .set_short("short".to_string(), 1i16)
            .set_int("int".to_string(), 1i32)
            .set_long("long".to_string(), 132323i64)
            .set_float("float".to_string(), 1f32)
            .set_double("double".to_string(), 1f64)
            .set_byte_array("bytearray".to_string(), vec![1u8])
            .set_string("string".to_string(), "string".to_string())
            .set_tag("list".to_string(), Tag::List(ListTag::new(vec![Tag::Byte(ByteTag::new(1i8))], NBT::TAG_LIST)))
            .set_int_array("custom_int_array".to_string(), vec![1, 2, 3, 5, 76, 56, 443]).clone();


        let root = TreeRoot::new(Tag::Compound(compound_tag), "CustomCompoundName".to_string());
        let must_compound_tag = root.must_get_compound_tag();
        if let Some(compound_tag) = must_compound_tag {
            println!("Result: {:?}", compound_tag.get_int_array("custom_int_array").expect("get_int_array() error"));
        }


        /*let mut serializer = NBTSerializer::new_big_endian();
        let dat = serializer.write(root);

        let mut serializer2 = NBTSerializer::new_big_endian();
        let root2 = serializer2.read(dat, &mut 0, 0);*/

        //self::assertTrue($root->equals($root2));
    }
}
