mod sina;

use crate::sina::sina::Sina;



use tokio::runtime::Runtime;

 fn main() {
    let mut c: Sina = Sina::new();
    let mut  r = Runtime::new().unwrap();
   r.block_on(c.get_total_symbol());
    r.block_on(c.get_total_real_q());

}
