use serde::{Serialize, Serializer};
use serde::ser::SerializeTuple;
use anyhow::Error;

extern crate bincode;

#[derive(Serialize)]
pub struct Output {
    pub magic: i32,
    pub version: u16,
    pub command: u16,
    pub sequence: i32,
    pub port: u8,
    pub padding: u8,
    pub flags: u16,
    pub timer: i32,
    pub universe: u8,
    #[serde(serialize_with = "serialize_array512")]
    pub data: [u8; 512],
}

fn serialize_array512<S, T>(array: &[T; 512], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut seq = serializer.serialize_tuple(512)?;
    for element in array.iter() {
        seq.serialize_element(element)?;
    }
    return seq.end();
}

/*
pub enum KinetCommand {   
    DiscoverSupplies, // 0x100
    Output, // 0x101
    DiscoverFixturesSerialRequest, // 0x102
    DiscoverFixturesChannelRequest, // 0x302
    OutputPDS480, // 0x801
    DiscoverSuppliesReply, // ???
    DiscoverFixturesSerialReply, // ???
    DiscoverFixturesChannelReply, // ???
}
*/

impl Default for Output {
    fn default() -> Output {
        Output {
            magic: 0x0401dc4a,
            version: 0x0100,
            command: 0x0101,
            sequence: 0x00000000,
            port: 0,
            padding: 0,
            flags: 0x0000,
            timer: -1,
            universe: 0,
            data: [0; 512],
        }
    }
}

impl Output {
    pub fn serialize(self) -> Result<Vec<u8>, Error> {
        return Ok(bincode::config().big_endian().serialize(&self)?);
    }
}