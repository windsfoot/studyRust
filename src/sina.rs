/*!
    从新浪获取行情数据
    目前只会全市场循环刷，异步机制学完后可将各线程分开独立循环刷
*/
pub mod sina {
    use chrono;
    //  use futures;
    use futures::executor::block_on;
    use futures::future; 
    use futures::stream::{self, StreamExt};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::time;

    //常量
    const MKT: [&str; 1] = ["hs_a"]; //["hs_a","cyb","kcb"];//定义市场名称
    const SYM_VOL: u32 = 100; //全A单页股票个数
    const MAX_A: i32 = 42; //全A页数
    const R_QUA: &str = "http://hq.sinajs.cn/list=";
    //const R_QUA1:&str="http://hq.sinajs.cn/rn=3qw0v&format=text&list=stock_sh_up_5min_20"; 5分钟涨速榜
    const MAX_QUA: usize = 70; //实时行情单次最大股票数量
    const QUA_DELAY: u64 = 3000; //抓取延时，毫秒
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

        ///抓取代码的链接控制
        ///fn set_dress(&mut self, x: i32, y: u32, z: &str) -> String   x:页数 y：每页数量 z：市场名
        fn set_dress(&mut self, x: i32, y: u32, z: &str) -> String {
            return format!("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData?page={}&num={}&sort=symbol&asc=1&node={}&_s_r_a=init",x,y,z);
        }

        ///抓取代码表
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

        ///获取全A代码
        async fn get_total_symbol_web(&mut self) {
            info!("从网络取回股票列表。");
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
            let mut s: String = String::new();
            for i in &self.symbol {
                s = s + &i;
                s = s + " ";
            }
            fs::write("symbol", s).unwrap();
        }

        ///代码表当日网络比较抓取后落地。
        pub async fn symbol_ready(&mut self) {
            let sy = fs::read_to_string("symbol");
            match sy {
                Ok(sym) => {
                    let me = fs::metadata("symbol").unwrap();
                    let mo = me.modified();
                    if let Ok(moo) = mo {
                        let file_time = moo
                            .duration_since(time::SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let lc = chrono::Local::today().and_hms(0, 0, 1).timestamp() as u64;
                        if file_time > lc {
                            let k: Vec<&str> = sym.split(" ").collect(); //);
                            self.symbol = k.into_iter().map(|x| x.to_string()).collect();
                            self.symbol.pop();
                            info!("当日列表文件symbol已更新，从本地读取列表。");
                        } else {
                            block_on(self.get_total_symbol_web());
                        }
                    }
                }
                Err(_) => {
                    block_on(self.get_total_symbol_web());
                }
            }
        }

        //分配抓取序列
        fn make_dress(&self) -> Vec<String> {
            info!("生成实时行情抓取网址。");
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

        ///解析抓回的数据
        ///to_symb(&self, text: String)
        fn to_symb(&self, text: String) {
            let v_text: Vec<&str> = text.split("\";\n").collect();
            for i in v_text {
                if let Some(k) = i.strip_prefix("var hq_str_") {
                    println!("{:?}", k);
                }
            }
        }

        ///抓取实时行情
        pub async fn get_total_realq(&self) {
            let urls = self.make_dress();
            loop {
                info!("get once");
                let client = reqwest::Client::new();
                let bodies = future::join_all(urls.clone().into_iter().map(|url| {
                    let client = &client;
                    async move {
                        let resp = client.get(&url).send().await?;
                        resp.text().await
                    }
                }))
                .await;
                for b in bodies {
                    match b {
                        Ok(b) => self.to_symb(b),
                        Err(e) => error!("Got an error: {}", e),
                    }
                }
                std::thread::sleep(time::Duration::from_millis(QUA_DELAY));
            }
        }
    }
}
