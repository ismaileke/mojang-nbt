use binary_utils::binary::Stream;
use crate::nbt::{NBT, TAG_END};
use crate::tag::tag::Tag;
use crate::tree_root::TreeRoot;

pub trait BaseNBTSerializer {

    fn get_stream(&mut self) -> &mut Stream;

    fn read_short(&mut self) -> i16;

    fn read_signed_short(&mut self) -> i16;

    fn read_int(&mut self) -> i32;

    fn read_long(&mut self) -> i64;

    fn read_float(&mut self) -> f32;

    fn read_double(&mut self) -> f64;

    fn read_int_array(&mut self) -> Vec<i32>;

    fn write_short(&mut self, value: i16);

    fn write_int(&mut self, value: i32);

    fn write_long(&mut self, value: i64);

    fn write_float(&mut self, value: f32);

    fn write_double(&mut self, value: f64);

    fn write_int_array(&mut self, value: Vec<i32>);

    fn read_root(&mut self, _max_depth: u32) -> Box<TreeRoot> where Self: Sized {
        let tag_type =  self.read_byte();

        if tag_type == TAG_END {
            panic!("Found TAG_End at the start of buffer");
        }

        let root_name = self.read_string();

        TreeRoot::new(NBT::create_tag(tag_type, self).expect("Base NBT Serializer Read Root Function"), root_name)
    }

    fn read(&mut self, buffer: Vec<u8>, offset: &mut u32, max_depth: u32) -> Box<TreeRoot> where Self: Sized {
        *self.get_stream() = Stream::new(buffer, *offset);

        let data = self.read_root(max_depth);

        *offset = self.get_stream().get_offset();

        data
    }

    fn read_headless(&mut self, buffer: Vec<u8>, offset: &mut u32, tag_type: u8, _max_depth: u32) -> Box<dyn Tag> where Self: Sized {
        *self.get_stream() = Stream::new(buffer, *offset);

        let data = NBT::create_tag(tag_type, self).expect("Base NBT Serializer Read Headless Function");

        *offset = self.get_stream().get_offset();

        data
    }

    fn read_multiple(&mut self, buffer: Vec<u8>, max_depth: u32) -> Vec<Box<TreeRoot>> where Self: Sized {
        *self.get_stream() = Stream::new(buffer, 0);

        let mut return_value = Vec::new();

        while !self.get_stream().feof() {
            return_value.push(self.read_root(max_depth));
        }

        return_value
    }

    fn write_root(&mut self, root: Box<TreeRoot>) where Self: Sized {
        self.write_byte(root.get_tag().get_type());
        self.write_string(root.get_name());

        root.get_tag().write(self);
    }

    fn write(&mut self, root: Box<TreeRoot>) -> Vec<u8> where Self: Sized {
        *self.get_stream() = Stream::new(vec![], 0);

        self.write_root(root);

        Vec::from(self.get_stream().get_buffer())
    }

    fn write_headless(&mut self, data: Box<dyn Tag>) -> Vec<u8> where Self: Sized {
        *self.get_stream() = Stream::new(vec![], 0);

        data.write(self);

        Vec::from(self.get_stream().get_buffer())
    }

    fn write_multiple(&mut self, tree_root: Vec<Box<TreeRoot>>) -> Vec<u8> where Self: Sized {
        *self.get_stream() = Stream::new(vec![], 0);

        for root in tree_root {
            self.write_root(root);
        }

        Vec::from(self.get_stream().get_buffer())
    }

    fn read_byte(&mut self) -> u8 {
        self.get_stream().get_byte()
    }

    fn read_signed_byte(&mut self) -> i8 { // CHECK AGAIN
        self.get_stream().get_byte() as i8
    }

    fn write_byte(&mut self, value: u8) {
        self.get_stream().put_byte(value);
    }

    fn read_byte_array(&mut self) -> Vec<u8> {
        let len = self.read_int();
        self.get_stream().get(len as u32)
    }

    fn write_byte_array(&mut self, value: Vec<u8>) {
        self.write_int(value.len() as i32); //TODO: overflow
        self.get_stream().put(value);
    }

    fn read_string(&mut self) -> String {
        let len = self.read_short();

        let value = self.get_stream().get(len as u32);

        String::from_utf8(value).expect("Base NBT Serializer, read_string fn, Vec<u8> to String(UTF-8) error")
    }

    fn write_string(&mut self, value: String) {
        self.write_short(value.len() as i16);
        self.get_stream().put(value.into_bytes());
    }
}
