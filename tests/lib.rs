#[cfg(test)]

mod tests {
    extern crate mojang_nbt;

    use mojang_nbt::tag::compound_tag::CompoundTag;
    use mojang_nbt::tree_root::TreeRoot;
    use std::collections::HashMap;

    #[test]
    fn test() {
        /*let compound_tag = CompoundTag::new(HashMap::new());
        let mut serializer = BigEndianNBTSerializer::new();
        let tag = NBT::create_tag(TAG_COMPOUND, &mut serializer).unwrap();
        println!("DATA: {}", tag.get_type());*/



        let compound_tag = CompoundTag::new(HashMap::new())
            .set_byte("byte".to_string(), 1i8)
            .set_short("short".to_string(), 1i16)
            .set_int("int".to_string(), 1u32)
            .set_long("long".to_string(), 1i64)
            .set_float("float".to_string(), 1f32)
            .set_double("double".to_string(), 1f64)
            .set_byte_array("bytearray".to_string(), vec![1u8])
            .set_string("hello".to_string(), "string".to_string())
            /*.set_tag("list".to_string(), ListTag::new(vec![Box::new(ByteTag::new(1i8)))]))*/
            .set_int_array("intarray".to_string(), vec![1]).clone();


        let root = TreeRoot::new(Box::new(compound_tag), "COMPOUND NAME".to_string());

        println!("DATA: {}", root.get_tag().get_value().downcast_ref::<CompoundTag>().unwrap().get_byte("byte").unwrap());




        /*let mut serializer = BigEndianNBTSerializer::new();
        let dat = serializer.write(root);

        let mut serializer2 = BigEndianNBTSerializer::new();
        let root2 = serializer2.read(dat, &mut 0, 0);*/

        //self::assertTrue($root->equals($root2));
    }
}
