/*!
    通达信日线数据解析
    pub total_iday: BTreeMap<代码：String, BTreeMap<日期：chrono::Date<Local>, [u32; 6]>>,
    ("sz000001.day", {2015-09-02+08:00: [1118, 1200, 1106, 1184, 1329799749, 281574592], 
        ...
    2020-10-27+08:00: [1800, 1800, 1750, 1776, 1322991936, 103486504], 2020-10-28+08:00: 
    [1776, 1790, 1729, 1763, 1325229133, 120582386]})

*/

use chrono::{Date, DateTime, Local, TimeZone};
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::mem;
use std::path::PathBuf;
use std::{fs, io};
pub struct Tdx {
    
    pub total_iday: BTreeMap<String, BTreeMap<chrono::Date<Local>, [u32; 6]>>,
}

impl Tdx {
    pub fn new() -> Self {
        return Tdx {
          
            total_iday: BTreeMap::new(),
        };
    }
    //读取单个日线文件
     fn read_iday(&mut self, f_name: &str)->Result< BTreeMap<chrono::Date<Local>, [u32; 6]>, &'static str> {
        let mut buffer = [0u8; 4]; //一个字段4个字节
        let tbuf = fs::read(f_name); //读取文件
        let mut iday: BTreeMap<chrono::Date<Local>, [u32; 6]>=BTreeMap::new();
        match tbuf {
            Ok(total) => {
                let i = total.len() / 4; //长度/4为总的字段数
                let mut j = 0;
                let mut dbuf = [0u32; 6];
                let mut date = chrono::offset::Local::today();
                while j < i {
                    buffer = [
                        total[j * 4 + 0],
                        total[j * 4 + 1],
                        total[j * 4 + 2],
                        total[j * 4 + 3],
                    ]; //读四个字节
                    match j % 8 {
                        0 => {
                            let d = u32::from_le_bytes(buffer);
                            date = Local.ymd(d as i32 / 10000, d % 10000 / 100, d % 100);
                        }
                        1..=6 => {
                            dbuf[j % 8 - 1] = u32::from_le_bytes(buffer);
                        }
                        7 => {
                            iday.insert(date, dbuf);
                            dbuf = [0, 0, 0, 0, 0, 0];
                        }
                        _ => error!("something error"),
                    }

                    //     println!("{:?}", self.iday);
                    j += 1;
                }
                return Ok(iday);
            }
            Err(er) => {
                error!("read iday file error{}",er);
                return Err("read iday file.error");}
        };
    }
    //读取目录下所有文件
    pub fn read_all(&mut self, path: &str) {
        let paths = fs::read_dir(path).unwrap();
        for p in paths {
            if let Some(e) = p.unwrap().path().to_str() {
                //   println!("{:?}",e);
                match self.read_iday(e){
                    Ok(iday)=>{
                        let pre=path.to_string()+"\\";
                        let ee=e.strip_prefix( &pre).unwrap();
                        self.total_iday.insert(ee.to_string(), iday);
                    }
                    Err(er)=>println!("err with {}",er),
                }
            }
        }
    }
}
