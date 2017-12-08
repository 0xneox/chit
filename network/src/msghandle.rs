use protocol::ChitRequest;
use server::MySender;
use byteorder::{BigEndian, ByteOrder};
use std::io;

pub fn net_msg_handler(mut payload: ChitRequest, mysender: &MySender) -> Result<Vec<u8>, io::Error> {
    if payload.len() > 4 {
        let msg = payload.split_off(4);
        let origin = BigEndian::read_u32(payload.as_ref());
        mysender.send((origin, msg));
    }
    Ok(vec![])
}

