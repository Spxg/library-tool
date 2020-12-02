use crate::CLIENT;
use crate::qrcode_api::QRCodeAPI;
use crate::message::Msg;

pub fn in_by_api1() -> Msg {
    let api = QRCodeAPI::in_code1();
    sweep(&api.url())
}

pub fn in_by_api2() -> Msg {
    let api = QRCodeAPI::in_code2();
    sweep(&api.url())
}

pub fn out() -> Msg {
    let api = QRCodeAPI::out_code();
    sweep(&api.url())
}

pub fn sweep(url: &str) -> Msg {
    let resp = CLIENT.get(url)
        .send().unwrap();
    Msg::from_url(resp.url())
}