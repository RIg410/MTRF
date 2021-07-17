use std::convert::TryFrom;
use std::fmt;

use anyhow::Error;

use crate::cmd::{Cmd, CtrResponse, Mode, CH_INDEX, CRC_INDEX, MESSAGE_LENGTH};

#[derive(Debug, Copy, Clone)]
pub struct Response {
    pub mode: Mode,
    pub ctr: CtrResponse,
    pub togl: u8,
    pub ch: u8,
    pub cmd: Cmd,
    pub id: u32,
    pub crc: u8,
}

impl TryFrom<[u8; MESSAGE_LENGTH]> for Response {
    type Error = Error;

    fn try_from(value: [u8; 17]) -> Result<Self, Self::Error> {
        let sum: u32 = value.iter().take(15).map(|b| *b as u32).sum();

        if sum.to_le_bytes()[0] != value[CRC_INDEX] {
            return Err(anyhow!("Invalid message crc:[{:?}]", value));
        }

        let mut id = [0; 4];
        id[0] = value[11];
        id[1] = value[12];
        id[2] = value[13];
        id[3] = value[14];

        Ok(Response {
            mode: Mode::try_from(value[1])?,
            ctr: CtrResponse::try_from(value[2])?,
            togl: value[3],
            ch: value[CH_INDEX],
            cmd: Cmd::try_from(&value[5..])?,
            id: u32::from_le_bytes(id),
            crc: value[CRC_INDEX],
        })
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ST:173 MODE:{} CTR:{} TOGL:{} CH:{} ",
            self.mode, self.ctr, self.togl, self.ch
        )?;
        write!(f, "CMD:{} ", self.cmd)?;
        write!(f, "ID:{} ", self.id)?;
        write!(f, "CRC:{} ", self.crc)?;
        write!(f, "SP:174")
    }
}
