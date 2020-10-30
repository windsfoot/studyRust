use std::fs;
use std::io::prelude::*;
use std::mem;

pub struct Tdx {}

impl Tdx {
    pub fn new() -> Self {
        return Tdx {};
    }
    pub fn read_iday(self) {
        let mut buffer = [0u8; 4];
        let mut file = fs::File::open("sz000001.day").unwrap();
        for _ in 0..5{
        file.read(&mut buffer).unwrap();
        unsafe {
            let c = mem::transmute::<[u8; 4], u32>(buffer);
            println!("{:?}", c);
        }
    }
}
}