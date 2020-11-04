mod dzh;
mod mylog;
mod sina;
mod tdx;
use crate::sina::sina::Sina;
#[macro_use]
extern crate log;
extern crate env_logger;
use futures::executor::block_on;
use std::time;
use tokio::runtime::Runtime;

fn main() {
    mylog::init_log();
    // let mut c: Sina = Sina::new();
    // let mut  r = Runtime::new().unwrap();
    // r.block_on(c.symbol_ready());
    // r.block_on(c.get2());
    // let mut t=tdx::Tdx::new();
    // t.read_all("D:\\new_dgzq_v6\\vipdoc\\sz\\lday");
    // println!("{:?}",t.iday);
    // let mut t=tdx::Tdx::new();
    // t.read_pwd();
    let mut t = dzh::Pwr::new();
    // println!("{:?}", time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_secs());
     //let mut  r = Runtime::new().unwrap();

   // let by = r.block_on(t.get_pwr_web("http://filedown.gw.com.cn/download/PWR/full_sz.PWR"));
    //match by {
      //  Ok(b) => println!("{:?}", b.as_ref()),
      //  Err(_) => {}
   // }
    // println!("{:?}",t);
    t.getpwr();
}
