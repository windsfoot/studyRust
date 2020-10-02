//{"symbol":"sz300001","code":"300001","name":"\u7279\u9510\u5fb7","trade":"17.650","pricechange":-0.15,"changepercent":-0.843,"buy":"17.650","sell":"17.660","settlement":"17.800","open":"17.840","high":"17.960","low":"17.500","volume":7221217,"amount":127975579,"ticktime":"16:30:00","per":65.37,"pb":4.034,"mktcap":1760711.182375,"nmc":1667436.901605,"turnoverratio":0.76437}
use serde_json;
pub mod si {
    #[derive(Serialize, Deserialize)]
    pub struct Symb {
        symbol: String,
        code: String,
        name: String,
        trade: f32,
        pricechange: f32,
        changepercent: f32,
        buy: f32,
        sell: f32,
        settlement: f32,
        open: f32,
        high: f32,
        low: f32,
        volume: f64,
        amount: f64,
        ticktime: String,
        per: f32,
        pb: f32,
        mktcap: f64,
        nmc: f64,
        turnoverratio: f64,
    }
}
