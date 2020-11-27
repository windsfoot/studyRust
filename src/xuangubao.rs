/*!
### <https://flash-api.xuangubao.cn/api/market_indicator/line?fields=market_temperature>
### 选股宝网站的市场热度*/

use chrono::{Date, DateTime, Local, TimeZone};

use reqwest;
use std::collections::BTreeMap;
use serde_json;

const HTTP: &str =
    "https://flash-api.xuangubao.cn/api/market_indicator/line?fields=market_temperature";

pub struct XuanGuBao {
    temp: BTreeMap<Date<Local>, f32>,
}

impl XuanGuBao {
    pub fn new() -> Self {
        return XuanGuBao {
            temp: BTreeMap::new(),
        };
    }
    pub fn get(&self) {
        let body = reqwest::blocking::get(HTTP);
        match body {
            Ok(text) => {
                let t=text.text().unwrap();
                let t:serde_json::Value=serde_json::from_str(&t).unwrap();
                let p=&t["data"];
                //for i in p {
                    println!("{:?}",p);
                //}
                

            },
            Err(_) => error!("选股宝网站错误！"),
        }
    }
}
