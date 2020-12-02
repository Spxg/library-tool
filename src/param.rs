use serde::{
    Serialize,
    Deserialize,
};
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::{
    CLIENT,
    LOGIN_ADDRESS,
};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LoginParam {
    username: String,
    password: String,
    lt: String,
    dllt: String,
    execution: String,
    _eventId: String,
    rmShown: String,
}

impl LoginParam {
    pub fn new(username: &str, password: &str) -> Self {
        let lt_re = "name=\"lt\" value=\"(?P<value>.*?)\"";
        let execution_re = "name=\"execution\" value=\"(?P<value>.*?)\"";
        let (lt, execution) = get_part_param(lt_re, execution_re);

        Self {
            username: username.into(),
            password: password.into(),
            lt,
            dllt: "userNamePasswordLogin".into(),
            execution,
            _eventId: "submit".into(),
            rmShown: "1".into(),
        }
    }

    pub fn basic_param(&self) -> String {
        let mut param: HashMap<String, String> = HashMap::new();
        param.insert("username".into(), self.username.clone());
        param.insert("password".into(), self.password.clone());
        let param = serde_json::to_string(&param).unwrap();
        param
    }
}

pub fn find_param() -> PathBuf {
    let mut param_loc = std::env::current_exe().unwrap();
    param_loc.pop();
    param_loc.push("param");
    param_loc
}

fn get_part_param(lt_re: &str, execution_re: &str) -> (String, String) {
    let part_param = CLIENT
        .get(LOGIN_ADDRESS)
        .send().unwrap()
        .text().unwrap();

    let get_value = |re: &str| -> String {
        let re = Regex::new(re).unwrap();
        let cap = re.captures(&part_param).unwrap();
        let value = cap.name("value").unwrap().as_str();
        value.into()
    };

    (get_value(lt_re), get_value(execution_re))
}