use super::{
    BulkString, RespArray, RespEncode, RespMap, RespNull, RespNullArray, RespNullBulkString,
    RespSet, SimpleError, SimpleString,
};

// +OK\r\n
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", *self).into_bytes()
    }
}

// -Error message\r\n
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-Error {}\r\n", *self).into_bytes()
    }
}

// :[<+|->]<value>\r\n
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

// $<length>\r\n<data>\r\n
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::with_capacity(length + 5);
        buf.extend_from_slice(&format!("${}\r\n", length).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// $-1\r\n
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// *<number-of-elements>\r\n<element-1>...<element-n>
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

// *-1\r\n
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// _\r\n
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

// #<t|f>\r\n
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { 't' } else { 'f' }).into_bytes()
    }
}

// ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        let ret = if self.abs() > 1e+8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };
        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

// %<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&format!("%{}\r\n", self.0.len()).into_bytes());
        for (key, value) in self.0 {
            buf.extend_from_slice(&SimpleString::new(&key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

// ~<number-of-elements>\r\n<element-1>...<element-n>
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&format!("~{}\r\n", self.0.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

#[cfg(test)]
mod tests {

    use crate::resp::RespFrame;

    use super::*;

    #[test]
    fn test_simple_string_encode() {
        let frame = SimpleString::new("OK");
        assert_eq!(frame.encode(), b"+OK\r\n");
    }

    #[test]
    fn test_simple_error_encode() {
        let frame = SimpleError("Error message".to_string());
        assert_eq!(frame.encode(), b"-Error Error message\r\n");
    }

    #[test]
    fn test_integer_encode() {
        let frame: RespFrame = 123.into();
        assert_eq!(frame.encode(), b":+123\r\n");

        let frame = -123;
        assert_eq!(frame.encode(), b":-123\r\n");
    }

    #[test]
    fn test_bulk_string_encode() {
        let frame = BulkString(b"Hello, World!".to_vec());
        assert_eq!(frame.encode(), b"$13\r\nHello, World!\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame = RespNullBulkString;
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_array_encode() {
        let frame = RespArray(vec![
            RespFrame::SimpleString(SimpleString::new("OK")),
            RespFrame::Integer(123),
        ]);
        assert_eq!(frame.encode(), b"*2\r\n+OK\r\n:+123\r\n");
    }

    #[test]
    fn test_null_array_encode() {
        let frame = RespNullArray;
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_encode() {
        let frame = RespNull;
        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_boolean_encode() {
        let frame = true;
        assert_eq!(frame.encode(), b"#t\r\n");
    }

    #[test]
    fn test_double_encode() {
        let frame = 123.456;
        assert_eq!(frame.encode(), b",+123.456\r\n");
    }
}
