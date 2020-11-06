/*!
    从大智慧网站上下载pwr除权文件，解析至btree结构中  
    BTreeMap<String, Vec<(DateTime<Local>, f32, f32, f32, f32)>>  
            <代码,   数组<(日期，           送股,配股,配股价,分红)>>  
*/

use bytes::{Buf, Bytes};
use chrono::{DateTime, Local, TimeZone};
use encoding_rs::{GB18030, GBK};
use futures::executor::block_on;
use reqwest;
use std::collections::BTreeMap;
use std::fs;
use std::str;
use std::time;
use tokio::runtime::Runtime;

///大智慧除权文件读取 split.pwr
/// 除权文件网址 http://filedown.gw.com.cn/download/FIN/full_sh.FIN
/// 数据的最新更新信息？: http://222.73.103.181/platform/download/datainfo.cfg
/// 除权文件：http://222.73.103.181/platform/download/PWR/full.PWR
/// 财务文件：http://222.73.103.181/platform/download/FIN/full.FIN
/// 板块文件: http://222.73.103.181/platform/download/ABK/full.ABK
/// 板块文件: http://222.73.103.181/platform/download/ABK/inc.ABK
//"http://filedown.gw.com.cn/download/PWR/fundfull.PWR",基金格式不同，以后再解析
//full_of full_sh full_so fundfull.pwr hkfull hkfull_cvb hkfull_zb
///
const DZH_PWR_DRESS: [&str; 2] = [
    "http://filedown.gw.com.cn/download/PWR/full_sz.PWR",
    "http://filedown.gw.com.cn/download/PWR/full_sh.PWR",
    // "http://filedown.gw.com.cn/download/PWR/full_of.PWR",
    // "http://filedown.gw.com.cn/download/PWR/full_so.PWR",
];

#[derive(Debug)]
pub struct Pwr {
    ///用来存储除权数据格式
    pub pwrmap: BTreeMap<String, Vec<(DateTime<Local>, f32, f32, f32, f32)>>,
}

impl Pwr {
    //初始化
    pub fn new() -> Self {
        return Pwr {
            pwrmap: BTreeMap::new(),
        };
    }

    ///获取大智慧pwr文件
    pub async fn get_pwr_web(&mut self, dress: &str) -> Result<Bytes, &'static str> {
        match reqwest::get(dress).await {
            Ok(resp) => match resp.bytes().await {
                Ok(text) => {
                    return Ok(text);
                }
                Err(_) => {
                    return Err("read response error.");
                }
            },
            Err(_) => {
                return Err("get pwr file error.");
            }
        }
    }

    ///解析pwr文件，push至pwrmap：btree
    pub fn parse_pwr(&mut self, pwrbuf: Bytes) {
        let p = 8;
        let i = pwrbuf.len() - p;
        let mut symbol = String::new();
        let mut symdata: Vec<(DateTime<Local>, f32, f32, f32, f32)> = Vec::new();
        if i % 120 == 0 {
            let j = i / 120;
            let mut k = 0;
            while k < j {
                let readbuf = pwrbuf.slice(p + k * 120..p + 120 * (k + 1));
                let f = &readbuf.slice(0..4);
                if f == &Bytes::from(&b"\xff\xff\xff\xff"[..]) {
                    if symdata.len() > 0 {
                        self.pwrmap.insert(symbol.clone(), symdata.clone());
                        symdata.clear();
                    }
                    let tmp = readbuf.slice(4..12);
                    let a = String::from_utf8(tmp.as_ref().to_vec());
                    match a {
                        Ok(sy) => symbol = sy,
                        Err(er) => error!("pwr文件中股票代码读取错误{}", er),
                    }
                } else {
                    let ubuf = [
                        f.as_ref()[0],
                        f.as_ref()[1],
                        f.as_ref()[2],
                        f.as_ref()[3],
                        0,
                        0,
                        0,
                        0,
                    ];
                    let dt: DateTime<Local> = Local.timestamp(i64::from_le_bytes(ubuf), 0);
                    let sg = readbuf.slice(4..8).get_f32_le();
                    let pg = readbuf.slice(8..12).get_f32_le();
                    let pgj = readbuf.slice(12..16).get_f32_le();
                    let fh = readbuf.slice(16..20).get_f32_le();
                    symdata.push((dt, sg, pg, pgj, fh));
                }
                k += 1;
                if k != j {
                    continue;
                }
                self.pwrmap.insert(symbol.clone(), symdata.clone());
            }
        } else {
            error!("pwr文件长度有误");
        }
    }
    pub fn getpwr(&mut self) {
        let mut r = Runtime::new().unwrap();
        for i in DZH_PWR_DRESS.iter() {
            match r.block_on(self.get_pwr_web(i)) {
                Ok(by) => self.parse_pwr(by),
                Err(er) => error!("er {}", er),
            }
        }
    }

    pub fn get_symbol(&self) -> &Pwr {
        return &self;
    }
}
