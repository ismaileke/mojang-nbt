use binary_utils::binary::Stream;
use crate::nbt::NBT;
use crate::tag::tag::Tag;
use crate::tree_root::TreeRoot;

pub enum NBTSerializer {
    BigEndian(Stream),
    LittleEndian(Stream),
}

impl NBTSerializer {
    pub fn new_big_endian() -> Self {
        NBTSerializer::BigEndian(Stream::new(vec![], 0))
    }

    pub fn new_little_endian() -> Self {
        NBTSerializer::LittleEndian(Stream::new(vec![], 0))
    }

    pub fn get_stream(&mut self) -> &mut Stream {
        match self {
            NBTSerializer::BigEndian(stream) => stream,
            NBTSerializer::LittleEndian(stream) => stream,
        }
    }

    pub fn read_short(&mut self) -> i16 {
        match self {
            NBTSerializer::BigEndian(stream) => stream.get_i16_be(),
            NBTSerializer::LittleEndian(stream) => stream.get_i16_le(),
        }
    }

    pub fn read_signed_short(&mut self) -> i16 {
        match self {
            NBTSerializer::BigEndian(stream) => stream.get_i16_be(),
            NBTSerializer::LittleEndian(stream) => stream.get_i16_le(),
        }
    }

    pub fn read_int(&mut self) -> i32 {
        match self {
            NBTSerializer::BigEndian(stream) => stream.get_i32_be(),
            NBTSerializer::LittleEndian(stream) => stream.get_i32_le(),
        }
    }

    pub fn read_long(&mut self) -> i64 {
        match self {
            NBTSerializer::BigEndian(stream) => stream.get_i64_be(),
            NBTSerializer::LittleEndian(stream) => stream.get_i64_le(),
        }
    }

    pub fn read_float(&mut self) -> f32 {
        match self {
            NBTSerializer::BigEndian(stream) => stream.get_f32_be(),
            NBTSerializer::LittleEndian(stream) => stream.get_f32_le(),
        }
    }

    pub fn read_double(&mut self) -> f64 {
        match self {
            NBTSerializer::BigEndian(stream) => stream.get_f64_be(),
            NBTSerializer::LittleEndian(stream) => stream.get_f64_le(),
        }
    }

    pub fn read_int_array(&mut self) -> Vec<i32> {
        let len = self.read_int();
        let mut int_array = Vec::new();

        let bytes = self.get_stream().get((len * 4) as u32);
        let mut data_stream = Stream::new(bytes, 0);

        while !data_stream.feof() {
            match self {
                NBTSerializer::BigEndian(_) => int_array.push(data_stream.get_i32_be()),
                NBTSerializer::LittleEndian(_) => int_array.push(data_stream.get_i32_le())
            }
        }

        int_array
    }

    pub fn write_short(&mut self, data: i16) {
        match self {
            NBTSerializer::BigEndian(stream) => stream.put_i16_be(data),
            NBTSerializer::LittleEndian(stream) => stream.put_i16_le(data),
        }
    }

    pub fn write_int(&mut self, data: i32) {
        match self {
            NBTSerializer::BigEndian(stream) => stream.put_i32_be(data),
            NBTSerializer::LittleEndian(stream) => stream.put_i32_le(data),
        }
    }

    pub fn write_long(&mut self, data: i64) {
        match self {
            NBTSerializer::BigEndian(stream) => stream.put_i64_be(data),
            NBTSerializer::LittleEndian(stream) => stream.put_i64_le(data),
        }
    }

    pub fn write_float(&mut self, value: f32) {
        match self {
            NBTSerializer::BigEndian(stream) => stream.put_f32_be(value),
            NBTSerializer::LittleEndian(stream) => stream.put_f32_le(value),
        }
    }

    pub fn write_double(&mut self, data: f64) {
        match self {
            NBTSerializer::BigEndian(stream) => stream.put_f64_be(data),
            NBTSerializer::LittleEndian(stream) => stream.put_f64_le(data),
        }
    }

    pub fn write_int_array(&mut self, data: Vec<i32>) {
        self.write_int(data.len() as i32);

        for &value in &data {
            match self {
                NBTSerializer::BigEndian(stream) => stream.put_i32_be(value),
                NBTSerializer::LittleEndian(stream) => stream.put_i32_le(value)
            }
        }
    }

    pub fn read_root(&mut self, _max_depth: u32) -> TreeRoot {
        let tag_type = self.read_byte();

        if tag_type == NBT::TAG_END {
            panic!("Found TAG_End at the start of buffer");
        }

        let root_name = self.read_string();

        TreeRoot::new(
            NBT::create_tag(tag_type, self).expect("NBT Serializer Read Root Function"),
            root_name,
        )
    }

    pub fn read(&mut self, buffer: Vec<u8>, offset: &mut u32, max_depth: u32) -> TreeRoot {
        *self.get_stream() = Stream::new(buffer, *offset);

        let data = self.read_root(max_depth);

        *offset = self.get_stream().get_offset();

        data
    }

    pub fn read_headless(&mut self, buffer: Vec<u8>, offset: &mut u32, tag_type: u8, _max_depth: u32) -> Tag {
        *self.get_stream() = Stream::new(buffer, *offset);

        let data = NBT::create_tag(tag_type, self).expect("NBT Serializer Read Headless Function");

        *offset = self.get_stream().get_offset();

        data
    }

    pub fn read_multiple(&mut self, buffer: Vec<u8>, max_depth: u32) -> Vec<TreeRoot> {
        *self.get_stream() = Stream::new(buffer, 0);

        let mut return_value = Vec::new();

        while !self.get_stream().feof() {
            return_value.push(self.read_root(max_depth));
        }

        return_value
    }

    pub fn write_root(&mut self, root: TreeRoot) {
        self.write_byte(root.get_tag().get_id());
        self.write_string(root.get_name());

        root.get_tag().write(self);
    }

    pub fn write(&mut self, root: TreeRoot) -> Vec<u8> {
        *self.get_stream() = Stream::new(vec![], 0);

        self.write_root(root);

        Vec::from(self.get_stream().get_buffer())
    }

    pub fn write_headless(&mut self, data: Tag) -> Vec<u8> {
        *self.get_stream() = Stream::new(vec![], 0);

        data.write(self);

        Vec::from(self.get_stream().get_buffer())
    }

    pub fn write_multiple(&mut self, tree_root: Vec<TreeRoot>) -> Vec<u8> {
        *self.get_stream() = Stream::new(vec![], 0);

        for root in tree_root {
            self.write_root(root);
        }

        Vec::from(self.get_stream().get_buffer())
    }

    pub fn read_byte(&mut self) -> u8 {
        self.get_stream().get_byte()
    }

    pub fn read_signed_byte(&mut self) -> i8 {
        self.get_stream().get_byte() as i8
    }

    pub fn write_byte(&mut self, value: u8) {
        self.get_stream().put_byte(value);
    }

    pub fn read_byte_array(&mut self) -> Vec<u8> {
        let len = self.read_int();
        self.get_stream().get(len as u32)
    }

    pub fn write_byte_array(&mut self, value: Vec<u8>) {
        self.write_int(value.len() as i32); // TODO: overflow
        self.get_stream().put(value);
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_short();

        let value = self.get_stream().get(len as u32);

        String::from_utf8(value).expect("NBT Serializer, read_string fn, Vec<u8> to String(UTF-8) error")
    }

    pub fn write_string(&mut self, value: String) {
        self.write_short(value.len() as i16);
        self.get_stream().put(value.into_bytes());
    }
}