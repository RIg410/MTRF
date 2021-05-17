pub mod request;
pub mod response;

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
    FirmwareUpdate = 5
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
    SendCommandToId = 9
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
    SERVICE(bool),
    ClearMemory
}

#[derive(Debug, Copy, Clone)]
pub enum SetBrightness {
    Fmt1(u8),
    Fmt3([u8; 3]),
}

#[derive(Debug, Copy, Clone)]
pub enum TemporaryOn {
    Fmt1(u8),
    Fmt2([u8; 2]),
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
            Cmd::SERVICE(_) => 131,
            Cmd::ClearMemory => 132,
        }
    }
}