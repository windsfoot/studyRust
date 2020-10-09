mod sina;




use crate::futures::executor::block_on;
use crate::sina::sina::Sina;
use futures;
use futures::future;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;



#[tokio::main]
async fn main() {
    let mut c:Sina=Sina::new();
 
   let cc=c.set_dress(1, 10, c.mkt[0]);
   println!("{:?}",cc);
   let future=c.get_symbol();
   block_on(future);
   println!("{:?}",c.symbol);

    
}
