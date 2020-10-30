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
        let tbuf = fs::read("sz000001.day");
        match tbuf {
            Ok(total) => {
                let i = total.len() / 4;
                let mut j = 0;
                while j < i {
                    buffer = [
                        total[j * 4 + 0],
                        total[j * 4 + 1],
                        total[j * 4 + 2],
                        total[j * 4 + 3],
                    ];
                    println!("{:?}", u32::from_le_bytes(buffer));
                    j+=1;
                }

            }
            Err(er) => println!("er with {:?}", er),
        };
    }
}
