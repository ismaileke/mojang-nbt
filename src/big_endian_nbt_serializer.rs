use std::error::Error;
use binary_utils::binary::Stream;
use crate::base_nbt_serializer::BaseNBTSerializer;

pub struct BigEndianNBTSerializer {
    binary_stream: Stream
}

impl BigEndianNBTSerializer {
    pub fn new() -> BigEndianNBTSerializer {
        BigEndianNBTSerializer{ binary_stream: Stream::new(vec![], 0) }
    }
}

impl BaseNBTSerializer for BigEndianNBTSerializer {

    fn get_stream(&mut self) -> &mut Stream {
        &mut self.binary_stream
    }
    fn read_short(&mut self) -> u16 {
        self.binary_stream.get_short()
    }

    fn read_signed_short(&mut self) -> i16 {
        self.binary_stream.get_signed_short()
    }
    fn read_int(&mut self) -> u32 {
        self.binary_stream.get_int()
    }

    fn read_long(&mut self) -> i64 {
        self.binary_stream.get_long()
    }

    fn read_float(&mut self) -> f32 {
        self.binary_stream.get_float()
    }

    fn read_double(&mut self) -> f64 {
        self.binary_stream.get_double()
    }

    fn read_int_array(&mut self) -> Result<Vec<u32>, dyn Error> {
        let len = self.binary_stream.get_int();
        if len < 0 {
            return Err(format!("Array length cannot be less than zero ({} < 0)", len));
        }

        let mut int_array = Vec::new();

        let mut data_stream = Stream::new(self.binary_stream.get(len * 4).unwrap(), 0);

        while !data_stream.feof() {
            // big endian 4 byte integer
            int_array.push(data_stream.get_int());
        }

        Ok(int_array)
    }

    fn write_short(&mut self, data: u16) {
        self.binary_stream.put_short(data);
    }

    fn write_int(&mut self, data: u32) {
        self.binary_stream.put_int(data);
    }

    fn write_long(&mut self, data: i64) {
        self.binary_stream.put_long(data);
    }

    fn write_float(&mut self, value: f32) {
        self.binary_stream.put_float(value)
    }

    fn write_double(&mut self, data: f64) {
        self.binary_stream.put_double(data);
    }

    fn write_int_array(&mut self, data: Vec<u32>) {
        self.binary_stream.put_int(data.len() as u32);

        for index in 0..data.len() {
            self.binary_stream.put_int(data[index]);
        }
    }
}