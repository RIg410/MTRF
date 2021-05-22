use std::convert::TryFrom;
use std::fmt;

use anyhow::Error;

pub mod request;
pub mod response;

pub const CH_INDEX: usize = 4;
pub const CMD_INDEX: usize = 5;

pub const CRC_INDEX: usize = 15;

pub const MESSAGE_LENGTH: usize = 17;

pub const REQUEST_ST: u8 = 171;
pub const RESPONSE_ST: u8 = 173;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    TX = 0,
    RX = 1,
    TxF = 2,
    RxF = 3,
    Service = 4,
    FirmwareUpdate = 5,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::TX => write!(f, "Tx"),
            Mode::RX => write!(f, "Rx"),
            Mode::TxF => write!(f, "TxF"),
            Mode::RxF => write!(f, "RxF"),
            Mode::Service => write!(f, "Service"),
            Mode::FirmwareUpdate => write!(f, "FirmwareUpdate"),
        }
    }
}

impl TryFrom<u8> for Mode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Mode::TX,
            1 => Mode::TX,
            2 => Mode::TX,
            3 => Mode::TX,
            4 => Mode::TX,
            5 => Mode::TX,
            _ => return Err(anyhow!("Failed to decode mode:{}", value))
        })
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::TX
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CtrRequest {
    SendCommand = 0,
    SendBroadcastCommand = 1,
    ReadResponse = 2,
    BindModeOn = 3,
    BindModeOff = 4,
    ClearChannel = 5,
    ClearMemory = 6,
    UnbindAddressFromChannel = 7,
    SendCommandToIdInChannel = 8,
    SendCommandToId = 9,
}

impl fmt::Display for CtrRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CtrRequest::SendCommand => write!(f, "SendCommand"),
            CtrRequest::SendBroadcastCommand => write!(f, "SendBroadcastCommand"),
            CtrRequest::ReadResponse => write!(f, "ReadResponse"),
            CtrRequest::BindModeOn => write!(f, "BindModeOn"),
            CtrRequest::BindModeOff => write!(f, "BindModeOff"),
            CtrRequest::ClearChannel => write!(f, "ClearChannel"),
            CtrRequest::ClearMemory => write!(f, "ClearMemory"),
            CtrRequest::UnbindAddressFromChannel => write!(f, "UnbindAddressFromChannel"),
            CtrRequest::SendCommandToIdInChannel => write!(f, "SendCommandToIdInChannel"),
            CtrRequest::SendCommandToId => write!(f, "SendCommandToId"),
        }
    }
}

impl Default for CtrRequest {
    fn default() -> Self {
        CtrRequest::SendCommand
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CtrResponse {
    Success = 0,
    NoResponse = 1,
    Error = 2,
    BindSuccess = 3,
}

impl fmt::Display for CtrResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CtrResponse::Success => write!(f, "Success"),
            CtrResponse::NoResponse => write!(f, "NoResponse"),
            CtrResponse::Error => write!(f, "Error"),
            CtrResponse::BindSuccess => write!(f, "BindSuccess"),
        }
    }
}

impl TryFrom<u8> for CtrResponse {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        Ok(match value {
            0 => CtrResponse::Success,
            1 => CtrResponse::NoResponse,
            2 => CtrResponse::Error,
            3 => CtrResponse::BindSuccess,
            _ => return Err(anyhow!("Failed to decode ctr response:{}", value)),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Cmd {
    Off,
    BrightDown,
    On,
    BrightUp,
    Switch,
    BrightBack,
    SetBrightness(SetBrightness),
    LoadPreset,
    SavePreset,
    Unbind,
    StopBright,
    BrightStepDown,
    BrightStepUp,
    BrightReg(u8),
    Bind,
    RollColor,
    SwitchColor,
    SwitchMode,
    SpeedMode,
    BatteryLow,
    SensTempHumi,
    TemporaryOn(TemporaryOn),
    Modes,
    ReadState,
    WriteState,
    SendState,
    Service(bool),
    ClearMemory,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cmd::Off => write!(f, "off"),
            Cmd::BrightDown => write!(f, "BrightDown"),
            Cmd::On => write!(f, "On"),
            Cmd::BrightUp => write!(f, "BrightUp"),
            Cmd::Switch => write!(f, "Switch"),
            Cmd::BrightBack => write!(f, "BrightBack"),
            Cmd::SetBrightness(br) => write!(f, "SetBrightness({})", br),
            Cmd::LoadPreset => write!(f, "LoadPreset"),
            Cmd::SavePreset => write!(f, "SavePreset"),
            Cmd::Unbind => write!(f, "Unbind"),
            Cmd::StopBright => write!(f, "StopBright"),
            Cmd::BrightStepDown => write!(f, "BrightStepDown"),
            Cmd::BrightStepUp => write!(f, "BrightStepUp"),
            Cmd::BrightReg(reg) => write!(f, "BrightReg({})", reg),
            Cmd::Bind => write!(f, "Bind"),
            Cmd::RollColor => write!(f, "RollColor"),
            Cmd::SwitchColor => write!(f, "SwitchColor"),
            Cmd::SwitchMode => write!(f, "SwitchMode"),
            Cmd::SpeedMode => write!(f, "SpeedMode"),
            Cmd::BatteryLow => write!(f, "BatteryLow"),
            Cmd::SensTempHumi => write!(f, "SensTempHumi"),
            Cmd::TemporaryOn(tem) => write!(f, "TemporaryOn({})", tem),
            Cmd::Modes => write!(f, "Modes"),
            Cmd::ReadState => write!(f, "ReadState"),
            Cmd::WriteState => write!(f, "WriteState"),
            Cmd::SendState => write!(f, "SendState"),
            Cmd::Service(bl) => write!(f, "Service({})", if *bl { 1 } else { 0 }),
            Cmd::ClearMemory => write!(f, "ClearMemory"),
        }
    }
}

impl TryFrom<&[u8]> for Cmd {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Error> {
        Ok(match value[0] {
            0 => Cmd::Off,
            1 => Cmd::BrightDown,
            2 => Cmd::On,
            3 => Cmd::BrightUp,
            4 => Cmd::Switch,
            5 => Cmd::BrightBack,
            6 => Cmd::SetBrightness(SetBrightness::try_from(&value[1..])?),
            7 => Cmd::LoadPreset,
            8 => Cmd::SavePreset,
            9 => Cmd::Unbind,
            10 => Cmd::StopBright,
            11 => Cmd::BrightStepDown,
            12 => Cmd::BrightStepUp,
            13 => Cmd::BrightReg(value[2]),
            15 => Cmd::Bind,
            16 => Cmd::RollColor,
            17 => Cmd::SwitchColor,
            18 => Cmd::SwitchMode,
            19 => Cmd::SpeedMode,
            20 => Cmd::BatteryLow,
            21 => Cmd::SensTempHumi,
            25 => Cmd::TemporaryOn(TemporaryOn::try_from(&value[1..])?),
            26 => Cmd::Modes,
            128 => Cmd::ReadState,
            129 => Cmd::WriteState,
            130 => Cmd::SendState,
            131 => Cmd::Service(value[2] == 1),
            132 => Cmd::ClearMemory,
            _ => return Err(anyhow!("Failed to decode cmd: {:?}", value)),
        })
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Cmd::Off
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SetBrightness {
    Fmt1(u8),
    Fmt3([u8; 3]),
}

impl TryFrom<&[u8]> for SetBrightness {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Error> {
        Ok(match value[0] {
            1 => SetBrightness::Fmt1(value[1]),
            3 => {
                let mut buf = [0; 3];
                buf[0] = value[1];
                buf[1] = value[2];
                buf[2] = value[3];
                SetBrightness::Fmt3(buf)
            }
            _=> return Err(anyhow!("Failed to decode SetBrightness: {:?}", value))
        })
    }
}

impl fmt::Display for SetBrightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetBrightness::Fmt1(val) => {
                write!(f, "fmt=1 D={}", val)
            }
            SetBrightness::Fmt3(val) => {
                let mut buff = [0; 4];
                buff[0] = val[0];
                buff[1] = val[1];
                buff[2] = val[2];
                write!(f, "fmt=3 D={}", u32::from_le_bytes(buff))
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TemporaryOn {
    Fmt1(u8),
    Fmt2([u8; 2]),
}

impl TryFrom<&[u8]> for TemporaryOn {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(match value[0] {
            1 => TemporaryOn::Fmt1(value[1]),
            3 => {
                let mut buf = [0; 2];
                buf[0] = value[1];
                buf[1] = value[2];
                TemporaryOn::Fmt2(buf)
            }
            _=> return Err(anyhow!("Failed to decode TemporaryOn: {:?}", value))
        })
    }
}

impl fmt::Display for TemporaryOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TemporaryOn::Fmt1(val) => {
                write!(f, "fmt=1 D={}", val)
            }
            TemporaryOn::Fmt2(val) => {
                let mut buff = [0; 4];
                buff[0] = val[0];
                buff[1] = val[1];
                write!(f, "fmt=2 D={}", u32::from_le_bytes(buff))
            }
        }
    }
}

impl Cmd {
    pub fn as_u8(&self) -> u8 {
        match self {
            Cmd::Off => 0,
            Cmd::BrightDown => 1,
            Cmd::On => 2,
            Cmd::BrightUp => 3,
            Cmd::Switch => 4,
            Cmd::BrightBack => 5,
            Cmd::SetBrightness(_) => 6,
            Cmd::LoadPreset => 7,
            Cmd::SavePreset => 8,
            Cmd::Unbind => 9,
            Cmd::StopBright => 10,
            Cmd::BrightStepDown => 11,
            Cmd::BrightStepUp => 12,
            Cmd::BrightReg(_) => 13,
            Cmd::Bind => 15,
            Cmd::RollColor => 16,
            Cmd::SwitchColor => 17,
            Cmd::SwitchMode => 18,
            Cmd::SpeedMode => 19,
            Cmd::BatteryLow => 20,
            Cmd::SensTempHumi => 21,
            Cmd::TemporaryOn(_) => 25,
            Cmd::Modes => 26,
            Cmd::ReadState => 128,
            Cmd::WriteState => 129,
            Cmd::SendState => 130,
            Cmd::Service(_) => 131,
            Cmd::ClearMemory => 132,
        }
    }
}