use super::RespDecode;

impl RespDecode for i64 {
    fn decode(_buf: Self) -> Result<super::RespFrame, String> {
        todo!()
    }
}
