mod sina;

use crate::futures::executor::block_on;
use crate::sina::sina::Sina;
use futures;
use futures::future;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use std::thread;
use std::time;

#[tokio::main]
async fn main() {
    let mut c: Sina = Sina::new();

    c.get_total_symbol().await;
    println!("{:?}\n{:?}", c.symbol, c.symbol.len());
    thread::sleep(time::Duration::from_secs(3));
    let p=c.make_dress();
    c.get_real_q(&p[0]).await;
}
