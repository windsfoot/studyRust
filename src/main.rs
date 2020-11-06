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
     let mut c: Sina = Sina::new();
     let mut  r = Runtime::new().unwrap();
     r.block_on(c.symbol_ready());
     r.block_on(c.get_total_realq());

    /*读取通达信日线
     let mut t=tdx::Tdx::new();
     t.read_all("D:\\new_dgzq_v6\\vipdoc\\sz\\lday");
     println!("{:?}",t.iday);
  */

    //读取除权信息
    /*
    let mut t = dzh::Pwr::new();
    t.getpwr();
    for i in t.pwrmap {
      println!("###{:?}\n", i);
    }*/
}
