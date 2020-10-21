//从新浪获取行情数据
pub mod sina {
    use futures;
    use std::time;

    //常量
    const MKT: [&str; 1] = ["hs_a"]; //["hs_a","cyb","kcb"];//定义市场名称
    const SYM_VOL: u32 = 100; //全A单页股票个数
    const MAX_A: i32 = 42; //全A页数
    const R_QUA: &str = "http://hq.sinajs.cn/list=";
    //const R_QUA1:&str="http://hq.sinajs.cn/rn=3qw0v&format=text&list=stock_sh_up_5min_20"; 5分钟涨速榜
    const MAX_QUA: usize = 70; //实时行情单次最大股票数量

    //新浪行情结构
    pub struct Sina {
        pub symbol: Vec<String>,
    }
    //找到解答，解决字符串参数格式化的方法：
    //1.定义一个小函数
    //2.定义一个宏  eg：macro_rules! hello {() => ("hello")};println!(hello!());
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

        //抓取实时行情
        pub async fn get_real_q(&self, r_dress: &String) {
            match reqwest::get(r_dress).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => self.to_symb(text),
                    Err(_) => println!("ERROR reading {}", r_dress),
                },
                Err(_) => println!("ERROR downloading {}", r_dress),
            }
        }
        //分配抓取序列
        pub fn make_dress(&self) -> Vec<String> {
            let i: usize = self.symbol.len();
            let mut r_dress: Vec<String> = Vec::new();
            let mut j: usize = 0; //总计数
            let mut k: usize = 0; //列计数
            while j < i {
                let mut tm: String = R_QUA.to_string();
                for _ in 0..MAX_QUA {
                    tm = tm + &self.symbol[j];
                    j = j + 1;
                    if j == i {
                        break;
                    }
                    tm = tm + ",";
                }
                r_dress.push(tm);
                k = k + 1;
            }
            return r_dress;
        }
        pub async fn get_total_real_q(&self) {
            let p = self.make_dress();
           // loop {
                for i in &p {
                    futures::join!(self.get_real_q(&i));
                }
                std::thread::sleep(time::Duration::from_secs(1));
        //  }
        }
        pub fn to_symb(&self, text: String) {
            let v_text:Vec<&str> = text.split("\";\n").collect();
            for i in v_text {
                println!("{:?}\n", i.strip_prefix("var hq_str_"));
            }
        }
    }

    //定义新浪行情数据
    // // ///struct Symb {
    // //     symbol: String,
    // //     code: String,
    // //     name: String,
    // //     trade: f32,
    // //     pricechange: f32,
    // //     changepercent: f32,
    // //     buy: f32,
    // /    sell: f32,
    //     settlement: f32,
    //     open: f32,
    //     high: f32,
    //     low: f32,
    //     volume: f64,
    //     amount: f64,
    //     ticktime: String,
    //     per: f32,
    //     pb: f32,
    //     mktcap: f64,
    //     nmc: f64,
    //     turnoverratio: f64,
    // }
}
