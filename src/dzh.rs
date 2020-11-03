use reqwest;
use std::fs;
use encoding_rs::GBK;

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
pub struct Pwr {
    symbol: String,
    date: i32,
    szg: i32,
    pg: i32,
    //pwr: &str,
}

impl Pwr {
    //初始化
    pub fn new() -> Self {
        return Pwr {
            symbol: "".to_string(),
            date: 0,
            szg: 0,
            pg: 0,
          //  pwr: "",
        };
    }
    pub async fn get_pwr_web(self) {
        match reqwest::get(DZH_PWR_DRESS).await {
            Ok(resp) => match resp.bytes().await {
                Ok(text) => {
                    let (cow, encoding_used, had_errors) = GBK.decode(text.as_ref());
                    print!("{}",cow );
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
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
