#[cfg(test)]

mod tests {
    extern crate mojang_nbt;

    use mojang_nbt::nbt::TAG_COMPOUND;
    use mojang_nbt::tag::compound_tag::CompoundTag;
    use mojang_nbt::tree_root::TreeRoot;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let compound_tag = CompoundTag::new(HashMap::new())
            .set_byte("byte".to_string(), -128i8)
            .set_short("short".to_string(), 1i16)
            .set_int("int".to_string(), 1i32)
            .set_long("long".to_string(), 132323i64)
            .set_float("float".to_string(), 1f32)
            .set_double("double".to_string(), 1f64)
            .set_byte_array("bytearray".to_string(), vec![1u8])
            .set_string("string".to_string(), "string".to_string())
            /*.set_tag("list".to_string(), Box::new(ListTag::new(vec![Box::new(ByteTag::new(1i8)))], TAG_LIST)))*/
            .set_int_array("custom_int_array".to_string(), vec![1, 2, 3, 5, 76, 56, 443]).clone();


        let root = TreeRoot::new(Box::new(compound_tag), "CustomCompoundName".to_string());

        let binding = root.get_tag();
        let compound_tag = binding.as_any().downcast_ref::<CompoundTag>().expect("That's not CompoundTag");
        if root.get_tag().get_type() == TAG_COMPOUND {
            println!("Result: {:?}", compound_tag.get_int_array("custom_int_array").expect("get_int_array() error"));
        }


        /*let mut serializer = BigEndianNBTSerializer::new();
        let dat = serializer.write(root);

        let mut serializer2 = BigEndianNBTSerializer::new();
        let root2 = serializer2.read(dat, &mut 0, 0);*/

        //self::assertTrue($root->equals($root2));
    }
}
