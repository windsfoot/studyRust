mod sina;

use crate::sina::sina::Sina;
use futures;
use futures::future;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use std::thread;
use std::time;
use tokio::runtime::Runtime;

 fn main() {
    let mut c: Sina = Sina::new();
    let mut  r = Runtime::new().unwrap();
   r.block_on(c.get_total_symbol());
   println!("start get real quo now！");
    //c.get_total_real_q().await;
    r.block_on(c.get_total_real_q());

}
