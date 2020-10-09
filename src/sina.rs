use futures;
use reqwest;
//从新浪获取行情数据
pub mod sina {
    //新浪行情网址
    pub struct Sina<'a> {
        pub symbol: Vec<String>,
        pub mkt: [&'a str; 4],
    }

    //找到解答，解决字符串参数格式化的方法：
    //1.定义一个小函数
    //2.定义一个宏  eg：macro_rules! hello {() => ("hello")};println!(hello!());
    //
    impl Sina<'_> {
        pub fn new() -> Self {
            return Sina {
                mkt: ["sh", "sz", "cyb", "kcb"],
                symbol: vec![],
            };
        }
        //抓取代码的链接控制
        pub fn set_dress(&mut self, x: i32, y: i32, z: &str) -> String {
            return format!("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData?page={}&num={}&sort=symbol&asc=1&node={}&_s_r_a=init",x,y,z);
        }
        //抓取代码表
        pub async fn get_symbol(&mut self) {
            let s: String = self.set_dress(100, 60, "cyb");
            match reqwest::get(&s).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        let v: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
                        self.symbol.push(String::from("Start"));
                        for i in v {
                            if let Some(k) = i.get("symbol") {
                                if let serde_json::Value::String(j) = k {
                                    self.symbol.push(j.to_string());
                                }
                            }
                        }
                    }
                    Err(_) => println!("ERROR reading {}", s),
                },
                Err(_) => println!("ERROR downloading {}", s),
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
