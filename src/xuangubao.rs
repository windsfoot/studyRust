/*!
### <https://flash-api.xuangubao.cn/api/market_indicator/line?fields=market_temperature>
### 选股宝网站的市场热度*/

use chrono::{Date, DateTime, Local, TimeZone};

use reqwest;
use std::collections::BTreeMap;

const HTTP:&str="https://flash-api.xuangubao.cn/api/market_indicator/line?fields=market_temperature";


pub struct XuanGuBao {
    temp: BTreeMap<Date<Local>, f32>,
}

impl XuanGuBao {
    pub fn new() -> Self {
        return XuanGuBao {
            temp: BTreeMap::new(),
        };
    }
    pub fn get(&self){
        let body=reqwest::blocking::get(HTTP);
        match body{
            Ok(text)=>println!("{:?}",text.text().unwrap()),
            Err(_)=>error!("选股宝网站错误！"),

        }
        

    }
}
