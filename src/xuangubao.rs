/*!
### <https://flash-api.xuangubao.cn/api/market_indicator/line?fields=market_temperature>
### 选股宝网站的市场热度*/

use chrono::{Date, DateTime, Local, TimeZone};

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::collections::BTreeMap;

const HTTP: &str =
    "https://flash-api.xuangubao.cn/api/market_indicator/line?fields=market_temperature";
const TemPath: &str = "xgbtem.dt";

#[derive(Debug, Serialize, Deserialize)]
struct Sdata {
    market_temperature: f64,
    timestamp: i64,
}
#[derive(Debug, Serialize, Deserialize)]
struct XGBTemp {
    code: u64,
    message: String,
    data: Vec<Sdata>,
}
pub struct XuanGuBao {
    temp: BTreeMap<DateTime<Local>, f64>,
    temperature: Vec<Sdata>,
}

impl XuanGuBao {
    pub fn new() -> Self {
        return XuanGuBao {
            temp: BTreeMap::new(),
            temperature: vec![],
        };
    }
    pub fn GetFromWeb(&mut self) {
        let body = reqwest::blocking::get(HTTP);
        match body {
            Ok(text) => {
                let t = text.text().unwrap();
                let t: XGBTemp = serde_json::from_str(&t).unwrap();
                self.temperature = t.data;
            }
            Err(_) => error!("选股宝网站错误！"),
        }
        //println!("{:?}",self.temperature);
    }
    pub fn ToMap(&mut self) {
        for i in &self.temperature {
            let d = i.timestamp;
            let dat = Local.timestamp(d, 0);
            self.temp.insert(dat, i.market_temperature);
        }
        for i in &self.temp{
            println!("{:?}", i);
        }
      
    }
    pub fn ToSled(& mut self) {
        match sled::open(TemPath) {
            Ok(db) => {
                for i in &self.temperature {
                   let k=i.market_temperature.to_le_bytes();
                    db.insert(i.timestamp.to_le_bytes(), &k);
                }
                self.temp.clear();
                for i in db.iter(){
                   
                    match i{
                        Ok(j)=>{
                            
                            let k=j.0.as_ref();
                            let v=j.1.as_ref();
                          
                            let tim= i64::from_le_bytes([k[0],k[1],k[2],k[3],k[4],k[5],k[6],k[7]]);
                            let tem= f64::from_le_bytes([v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7]]);
                            let dt=Local.timestamp(tim,0);
                            self.temp.insert(dt, tem);
                        },
                        Err(_)=>{},
                    }
                }
            }
            Err(_) => error!("open TemPath error!"),
        }
    }
}
