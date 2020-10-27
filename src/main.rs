mod sina;
mod mylog;
use crate::sina::sina::Sina;
#[macro_use]
extern crate log;
extern crate env_logger;
use tokio::runtime::Runtime;

 fn main() {
    mylog::init_log();
    let mut c: Sina = Sina::new();
    let mut  r = Runtime::new().unwrap();
    r.block_on(c.symbol_ready());
    r.block_on(c.get2());

}
