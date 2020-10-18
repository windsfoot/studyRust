mod sina;

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
    c.get_total_real_q().await;
    while true{}
}
