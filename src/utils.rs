use artnet_protocol::{ArtCommand, PollReply, ARTNET_PROTOCOL_VERSION};
use std::convert::AsMut;
use std::net::{SocketAddr, UdpSocket, Ipv4Addr};
use anyhow::Error;

// Helper function to clone a slice into an array reference
pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

pub fn send_artnet_command(command: ArtCommand, socket: &UdpSocket, dest: &SocketAddr) -> Result<(), Error> {
    let bytes = command.into_buffer()?;
    socket.send_to(&bytes, dest)?;
    Ok(())
}

// Fake an implementation of the Default trait for PollReply
// Implementing Default would violate the orphan rules for trait implementaitons
pub fn default_poll_reply() -> PollReply {
    PollReply {
        address: Ipv4Addr::new(127, 0, 0, 1),
        port: 6454,
        version: ARTNET_PROTOCOL_VERSION,
        port_address: [0, 0],
        oem: [0, 0],
        ubea_version: 0,
        status_1: 0,
        esta_code: 0,
        short_name: [0; 18],
        long_name: [0; 64],
        node_report: [0; 64],
        num_ports: [0; 2],
        port_types: [0; 4],
        good_input: [0; 4],
        good_output: [0; 4],
        swin: [0; 4],
        swout: [0; 4],
        sw_video: 0,
        sw_macro: 0,
        sw_remote: 0,
        spare: [0; 3],
        style: 0,
        mac: [0; 6],
        bind_ip: [0; 4],
        bind_index: 0,
        status_2: 0,
        filler: [0; 26],
    }
}
