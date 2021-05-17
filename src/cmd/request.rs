use std::fmt;

use anyhow::Error;

use crate::cmd::{Cmd, CtrRequest, MESSAGE_LENGTH, Mode, SetBrightness, TemporaryOn};

const ST: u8 = 171;
const SP: u8 = 172;
const RES: u8 = 0;

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub struct Request([u8; MESSAGE_LENGTH]);

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd = &self.0;
        write!(f, "ST:{} MODE:{} CTR:{} RES:{} CH:{} CMD:{} FMT:{} D0:{} D1:{} D2:{} D3:{} ID0:{} ID1:{} ID2:{} ID3:{} CRC:{} SP:{}",
               cmd[0], cmd[1], cmd[2], cmd[3], cmd[4], cmd[5], cmd[6], cmd[7], cmd[8], cmd[9], cmd[10], cmd[11], cmd[12], cmd[13], cmd[14], cmd[15], cmd[16],
        )
    }
}

impl From<[u8; MESSAGE_LENGTH]> for Request {
    fn from(buffer: [u8; MESSAGE_LENGTH]) -> Self {
        Request(buffer)
    }
}

impl AsRef<[u8]> for Request {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RequestBuilder {
    pub mode: Option<Mode>,
    pub ctr: Option<CtrRequest>,
    ch: u8,
    pub cmd: Option<Cmd>,
    id: u32,
}

impl RequestBuilder {
    pub fn ch(&self) -> u8 {
        self.ch
    }

    pub fn set_ch(&mut self, ch: u8) -> Result<(), Error> {
        ensure!(ch < 64, "The ch value must be between 0 and 63");
        self.ch = ch;
        Ok(())
    }

    pub fn to_message(self) -> Request {
        let mut msg = [0; MESSAGE_LENGTH];
        msg[0] = ST;

        if let Some(mode) = self.mode {
            msg[1] = mode as u8;
        } else {
            msg[1] = 0;
        }

        if let Some(ctr) = self.ctr {
            msg[2] = ctr as u8;
        } else {
            msg[2] = 0;
        }

        msg[3] = RES;
        msg[4] = self.ch;

        if let Some(cmd) = self.cmd {
            msg[5] = cmd.as_u8();
            match cmd {
                Cmd::SetBrightness(br) => {
                    match br {
                        SetBrightness::Fmt1(d0) => {
                            msg[6] = 1;
                            msg[7] = d0;
                        }
                        SetBrightness::Fmt3(d) => {
                            msg[6] = 3;
                            msg[7] = d[0];
                            msg[8] = d[1];
                            msg[9] = d[2];
                        }
                    }
                }
                Cmd::BrightReg(reg) => {
                    msg[6] = 1;
                    msg[7] = reg;
                }
                Cmd::TemporaryOn(tem) => {
                    match tem {
                        TemporaryOn::Fmt1(d0) => {
                            msg[6] = 1;
                            msg[7] = d0;
                        }
                        TemporaryOn::Fmt2(d) => {
                            msg[6] = 2;
                            msg[7] = d[0];
                            msg[8] = d[1];
                        }
                    }
                }
                Cmd::SERVICE(serv) => {
                    msg[7] = if serv { 1 } else { 0 };
                }
                Cmd::ClearMemory => {
                    msg[6] = 4;
                    msg[7] = 170;
                    msg[8] = 85;
                    msg[9] = 170;
                    msg[10] = 85;
                }
                Cmd::Off |
                Cmd::BrightDown |
                Cmd::On |
                Cmd::BrightUp |
                Cmd::Switch |
                Cmd::BrightBack |
                Cmd::LoadPreset |
                Cmd::SavePreset |
                Cmd::Unbind |
                Cmd::StopBright |
                Cmd::BrightStepDown |
                Cmd::BrightStepUp |
                Cmd::Bind |
                Cmd::RollColor |
                Cmd::SwitchColor |
                Cmd::SwitchMode |
                Cmd::SpeedMode |
                Cmd::BatteryLow |
                Cmd::SensTempHumi |
                Cmd::Modes |
                Cmd::ReadState |
                Cmd::WriteState |
                Cmd::SendState => {
                    // default parameters
                }
            }
        } else {
            msg[5] = 0;
        }

        let mut idx = 11;
        for b in self.id.to_le_bytes().iter() {
            msg[idx] = *b;
            idx += 1;
        }

        let mut sum: u32 = 0;
        for byte in msg.iter().take(15) {
            sum += *byte as u32;
        }

        msg[15] = sum.to_le_bytes()[0];
        msg[16] = SP;

        Request(msg)
    }
}

pub fn set_mod(md: Mode) -> Request {
    let mut req = RequestBuilder::default();
    req.mode = Some(md);
    req.to_message()
}

#[cfg(test)]
mod test {
    use crate::cmd::request::{RequestBuilder, Request};
    use crate::cmd::*;

    #[test]
    pub fn test_crc() {
        let mut req = RequestBuilder::default();

        req.mode = Some(Mode::TxF);
        req.set_ch(5).unwrap();
        req.cmd = Some(Cmd::Bind);

        assert_eq!(
            Request([171, 2, 0, 0, 5, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 172]),
            req.to_message()
        );

        req.cmd = Some(Cmd::SERVICE(true));
        assert_eq!(
            Request([171, 2, 0, 0, 5, 131, 0, 1, 0, 0, 0, 0, 0, 0, 0, 54, 172]),
            req.to_message()
        );



    }
}