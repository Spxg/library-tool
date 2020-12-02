use serde::{
    Serialize,
    Deserialize,
};
use crate::{
    CLIENT,
    IN_CODE_API1,
    IN_CODE_API2,
    OUT_CODE_API,
};

#[derive(Serialize, Deserialize)]
pub struct QRCodeAPI {
    success: bool,
    url: String,
}

impl QRCodeAPI {
    pub fn in_code1() -> Self {
        CLIENT.get(IN_CODE_API1)
            .send().unwrap()
            .json().unwrap()
    }

    pub fn in_code2() -> Self {
        CLIENT.get(IN_CODE_API2)
            .send().unwrap()
            .json().unwrap()
    }

    pub fn out_code() -> Self {
        CLIENT.get(OUT_CODE_API)
            .send().unwrap()
            .json().unwrap()
    }

    #[allow(unused)]
    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}