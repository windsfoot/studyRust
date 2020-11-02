mod dzh;
mod mylog;
mod sina;
mod tdx;
use crate::sina::sina::Sina;
#[macro_use]
extern crate log;
extern crate env_logger;
use tokio::runtime::Runtime;
use std::time;

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
    t.read_pwd();
    println!("{}", t.get_symbol());

    println!("{:?}", time::SystemTime::now());
}
