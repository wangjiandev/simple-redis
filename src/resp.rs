use std::collections::{HashMap, HashSet};

pub mod decode;
pub mod encode;

// 解析redis的frame
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString(RespNullBulkString),
    Array(Vec<RespFrame>),
    NullArray(RespNullArray),
    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>),
}

pub struct SimpleString();

pub struct SimpleError();

pub struct RespNull;

pub struct RespNullArray;

pub struct RespNullBulkString;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

// impl RespDecode for BytesMut {
//     fn decode(buf: Self) -> Result<RespFrame, String> {
//         todo!()
//     }
// }
