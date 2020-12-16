mod opt;
mod param;
mod library;
mod account;
mod message;
mod qrcode_api;

use lazy_static::lazy_static;
use reqwest::blocking::Client;
use std::time::Duration;
use structopt::StructOpt;
use opt::Opt;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use crate::opt::Command;
use crate::opt::InOrOut;
use crate::message::Msg;
use crate::param::{
    LoginParam,
    find_param,
};

const LOGIN_ADDRESS: &'static str = "http://authserver.tjut.edu.cn/authserver/login";
const LOGOUT_ADDRESS: &'static str = "http://authserver.tjut.edu.cn/authserver/logout";
const IN_CODE_API1: &'static str = "http://ehall.tjut.edu.cn/rsfw/sys/tjlgdxtsgyy/setting/renderEwmCode.do";
const IN_CODE_API2: &'static str = "http://ehall.tjut.edu.cn/rsfw/sys/tjlgdxtsgyy/setting/renderEwmCode2.do";
const OUT_CODE_API: &'static str = "http://ehall.tjut.edu.cn/rsfw/sys/tjlgdxtsgyy/setting/renderEwmOutCode.do";

lazy_static! {
    static ref CLIENT: Client = {
        let builder = Client::builder();
        builder.cookie_store(true)
               .timeout(Duration::new(5, 0))
               .build().unwrap()
    };
}

fn main() {
    let opt = Opt::from_args();
    let param_loc = find_param();
    let mut status = false;

    match opt.cmd {
        Command::Login {
            username, password
        } => {
            let param = LoginParam::new(&username, &password);
            if account::login(&param) {
                status = true;
                println!("[success] login info saved");
            } else {
                println!("[failed] account login");
            }
        }
        Command::Library {
            op
        } => {
            if param_loc.exists() {
                let mut buf = [0; 512];
                let mut param = File::open(param_loc).unwrap();

                let len = param.read(&mut buf).unwrap();
                let param: HashMap<String, String> = serde_json::from_slice(&buf[0..len]).unwrap();
                let param = LoginParam::new(&param["username"], &param["password"]);
                if account::login(&param) {
                    status = true;
                    println!("[success] account login");
                } else {
                    println!("[failed] account logout");
                }
            } else {
                println!("[failed] not found login param, please login");
            }

            let do_it = || -> Msg {
                let msg = match op {
                    InOrOut::In { api } => {
                        if api == 1 {
                            library::in_by_api1()
                        } else if api == 2 {
                            library::in_by_api2()
                        } else {
                            Msg::unknown_api()
                        }
                    },
                    InOrOut::Out => library::out()
                };

                if msg.is_success() {
                    println!("[success] msg: {:?}", msg.msg_type());
                } else {
                    println!("[failed] msg: {:?}", msg.msg_type());
                }
                msg
            };

            while status && do_it().is_invalid_code() {}
        }
    }

    if status {
        account::logout();
        println!("[success] account logout");
    }
}
