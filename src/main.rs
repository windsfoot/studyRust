use crate::si::si::Symb;
use futures;
use futures::future;
use futures::stream::{self, StreamExt};
use serde_json;
mod si;


#[tokio::main]
async fn main() {
    let paths = vec![

        "http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData?page=1&num=40&sort=symbol&asc=1&node=cyb&_s_r_a=init".to_string(),
    ];
    let fetches = futures::stream::iter(paths.into_iter().map(|path| async move {
        match reqwest::get(&path).await {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    //println!("{:?}",text);
                    let v: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
                    //println!("{:?}",v);
                    for i in v {
                       let vv: serde_json::Value     = serde_json::from_value(i).unwrap();
                        println!("{:?}\n", vv);
                        break;
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
