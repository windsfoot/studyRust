use futures;
use reqwest;
//从新浪获取行情数据
pub mod sina {
    //新浪行情网址
    pub struct HttpAdress {
        pub dress: String,
    }

    //找到解答，解决字符串参数格式化的方法：
    //1.定义一个小函数
    //2.定义一个宏  eg：macro_rules! hello {() => ("hello")};println!(hello!());
    //
    impl HttpAdress {
        pub fn set_dress(&mut self, x: i32, y: i32, z: &str) {
            self.dress   =format!("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData?page={}&num={}&sort=symbol&asc=1&node={}&_s_r_a=init",x,y,z);
        }
        pub fn get(self) -> String {
            return self.dress;
        }
         pub async fn get_symbol(mut self) {
            self.set_dress(1, 10, "cyb");
            match reqwest::get(self.dress).await {
                Ok(resp) => match resp.text() {
                    Ok(text) => {
                        let v: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
                        for i in v {
                            //  let (vv:Symb,er) = serde_json::from_value(i);
                            if let Some(k) = i.get("symbol") {
                                println!("{}\n", k);
                            }
                        }
                    }
                    Err(_) => println!("ERROR reading {}", path),
                },
                Err(_) => println!("ERROR downloading {}", path),
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
