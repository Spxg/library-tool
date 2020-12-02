use std::fs::OpenOptions;
use std::io::Write;
use crate::LOGIN_ADDRESS;
use crate::{
    CLIENT,
    LOGOUT_ADDRESS,
};
use crate::param::{
    LoginParam,
    find_param,
};

pub fn login(param: &LoginParam) -> bool {
    let resp = CLIENT.post(LOGIN_ADDRESS)
        .form(param)
        .send();

    let status = match resp {
        // its normal
        Err(e) if e.is_timeout() => true,
        Err(e) => panic!("{:?}", e),
        Ok(resp) => !resp.text().unwrap().contains("errMsg"),
    };
    if status { save_param(param); }
    status
}

pub fn logout() {
    CLIENT.get(LOGOUT_ADDRESS).send().unwrap();
}

pub fn save_param(param: &LoginParam) {
    let mut option = OpenOptions::new();
    let param_loc = find_param();
    let mut file = option
        .create(true)
        .write(true)
        .open(param_loc).unwrap();
    file.write(param.basic_param().as_bytes()).unwrap();
}