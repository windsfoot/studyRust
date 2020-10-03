use futures;
use futures::future;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;

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
