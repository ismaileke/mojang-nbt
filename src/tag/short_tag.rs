use crate::nbt_serializer::{NBTReader, NBTWriter};

#[derive(Clone, Debug, serde::Serialize)]
pub struct ShortTag {
    value: i16
}

impl ShortTag {

    pub fn new(value: i16) -> Self {
        ShortTag{ value }
    }

    pub fn get_value(&self) -> i16 {
        self.value
    }

    pub fn read(serializer: &mut NBTReader) -> ShortTag {
        let signed_short = serializer.read_signed_short();
        ShortTag { value: signed_short }
    }

    pub fn write(&self, serializer: &mut NBTWriter) {
        serializer.write_short(self.get_value());
    }
}
