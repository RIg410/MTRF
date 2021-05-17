use crate::cmd::MESSAGE_LENGTH;

#[derive(Debug, Copy, Clone)]
pub struct Response([u8; MESSAGE_LENGTH]);

impl From<[u8; MESSAGE_LENGTH]> for Response {
    fn from(buffer: [u8; MESSAGE_LENGTH]) -> Self {
        Response(buffer)
    }
}