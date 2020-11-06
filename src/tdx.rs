/*!
    通达信日线数据解析  
*/

use std::io::prelude::*;
use std::mem;
use std::path::PathBuf;
use std::{fs, io};
pub struct Tdx {
    pub iday: Vec<[u32; 7]>,
}

impl Tdx {
    pub fn new() -> Self {
        return Tdx { iday: Vec::new() };
    }
//读取单个日线文件
     pub fn read_iday(&mut self, f_name: &str) {
        let mut buffer = [0u8; 4];
        let tbuf = fs::read(f_name);
        match tbuf {
            Ok(total) => {
                let i = total.len() / 4;
                let mut j = 0;
                let mut dbuf = [0u32; 7];
                while j < i {
                    buffer = [
                        total[j * 4 + 0],
                        total[j * 4 + 1],
                        total[j * 4 + 2],
                        total[j * 4 + 3],
                    ];
                    if j % 8 < 7 {
                        dbuf[j % 8] = u32::from_le_bytes(buffer);
                    } else {
                        if j % 8 == 7 {
                            self.iday.push(dbuf);
                        }
                        dbuf = [0, 0, 0, 0, 0, 0, 0];
                    }
                    //     println!("{:?}", self.iday);
                    j += 1;
                }
            }
            Err(er) => println!("er with {:?}", er),
        };
    }
//读取目录下所有文件
    pub fn read_all(&mut self, path: &str) {
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            if let Some(e) = path.unwrap().path().to_str(){
         //   println!("{:?}",e);
            self.read_iday(e);  
            }           
        }
    }
}
