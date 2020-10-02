use futures;
use futures::future;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
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

#[tokio::main]
async fn main() {
    let aa = format!("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData?page={}&num=80&sort=symbol&asc=1&node=cyb&_s_r_a=init", 1);
    let paths = vec![aa];
    let fetches = futures::stream::iter(paths.into_iter().map(|path| async move {
        match reqwest::get(&path).await {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    //println!("{:?}",text);
                    let v: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
                    //println!("{:?}",v);
                    for i in v {
                        //  let (vv:Symb,er) = serde_json::from_value(i);
                        if let Some(k) = i.get("symbol") {
                            println!("{}\n", k);
                        };
                    }

                    //let v1:Vec::<serde_json::Value>=v;
                }

                Err(_) => println!("ERROR reading {}", path),
            },
            Err(_) => println!("ERROR downloading {}", path),
        }
    }))
    .buffer_unordered(8)
    .collect::<Vec<()>>();
    println!("Waiting...");
    fetches.await;
}
