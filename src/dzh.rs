use bytes::Bytes;
use encoding_rs::{GB18030, GBK};
use reqwest;
use std::collections::BTreeMap;
use std::fs;
use std::str;

//大智慧除权文件读取 split.pwr
// 除权文件网址 http://filedown.gw.com.cn/download/FIN/full_sh.FIN
// 数据的最新更新信息？: http://222.73.103.181/platform/download/datainfo.cfg
// 除权文件：http://222.73.103.181/platform/download/PWR/full.PWR
// 财务文件：http://222.73.103.181/platform/download/FIN/full.FIN
// 板块文件: http://222.73.103.181/platform/download/ABK/full.ABK
// 板块文件: http://222.73.103.181/platform/download/ABK/inc.ABK
//
const DZH_PWR_DRESS: &str = "http://filedown.gw.com.cn/download/PWR/full.PWR";
#[derive(Debug)]
pub struct Pwr<'a> {
    pwrmap: BTreeMap<&'a str, [i32; 4]>,
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
    pub async fn get_pwr_web(&mut self) {
        match reqwest::get(DZH_PWR_DRESS).await {
            Ok(resp) => match resp.bytes().await {
                Ok(text) => {
                    self.pwrbuf = text;
                    //   let (cow, encoding_used, errors) = GB18030.decode(self.pwrbuf.as_ref());
                    //  println!("{:?}{:?}",encoding_used,errors);
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
    pub fn parse_pwr(&mut self) {
        let mut p = 8;

        let i = self.pwrbuf.len() - 8;
        if i % 120 == 0 {
            let j = i / 120;
            let mut k = 0;
            while k < j {
                let readbuf = self.pwrbuf.slice(p+k*120..p + 120*(k+1));
                let f=&readbuf.slice(0..4);
                if f==&Bytes::from(&b"\xff\xff\xff\xff"[..]){
                    let a=readbuf.slice(4..12);
                    println!("{:?}",a);
                }
               //let (cow, encoding_used, errors) = GB18030.decode(readbuf.as_ref());
               // println!("{:?}",f);
                k+=1;
            }
        } else {
            error!("pwr文件长度有误");
        }
    }
    pub fn get_symbol(&self) -> &Pwr {
        return &self;
    }
}
