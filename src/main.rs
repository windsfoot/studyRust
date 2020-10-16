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

   for _ in 1..5 {
      c.get_total_symbol().await;
      println!("{:?}\n{:?}", c.symbol, c.symbol.len());
      thread::sleep(time::Duration::from_secs(3));
   }
   //  let future=c.get_total_symbol().await;
   //block_on(future);
   println!("{:?}\n{:?}", c.symbol, c.symbol.len());
}
