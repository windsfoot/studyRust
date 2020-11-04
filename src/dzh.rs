use bytes::{Buf, Bytes};
use chrono::{DateTime, Local, TimeZone};
use encoding_rs::{GB18030, GBK};
use reqwest;
use std::collections::BTreeMap;
use std::fs;
use std::str;
use std::time;

//大智慧除权文件读取 split.pwr
// 除权文件网址 http://filedown.gw.com.cn/download/FIN/full_sh.FIN
// 数据的最新更新信息？: http://222.73.103.181/platform/download/datainfo.cfg
// 除权文件：http://222.73.103.181/platform/download/PWR/full.PWR
// 财务文件：http://222.73.103.181/platform/download/FIN/full.FIN
// 板块文件: http://222.73.103.181/platform/download/ABK/full.ABK
// 板块文件: http://222.73.103.181/platform/download/ABK/inc.ABK
//
const DZH_PWR_DRESS: &str = "http://filedown.gw.com.cn/download/PWR/full_sz.PWR";
//full_of full_sh full_so fundfull.pwr hkfull hkfull_cvb hkfull_zb

#[derive(Debug)]
pub struct Pwr<'a> {
    pwrmap: BTreeMap<&'a str, [i64; 4]>,
    pwrbuf: Bytes,
}

impl Pwr<'_> {
    //初始化
    pub fn new() -> Self {
        return Pwr {
            pwrmap: BTreeMap::new(),
            pwrbuf: Bytes::new(),
        };
    }

    //获取大智慧pwr文件
    pub async fn get_pwr_web(&mut self) {
        match reqwest::get(DZH_PWR_DRESS).await {
            Ok(resp) => match resp.bytes().await {
                Ok(text) => {
                    self.pwrbuf = text;
                }
                Err(_) => {}
            },
            Err(er) => {
                error!("{}", er);
            }
        }
    }
    pub fn parse_pwr(&mut self) {
        let p = 8;
        let i = self.pwrbuf.len() - 8;
        if i % 120 == 0 {
            let j = i / 120;
            let mut k = 0;
            while k < j {
                let readbuf = self.pwrbuf.slice(p + k * 120..p + 120 * (k + 1));

                let f = &readbuf.slice(0..4);
                if f == &Bytes::from(&b"\xff\xff\xff\xff"[..]) {
                    let a = readbuf.slice(4..12);
                    println!("{:?}", a);
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
                    println!(
                        "日期{:?}送股{:?}配股{:?}配股价{:?}分红{:?}",
                        dt, sg, pg, pgj, fh
                    );
                }
                //let (cow, encoding_used, errors) = GB18030.decode(readbuf.as_ref());
                //println!("{:?}",f);
                k += 1;
            }
        } else {
            error!("pwr文件长度有误");
        }
    }
    pub fn get_symbol(&self) -> &Pwr {
        return &self;
    }
}
