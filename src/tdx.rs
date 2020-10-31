use std::fs;
use std::io::prelude::*;
use std::mem;

pub struct Tdx {
   pub iday:Vec<[u32;7]>,
}

impl Tdx {
    pub fn new() -> Self {
        return Tdx {
            iday:Vec::new(),        };
    }

    pub fn read_iday(&mut self) {
        let mut buffer = [0u8; 4];
        let tbuf = fs::read("sz000001.day");
        match tbuf {
            Ok(total) => {
                let i = total.len() / 4;
                let mut j = 0;
                let mut dbuf=[0u32;7];
                while j < i {
                    buffer = [
                        total[j * 4 + 0],
                        total[j * 4 + 1],
                        total[j * 4 + 2],
                        total[j * 4 + 3],
                    ];
                    if j%8<7{
                        dbuf[j%8]=u32::from_le_bytes(buffer);

                    }else{
                        if j%8==7{self.iday.push(dbuf);}
                        
                        dbuf=[0,0,0,0,0,0,0];
                    }
               //     println!("{:?}", self.iday);
                    j += 1;
                }
            }
            Err(er) => println!("er with {:?}", er),
        };

        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }
}
