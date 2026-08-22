use binary_utils::binary::{Reader, Writer};
use crate::nbt::NBT;
use crate::tag::tag::Tag;
use crate::tree_root::TreeRoot;

pub enum NBTReader<'a> {
    BigEndian(Reader<'a>),
    LittleEndian(Reader<'a>),
    Network(Reader<'a>)
}

pub enum NBTWriter {
    BigEndian(Writer),
    LittleEndian(Writer),
    Network(Writer)
}

impl<'a> NBTReader<'a> {
    pub fn new_big_endian() -> Self {
        NBTReader::BigEndian(Reader::new(&[]))
    }

    pub fn new_little_endian() -> Self {
        NBTReader::LittleEndian(Reader::new(&[]))
    }

    pub fn new_network() -> Self {
        NBTReader::Network(Reader::new(&[]))
    }

    pub fn get_stream(&mut self) -> &mut Reader<'a> {
        match self {
            NBTReader::BigEndian(s) | NBTReader::LittleEndian(s) | NBTReader::Network(s) => s,
        }
    }

    pub fn read_short(&mut self) -> u16 {
        match self {
            NBTReader::BigEndian(stream) => stream.get_u16_be(),
            NBTReader::LittleEndian(stream) => stream.get_u16_le(),
            NBTReader::Network(stream) => stream.get_u16_le()
        }
    }

    pub fn read_signed_short(&mut self) -> i16 {
        match self {
            NBTReader::BigEndian(stream) => stream.get_i16_be(),
            NBTReader::LittleEndian(stream) => stream.get_i16_le(),
            NBTReader::Network(stream) => stream.get_i16_le(),
        }
    }

    pub fn read_int(&mut self) -> i32 {
        match self {
            NBTReader::BigEndian(stream) => stream.get_i32_be(),
            NBTReader::LittleEndian(stream) => stream.get_i32_le(),
            NBTReader::Network(stream) => stream.get_var_i32()
        }
    }

    pub fn read_long(&mut self) -> i64 {
        match self {
            NBTReader::BigEndian(stream) => stream.get_i64_be(),
            NBTReader::LittleEndian(stream) => stream.get_i64_le(),
            NBTReader::Network(stream) => stream.get_var_i64()
        }
    }

    pub fn read_float(&mut self) -> f32 {
        match self {
            NBTReader::BigEndian(stream) => stream.get_f32_be(),
            NBTReader::LittleEndian(stream) => stream.get_f32_le(),
            NBTReader::Network(stream) => stream.get_f32_le()
        }
    }

    pub fn read_double(&mut self) -> f64 {
        match self {
            NBTReader::BigEndian(stream) => stream.get_f64_be(),
            NBTReader::LittleEndian(stream) => stream.get_f64_le(),
            NBTReader::Network(stream) => stream.get_f64_le()
        }
    }

    pub fn read_int_array(&mut self) -> Vec<i32> {
        let len = self.read_int();

        let bytes = match self {
            NBTReader::BigEndian(r) | NBTReader::LittleEndian(r) | NBTReader::Network(r) => {
                r.get((len * 4) as usize)
            }
        };
        let mut data_stream = Reader::new(bytes);
        let mut int_array = Vec::new();

        while !data_stream.feof() {
            match self {
                NBTReader::BigEndian(_) => int_array.push(data_stream.get_i32_be()),
                NBTReader::LittleEndian(_) => int_array.push(data_stream.get_i32_le()),
                NBTReader::Network(_) => int_array.push(data_stream.get_var_i32())
            }
        }

        int_array
    }

    pub fn read_root(&mut self, _max_depth: u32) -> TreeRoot<'a> {
        let tag_type = self.read_byte();

        if tag_type == NBT::TAG_END {
            panic!("Found TAG_End at the start of buffer");
        }

        let root_name = self.read_string();

        let tag = NBT::create_tag(tag_type, self).expect("NBT read_root");
        TreeRoot::new(tag, root_name)
    }

    pub fn read(&mut self, buffer: &'a [u8], offset: &mut usize, max_depth: u32) -> TreeRoot<'a> {
        let mut reader = Reader::new(buffer);
        reader.set_offset(*offset);
        *self.get_stream() = reader;

        let data = self.read_root(max_depth);

        *offset = self.get_stream().offset();

        data
    }

    pub fn read_headless(&mut self, buffer: &'a [u8], offset: &mut usize, tag_type: u8, _max_depth: u32) -> Tag {
        let mut reader = Reader::new(buffer);
        reader.set_offset(*offset);
        *self.get_stream() = reader;

        let data = NBT::create_tag(tag_type, self).expect("NBT Serializer Read Headless Function");

        *offset = self.get_stream().offset();

        data
    }

    pub fn read_multiple(&mut self, buffer: &'a [u8], max_depth: u32) -> Vec<TreeRoot<'a>> {
        *self.get_stream() = Reader::new(buffer);

        let mut return_value = Vec::new();

        while !self.get_stream().feof() {
            return_value.push(self.read_root(max_depth));
        }

        return_value
    }

    pub fn read_byte(&mut self) -> u8 {
        self.get_stream().get_u8()
    }

    pub fn read_signed_byte(&mut self) -> i8 {
        self.get_stream().get_u8() as i8
    }

    pub fn read_byte_array(&mut self) -> &'a [u8] {
        let len = self.read_int();
        self.get_stream().get(len as usize)
    }

    pub fn read_string(&mut self) -> &'a str {
        match self {
            NBTReader::Network(stream) => {
                let len = stream.get_var_u32();
                let value = stream.get(len as usize);
                str::from_utf8(value).expect("NBT Serializer, read_string fn, Vec<u8> to String(UTF-8) error")
            },
            NBTReader::BigEndian(_stream) |
            NBTReader::LittleEndian(_stream) => {
                let len = self.read_short();
                let value = self.get_stream().get(len as usize);
                str::from_utf8(value).expect("NBT Serializer, read_string fn, Vec<u8> to String(UTF-8) error")
            }
        }
    }
}

impl NBTWriter {
    pub fn new_big_endian() -> Self {
        NBTWriter::BigEndian(Writer::new())
    }

    pub fn new_little_endian() -> Self {
        NBTWriter::LittleEndian(Writer::new())
    }

    pub fn new_network() -> Self {
        NBTWriter::Network(Writer::new())
    }

    pub fn get_stream(&mut self) -> &mut Writer {
        match self {
            NBTWriter::BigEndian(stream) => stream,
            NBTWriter::LittleEndian(stream) => stream,
            NBTWriter::Network(stream) => stream
        }
    }


    pub fn write_short(&mut self, data: i16) {
        match self {
            NBTWriter::BigEndian(stream) => stream.put_i16_be(data),
            NBTWriter::LittleEndian(stream) => stream.put_i16_le(data),
            NBTWriter::Network(stream) => stream.put_i16_le(data)
        }
    }

    pub fn write_int(&mut self, data: i32) {
        match self {
            NBTWriter::BigEndian(stream) => stream.put_i32_be(data),
            NBTWriter::LittleEndian(stream) => stream.put_i32_le(data),
            NBTWriter::Network(stream) => stream.put_var_i32(data)
        }
    }

    pub fn write_long(&mut self, data: i64) {
        match self {
            NBTWriter::BigEndian(stream) => stream.put_i64_be(data),
            NBTWriter::LittleEndian(stream) => stream.put_i64_le(data),
            NBTWriter::Network(stream) => stream.put_var_i64(data)
        }
    }

    pub fn write_float(&mut self, value: f32) {
        match self {
            NBTWriter::BigEndian(stream) => stream.put_f32_be(value),
            NBTWriter::LittleEndian(stream) => stream.put_f32_le(value),
            NBTWriter::Network(stream) => stream.put_f32_le(value)
        }
    }

    pub fn write_double(&mut self, data: f64) {
        match self {
            NBTWriter::BigEndian(stream) => stream.put_f64_be(data),
            NBTWriter::LittleEndian(stream) => stream.put_f64_le(data),
            NBTWriter::Network(stream) => stream.put_f64_le(data)
        }
    }

    pub fn write_int_array(&mut self, data: Vec<i32>) {
        self.write_int(data.len() as i32);

        for &value in &data {
            match self {
                NBTWriter::BigEndian(stream) => stream.put_i32_be(value),
                NBTWriter::LittleEndian(stream) => stream.put_i32_le(value),
                NBTWriter::Network(_stream) => self.write_int(value)
            }
        }
    }

    pub fn write_root(&mut self, root: TreeRoot) {
        self.write_byte(root.get_tag().get_id());
        self.write_string(root.get_name());

        root.get_tag().write(self);
    }

    pub fn write(&mut self, root: TreeRoot) -> &[u8] {
        *self.get_stream() = Writer::new();

        self.write_root(root);

        self.get_stream().as_slice()
    }

    pub fn write_headless(&mut self, data: Tag) -> &[u8] {
        *self.get_stream() = Writer::new();

        data.write(self);

        self.get_stream().as_slice()
    }

    pub fn write_multiple(&mut self, tree_root: Vec<TreeRoot>) -> &[u8] {
        *self.get_stream() = Writer::new();

        for root in tree_root {
            self.write_root(root);
        }

        self.get_stream().as_slice()
    }

    pub fn write_byte(&mut self, value: u8) {
        self.get_stream().put_u8(value);
    }

    pub fn write_byte_array(&mut self, value: &[u8]) {
        self.write_int(value.len() as i32); // TODO: overflow
        self.get_stream().put(value);
    }

    pub fn write_string(&mut self, value: &str) {
        match self {
            NBTWriter::Network(stream) => {
                stream.put_var_u32(value.len() as u32);
                stream.put(value.as_bytes());
            },
            NBTWriter::BigEndian(_stream) |
            NBTWriter::LittleEndian(_stream) => {
                self.write_short(value.len() as i16);
                self.get_stream().put(value.as_bytes());
            }
        }
    }
}