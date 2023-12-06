use crate::net::network_message::NetworkMessage;

#[test]
fn network_message_write_u8() {

    let mut msg = NetworkMessage::default();
    msg.write_u8(1);
    msg.write_u8(2);
    msg.write_u8(3);
    msg.reset();

    assert_eq!(msg.read_u8().unwrap(), 1);
    assert_eq!(msg.read_u8().unwrap(), 2);
    assert_eq!(msg.read_u8().unwrap(), 3);
}

#[test]
fn network_message_write_u16() {

    let mut msg = NetworkMessage::default();
    msg.write_u16(1);
    msg.write_u16(2);
    msg.write_u16(3);
    msg.reset();

    assert_eq!(msg.read_u16().unwrap(), 1);
    assert_eq!(msg.read_u16().unwrap(), 2);
    assert_eq!(msg.read_u16().unwrap(), 3);
}

#[test]
fn network_message_write_u32() {

    let mut msg = NetworkMessage::default();
    msg.write_u32(1);
    msg.write_u32(2);
    msg.write_u32(3);
    msg.reset();

    assert_eq!(msg.read_u32().unwrap(), 1);
    assert_eq!(msg.read_u32().unwrap(), 2);
    assert_eq!(msg.read_u32().unwrap(), 3);
}

#[test]
fn network_message_write_u64() {

    let mut msg = NetworkMessage::default();
    msg.write_u64(1);
    msg.write_u64(2);
    msg.write_u64(3);
    msg.reset();

    assert_eq!(msg.read_u64().unwrap(), 1);
    assert_eq!(msg.read_u64().unwrap(), 2);
    assert_eq!(msg.read_u64().unwrap(), 3);
}

#[test]
fn network_message_write_i8() {

    let mut msg = NetworkMessage::default();
    msg.write_i8(1);
    msg.write_i8(2);
    msg.write_i8(3);
    msg.reset();

    assert_eq!(msg.read_i8().unwrap(), 1);
    assert_eq!(msg.read_i8().unwrap(), 2);
    assert_eq!(msg.read_i8().unwrap(), 3);
}

#[test]
fn network_message_write_i16() {

    let mut msg = NetworkMessage::default();
    msg.write_i16(1);
    msg.write_i16(2);
    msg.write_i16(3);
    msg.reset();

    assert_eq!(msg.read_i16().unwrap(), 1);
    assert_eq!(msg.read_i16().unwrap(), 2);
    assert_eq!(msg.read_i16().unwrap(), 3);
}

#[test]
fn network_message_write_i32() {

    let mut msg = NetworkMessage::default();
    msg.write_i32(1);
    msg.write_i32(2);
    msg.write_i32(3);
    msg.reset();

    assert_eq!(msg.read_i32().unwrap(), 1);
    assert_eq!(msg.read_i32().unwrap(), 2);
    assert_eq!(msg.read_i32().unwrap(), 3);
}

#[test]
fn network_message_write_i64() {

    let mut msg = NetworkMessage::default();
    msg.write_i64(1);
    msg.write_i64(2);
    msg.write_i64(3);
    msg.reset();

    assert_eq!(msg.read_i64().unwrap(), 1);
    assert_eq!(msg.read_i64().unwrap(), 2);
    assert_eq!(msg.read_i64().unwrap(), 3);
}
