use crate::base_nbt_serializer::BaseNBTSerializer;
use binary_utils::binary::Stream;

pub struct LittleEndianNBTSerializer {
    binary_stream: Stream
}

impl LittleEndianNBTSerializer {
    pub fn new() -> LittleEndianNBTSerializer {
        LittleEndianNBTSerializer{ binary_stream: Stream::new(vec![], 0) }
    }
}

impl BaseNBTSerializer for LittleEndianNBTSerializer {

    fn get_stream(&mut self) -> &mut Stream {
        &mut self.binary_stream
    }

    fn read_short(&mut self) -> u16 {
        self.binary_stream.get_u16_le()
    }

    fn read_signed_short(&mut self) -> i16 {
        self.binary_stream.get_i16_le()
    }
    fn read_int(&mut self) -> u32 {
        self.binary_stream.get_u32_le()
    }

    fn read_long(&mut self) -> i64 {
        self.binary_stream.get_i64_le()
    }

    fn read_float(&mut self) -> f32 {
        self.binary_stream.get_f32_le()
    }

    fn read_double(&mut self) -> f64 {
        self.binary_stream.get_f64_le()
    }

    fn read_int_array(&mut self) -> Vec<u32> {
        let len = self.read_int();

        let mut int_array = Vec::new();

        let mut data_stream = Stream::new(self.binary_stream.get(len * 4), 0);

        while !data_stream.feof() {
            // big endian 4 byte integer
            int_array.push(data_stream.get_u32_le());
        }

        int_array
    }

    fn write_short(&mut self, data: u16) {
        self.binary_stream.put_u16_le(data)
    }

    fn write_int(&mut self, data: u32) {
        self.binary_stream.put_u32_le(data)
    }

    fn write_long(&mut self, data: i64) {
        self.binary_stream.put_i64_le(data)
    }

    fn write_float(&mut self, value: f32) {
        self.binary_stream.put_f32_le(value)
    }

    fn write_double(&mut self, data: f64) {
        self.binary_stream.put_f64_le(data)
    }

    fn write_int_array(&mut self, data: Vec<u32>) {
        self.write_int(data.len() as u32);

        for index in 0..data.len() {
            self.binary_stream.put_u32_le(data[index]);
        }
    }
}
