use std::fs;
//大智慧除权文件读取 split.pwr
// 除权文件网址 http://filedown.gw.com.cn/download/FIN/full_sh.FIN
//
#[derive(Debug)]
pub struct Pwr {
    symbol: String,
    date: i32,
    szg: i32,
    pg: i32,
}

impl Pwr {
    //读取大智慧除权文件
    pub fn new() -> Self {
        return Pwr {
            symbol: "".to_string(),
            date: 0,
            szg: 0,
            pg: 0,
        };
    }
    pub fn read_pwr(&mut self) {
        let mut p = 12;
        let pwrbuf = fs::read("split.pwr").unwrap();

        self.symbol = std::str::from_utf8(&pwrbuf[p..p + 8]).unwrap().to_string();
        p += 16;
        self.date = i32::from_le_bytes([pwrbuf[p], pwrbuf[p + 1], pwrbuf[p + 2], pwrbuf[p + 3]]);
        p += 4;
        self.date = i32::from_le_bytes([pwrbuf[p], pwrbuf[p + 1], pwrbuf[p + 2], pwrbuf[p + 3]]);
        p += 4;
        self.pg = i32::from_le_bytes([pwrbuf[p], pwrbuf[p + 1], pwrbuf[p + 2], pwrbuf[p + 3]]);
    }
    pub fn get_symbol(&self) -> &Pwr {
        return &self;
    }
}
