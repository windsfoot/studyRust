use futures;
use reqwest;

//从新浪获取行情数据
pub mod sina {
    //常量
    const MKT: [&str; 1] = ["hs_a"]; //["hs_a","cyb","kcb"];//定义市场名称
    const SYM_VOL: u32 = 100; //全A单页股票个数
    const MAX_A: i32 = 42; //全A页数
    const R_QUA: &str = "http://hq.sinajs.cn/list=";
    //const R_QUA1:&str="http://hq.sinajs.cn/rn=3qw0v&format=text&list=stock_sh_up_5min_20"; 5分钟涨速榜

    //新浪行情结构
    pub struct Sina {
        pub symbol: Vec<String>,
    }

    //找到解答，解决字符串参数格式化的方法：
    //1.定义一个小函数
    //2.定义一个宏  eg：macro_rules! hello {() => ("hello")};println!(hello!());
    //
    impl Sina {
        pub fn new() -> Self {
            return Sina { symbol: vec![] };
        }

        //抓取代码的链接控制
        pub fn set_dress(&mut self, x: i32, y: u32, z: &str) -> String {
            return format!("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData?page={}&num={}&sort=symbol&asc=1&node={}&_s_r_a=init",x,y,z);
        }
        //抓取代码表
        async fn get_symbol(&mut self, dress: String) {
            match reqwest::get(&dress).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        let v: Vec<serde_json::Value> = serde_json::from_str(&text).expect(&text);
                        for i in v {
                            if let Some(k) = i.get("symbol") {
                                if let serde_json::Value::String(j) = k {
                                    self.symbol.push(j.to_string());
                                }
                            }
                        }
                    }
                    Err(_) => println!("ERROR reading {}", dress),
                },
                Err(_) => println!("ERROR downloading {}", dress),
            }
        }

        //获取全A代码
        pub async fn get_total_symbol(&mut self) {
            for i in &MKT {
                let mut j = 1;
                match i {
                    &"hs_a" => {
                        //处理A股代码
                        while j < MAX_A {
                            let s: String = self.set_dress(j, SYM_VOL, i);
                            futures::join!(self.get_symbol(s));
                            j = j + 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        //抓取事实行情
        pub async fn get_real_q(&mut self) {
            let r_dress = String::from(R_QUA) + &self.symbol[0];
            println!("{:?}", r_dress);
            match reqwest::get(&r_dress).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        if let Some(t1) = text.get(21..text.len()-4) {
                            println!("{:?}", t1);
                        }
                        // let v: Vec<serde_json::Value> = serde_json::from_str(&text).expect(&text);
                        // for i in v {
                        //     if let Some(k) = i.get("symbol") {
                        //         if let serde_json::Value::String(j) = k {
                        //
                        //             self.symbol.push(j.to_string());
                        //        }
                        //     }
                        // }
                    }
                    Err(_) => println!("ERROR reading {}", r_dress),
                },
                Err(_) => println!("ERROR downloading {}", r_dress),
            }
        }
    }

    //定义新浪行情数据
    struct Symb {
        symbol: String,
        code: String,
        name: String,
        trade: f32,
        pricechange: f32,
        changepercent: f32,
        buy: f32,
        sell: f32,
        settlement: f32,
        open: f32,
        high: f32,
        low: f32,
        volume: f64,
        amount: f64,
        ticktime: String,
        per: f32,
        pb: f32,
        mktcap: f64,
        nmc: f64,
        turnoverratio: f64,
    }
}
