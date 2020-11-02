
use std::fs;
//大智慧除权文件读取 split.pwr
pub struct Pwr {
    symbol:String,
}

impl Pwr{
    //读取大智慧除权文件
    pub fn new()->Self{
        return Pwr{symbol:"".to_string(),};
    }
    pub fn read_pwd(&mut self) {
        let pwrbuf = fs::read("split.pwr").unwrap();

        self.symbol= std::str::from_utf8(&pwrbuf[12..20]).unwrap().to_string();
    }
    pub fn get_symbol(&self)->&str{
        return &self.symbol;
    }
}
