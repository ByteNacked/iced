#![feature(const_generics)]


use core::ops::Range;

pub enum MemoryError {
    OutOfSpace,
    OutOfRange,
    BadAlignment,
    CrcNotMatched,
}

pub trait Storage {
    type Error;
    //fn new(addr_range : Range<usize>) -> Self;
    fn write(&mut self, uid : usize, buf : &[u8]);
    fn read(&self, uid : usize, buf : &mut [u8]) -> usize;
}

struct DefaultStorage;

impl Storage for DefaultStorage {

    type Error = ();

    fn write(&mut self, uid : usize, buf : &[u8]) {
        println!("Write! uid: {}", uid);
    }
    fn read(&self, uid : usize, buf : &mut [u8]) -> usize {
        println!("Read! uid: {}", uid);
        0
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
