use reqwest::Url;
use regex::Regex;
use std::str::FromStr;

pub struct Msg {
    success: bool,
    msg_type: MsgType,
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum MsgType {
    InvalidQRCode = 1,
    InvalidAccount = 2,
    OutDone = 3,
    InvalidMember = 4,
    MaxNumber = 5,
    Late = 6,
    OverTimeIn = 7,
    InDown = 8,
    NotOder = 9,
    TooFast = 10,
    NotInButOut = 11,
    OverTimeOut = 12,
    UnknownApi = 13,
    Other = 14,
}

impl Msg {
    pub fn from_url(url: &Url) -> Self {
        let mut msg: String = url.fragment().unwrap().into();
        msg.push('&');

        let rule = "type=(?P<success>.*?)&msgType=(?P<msg_type>.*?)&";
        let re = Regex::new(rule).unwrap();
        let cap = re.captures(&msg).unwrap();
        let success = bool::from_str(cap.name("success").unwrap().as_str()).unwrap();
        let msg_type = u8::from_str(cap.name("msg_type").unwrap().as_str()).unwrap();

        Self {
            success,
            msg_type: msg_type.into(),
        }
    }

    pub fn msg_type(&self) -> MsgType {
        self.msg_type
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn is_invalid_code(&self) -> bool {
        MsgType::InvalidQRCode == self.msg_type
    }

    pub fn unknown_api() -> Self {
        Self {
            success: false,
            msg_type: MsgType::UnknownApi
        }
    }
}

impl From<u8> for MsgType {
    fn from(orig: u8) -> Self {
        match orig {
            1 => MsgType::InvalidQRCode,
            2 => MsgType::InvalidAccount,
            3 => MsgType::OutDone,
            4 => MsgType::InvalidMember,
            5 => MsgType::MaxNumber,
            6 => MsgType::Late,
            7 => MsgType::OverTimeIn,
            8 => MsgType::InDown,
            9 => MsgType::NotOder,
            10 => MsgType::TooFast,
            11 => MsgType::NotInButOut,
            12 => MsgType::OverTimeOut,
            _ => MsgType::Other
        }
    }
}